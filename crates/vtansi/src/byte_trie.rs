//! Fast byte trie optimized for incremental ANSI escape sequence matching.
//!
//! This implementation is specifically designed for:
//! - Short byte sequences (typically 2-10 bytes)
//! - High prefix sharing (e.g., all CSI sequences start with ESC+[)
//! - Incremental byte-by-byte matching
//! - Static data known at compile time
//!
//! # Performance Characteristics
//!
//! - O(1) lookup per byte via direct array indexing
//! - 256 bytes per node (`[u16; 128]` array, 7-bit encoding only)
//! - Cache-friendly Vec-based node storage
//! - Zero-cost cursor cloning (just a usize)
//! - Maximum 65,535 nodes (plenty for any practical trie)
//!
//! # 7-bit Encoding
//!
//! VT/ANSI sequences use 7-bit encoding (bytes 0x00-0x7F), so we only need
//! 128 entries per node instead of 256. This halves memory usage while
//! maintaining O(1) lookup performance. Attempting to insert bytes >= 128
//! will panic at trie construction time.

/// Sentinel value indicating no child at this index.
const NO_CHILD: u16 = u16::MAX;

/// A node in the byte trie with O(1) direct-indexed lookup.
///
/// Uses a 128-element array for constant-time child lookup, exploiting the fact
/// that VT sequences use 7-bit encoding (bytes 0x00-0x7F only). Using `u16` indices
/// reduces memory to ~256 bytes per node while supporting up to 65,535 nodes.
/// The array is stored inline to avoid pointer indirection during lookup.
#[derive(Debug, Clone)]
struct TrieNode<V> {
    /// Value stored at this node if it represents a complete match.
    value: Option<V>,
    /// Direct-indexed children array for 7-bit bytes (0x00-0x7F).
    /// Index is the byte value, value is the child node index.
    /// `NO_CHILD` (`u16::MAX`) indicates no child.
    children: [u16; 128],
    /// Whether there are any actual childern in `children`.
    has_children: bool,
}

impl<V> TrieNode<V> {
    fn new() -> Self {
        Self {
            value: None,
            children: [NO_CHILD; 128],
            has_children: false,
        }
    }

    /// Gets the child index for a given byte. O(1) direct array access.
    ///
    /// For bytes >= 128, returns `None` (out of 7-bit range).
    #[inline]
    fn get_child(&self, byte: u8) -> Option<u16> {
        if byte >= 128 {
            None
        } else {
            match self.children[byte as usize] {
                NO_CHILD => None,
                idx => Some(idx),
            }
        }
    }

    /// Inserts a child, returning the old value if it existed.
    ///
    /// # Panics
    ///
    /// Panics if `byte >= 128` since VT sequences use 7-bit encoding.
    /// Panics if `child_idx >= 65535` (node limit exceeded).
    #[inline]
    fn insert_child(&mut self, byte: u8, child_idx: u16) -> Option<usize> {
        assert!(
            byte < 128,
            "VT sequences use 7-bit encoding; byte 0x{byte:02X} is out of range"
        );
        let slot = &mut self.children[byte as usize];
        if *slot == NO_CHILD {
            *slot = child_idx;
            self.has_children = true;
            None
        } else {
            let old = *slot as usize;
            *slot = child_idx;
            Some(old)
        }
    }

    /// Returns true if this node has any children.
    #[inline]
    fn has_children(&self) -> bool {
        self.has_children
    }
}

/// A byte trie for efficient prefix matching.
#[derive(Debug, Clone)]
pub struct ByteTrie<V> {
    /// All nodes in the trie. Index 0 is always the root.
    nodes: Vec<TrieNode<V>>,
}

impl<V> Default for ByteTrie<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> ByteTrie<V> {
    /// Creates a new empty trie.
    #[must_use]
    pub fn new() -> Self {
        Self {
            nodes: vec![TrieNode::new()],
        }
    }

    /// Inserts a key-value pair into the trie.
    ///
    /// # Panics
    ///
    /// Panics if any byte in `key` is >= 128 (VT sequences use 7-bit encoding).
    pub fn insert(&mut self, key: &[u8], value: V) {
        let mut node_idx = 0; // Start at root

        for &byte in key {
            // Get or create child node
            let next_idx =
                if let Some(child_idx) = self.nodes[node_idx].get_child(byte) {
                    child_idx
                } else {
                    // Create new node
                    let new_idx = self.nodes.len();
                    assert!(
                        new_idx < u16::MAX as usize,
                        "Trie node limit exceeded (max 65535 nodes)"
                    );
                    #[allow(clippy::cast_possible_truncation)]
                    // Checked by assert above
                    let new_idx = new_idx as u16;
                    self.nodes.push(TrieNode::new());
                    self.nodes[node_idx].insert_child(byte, new_idx);
                    new_idx
                };

            node_idx = next_idx as usize;
        }

        // Store value at final node
        self.nodes[node_idx].value = Some(value);
    }

    /// Creates a cursor at the root of the trie.
    #[inline]
    #[must_use]
    pub fn cursor(&self) -> ByteTrieCursor<'_, V> {
        ByteTrieCursor {
            trie: self,
            node_idx: Some(0u16),
        }
    }

    /// Returns the number of nodes in the trie.
    #[inline]
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the memory usage of the trie in bytes.
    ///
    /// This includes the Vec overhead and all node storage, but not the
    /// size of stored values (since V could contain pointers to external data).
    #[must_use]
    pub fn memory_usage_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            + self.nodes.capacity() * std::mem::size_of::<TrieNode<V>>()
    }

    /// Returns memory statistics for debugging/comparison.
    #[must_use]
    pub fn memory_stats(&self) -> TrieMemoryStats {
        TrieMemoryStats {
            node_count: self.nodes.len(),
            node_capacity: self.nodes.capacity(),
            bytes_per_node: std::mem::size_of::<TrieNode<V>>(),
            total_bytes: self.memory_usage_bytes(),
        }
    }
}

/// Memory statistics for a `ByteTrie`.
#[derive(Debug, Clone, Copy)]
pub struct TrieMemoryStats {
    /// Number of nodes in the trie.
    pub node_count: usize,
    /// Capacity of the node vector.
    pub node_capacity: usize,
    /// Size of each node in bytes.
    pub bytes_per_node: usize,
    /// Total memory usage in bytes.
    pub total_bytes: usize,
}

impl std::fmt::Display for TrieMemoryStats {
    #[allow(clippy::cast_precision_loss)] // Acceptable for display purposes
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} nodes × {} bytes/node = {} bytes ({:.2} KB)",
            self.node_count,
            self.bytes_per_node,
            self.total_bytes,
            self.total_bytes as f64 / 1024.0
        )
    }
}

/// Answer type for incremental trie queries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer<'a, V> {
    /// Current position is a prefix of one or more keys, but not a complete match.
    Prefix,
    /// Current position is a complete match, but not a prefix of other keys.
    Match(&'a V),
    /// Current position is both a complete match and a prefix of other keys.
    PrefixAndMatch(&'a V),
    /// Current position does not match any key or prefix.
    DeadEnd,
}

impl<V> Answer<'_, V> {
    /// Returns true if this is a valid prefix (more bytes can be added).
    #[inline]
    #[must_use]
    pub fn is_prefix(&self) -> bool {
        matches!(self, Self::Prefix | Self::PrefixAndMatch(_))
    }

    /// Returns true if this is a complete match.
    #[inline]
    #[must_use]
    pub fn is_match(&self) -> bool {
        matches!(self, Self::Match(_) | Self::PrefixAndMatch(_))
    }

    /// Returns true if this is a dead end (no matches possible).
    #[inline]
    #[must_use]
    pub fn is_dead_end(&self) -> bool {
        matches!(self, Self::DeadEnd)
    }
}

/// A cursor for incremental trie matching.
#[derive(Clone, Copy, Debug)]
pub struct ByteTrieCursor<'a, V> {
    trie: &'a ByteTrie<V>,
    node_idx: Option<u16>,
}

impl<'a, V> ByteTrieCursor<'a, V> {
    /// Peek one byte from the current position without advancing the cursor.
    #[must_use]
    #[inline]
    fn peek(&self, byte: u8) -> (Answer<'a, V>, Option<u16>) {
        match self.node_idx {
            None => (Answer::DeadEnd, None),
            Some(node_idx) => {
                let node = &self.trie.nodes[node_idx as usize];
                if let Some(child_idx) = node.get_child(byte) {
                    let answer = self.deref_at_unchecked(child_idx);
                    (answer, Some(child_idx))
                } else {
                    (Answer::DeadEnd, None)
                }
            }
        }
    }

    /// Advances the cursor by one byte and returns the result.
    #[inline]
    pub fn advance(&mut self, byte: u8) -> Answer<'a, V> {
        let (answer, child_idx) = self.peek(byte);
        self.node_idx = child_idx;
        answer
    }

    /// Advances the cursor by multiple bytes in sequence.
    ///
    /// Returns the final answer after processing all bytes, or `Answer::DeadEnd`
    /// if any byte fails to match.
    ///
    /// The cursor position is updated to reflect how far we advanced.
    #[inline]
    pub fn advance_slice(&mut self, bytes: &[u8]) -> Answer<'a, V> {
        let mut last_answer = Answer::Prefix;

        for &byte in bytes {
            last_answer = self.advance(byte);
            if last_answer.is_dead_end() {
                return Answer::DeadEnd;
            }
        }

        last_answer
    }

    /// Return the state at the current cursor position.
    #[inline]
    #[must_use]
    pub fn deref(&self) -> Answer<'a, V> {
        match self.node_idx {
            None => Answer::DeadEnd,
            Some(node_idx) => self.deref_at_unchecked(node_idx),
        }
    }

    /// Query the trie at specified child index.
    #[inline]
    #[must_use]
    fn deref_at_unchecked(&self, child_idx: u16) -> Answer<'a, V> {
        let child = &self.trie.nodes[child_idx as usize];

        match (&child.value, child.has_children()) {
            (Some(value), true) => Answer::PrefixAndMatch(value),
            (Some(value), false) => Answer::Match(value),
            (None, true) => Answer::Prefix,
            (None, false) => {
                unreachable!("leaf nodes should always have values")
            }
        }
    }
}

/// Builder for constructing a `ByteTrie`.
pub struct ByteTrieBuilder<V> {
    trie: ByteTrie<V>,
}

impl<V> Default for ByteTrieBuilder<V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<V> ByteTrieBuilder<V> {
    /// Creates a new builder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            trie: ByteTrie::new(),
        }
    }

    /// Inserts a key-value pair.
    ///
    /// # Panics
    ///
    /// Panics if any byte in `key` is >= 128 (VT sequences use 7-bit encoding).
    pub fn insert(&mut self, key: &[u8], value: V) -> &mut Self {
        self.trie.insert(key, value);
        self
    }

    /// Builds the final trie.
    #[must_use]
    pub fn build(self) -> ByteTrie<V> {
        self.trie
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_trie() {
        let trie: ByteTrie<u32> = ByteTrie::new();
        assert_eq!(trie.node_count(), 1); // Just root

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance(b'a'), Answer::DeadEnd);
    }

    #[test]
    fn test_single_key() {
        let mut trie = ByteTrie::new();
        trie.insert(b"hello", 42);

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"hello"), Answer::Match(&42));

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"hell"), Answer::Prefix);
        assert_eq!(cursor.deref(), Answer::Prefix);
    }

    #[test]
    fn test_prefix_matching() {
        let mut trie = ByteTrie::new();
        trie.insert(b"cat", 1);
        trie.insert(b"cats", 2);
        trie.insert(b"dog", 3);

        let mut cursor = trie.cursor();

        assert_eq!(cursor.advance(b'c'), Answer::Prefix);
        assert_eq!(cursor.advance(b'a'), Answer::Prefix);
        assert_eq!(cursor.advance(b't'), Answer::PrefixAndMatch(&1));
        assert_eq!(cursor.advance(b's'), Answer::Match(&2));
    }

    #[test]
    fn test_advance_slice() {
        let mut trie = ByteTrie::new();
        trie.insert(b"\x1B[A", 1);
        trie.insert(b"\x1B[B", 2);
        trie.insert(b"\x1BOA", 3);

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"\x1B["), Answer::Prefix);
        assert_eq!(cursor.advance(b'A'), Answer::Match(&1));

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"\x1BOA"), Answer::Match(&3));

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"\x1B[Z"), Answer::DeadEnd);
    }

    #[test]
    fn test_shared_prefix() {
        let mut trie = ByteTrie::new();
        trie.insert(b"\x1B[A", "up");
        trie.insert(b"\x1B[B", "down");
        trie.insert(b"\x1B[C", "right");
        trie.insert(b"\x1B[D", "left");

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"\x1B[A"), Answer::Match(&"up"));

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"\x1B[B"), Answer::Match(&"down"));

        // All arrow keys share the prefix "\x1B["
        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"\x1B["), Answer::Prefix);
    }

    #[test]
    fn test_dead_end() {
        let mut trie = ByteTrie::new();
        trie.insert(b"abc", 1);

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance(b'a'), Answer::Prefix);
        assert_eq!(cursor.advance(b'x'), Answer::DeadEnd);
        assert_eq!(cursor.deref(), Answer::DeadEnd);
    }

    #[test]
    fn test_cursor_copy() {
        let mut trie = ByteTrie::new();
        trie.insert(b"abc", 1);
        trie.insert(b"abd", 2);

        let mut cursor1 = trie.cursor();
        cursor1.advance_slice(b"ab");

        // Copy the cursor and advance differently
        let mut cursor2 = cursor1;
        assert_eq!(cursor1.advance(b'c'), Answer::Match(&1));
        assert_eq!(cursor2.advance(b'd'), Answer::Match(&2));
    }

    #[test]
    fn test_builder() {
        let mut builder = ByteTrieBuilder::new();
        builder
            .insert(b"foo", 1)
            .insert(b"bar", 2)
            .insert(b"baz", 3);

        let trie = builder.build();

        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance_slice(b"foo"), Answer::Match(&1));
    }

    #[test]
    fn test_ansi_sequences() {
        let mut trie = ByteTrie::new();

        // Common ANSI escape sequences
        trie.insert(b"\x1B[A", "cursor_up");
        trie.insert(b"\x1B[B", "cursor_down");
        trie.insert(b"\x1B[C", "cursor_forward");
        trie.insert(b"\x1B[D", "cursor_back");
        trie.insert(b"\x1B[H", "cursor_home");
        trie.insert(b"\x1B[F", "cursor_end");
        trie.insert(b"\x1B[1;2A", "shift_up");
        trie.insert(b"\x1B[1;5A", "ctrl_up");
        trie.insert(b"\x1BOA", "cursor_up_app");
        trie.insert(b"\x1BOB", "cursor_down_app");

        // Test incremental matching
        let mut cursor = trie.cursor();
        assert_eq!(cursor.advance(b'\x1B'), Answer::Prefix);
        assert_eq!(cursor.advance(b'['), Answer::Prefix);
        assert_eq!(cursor.advance(b'1'), Answer::Prefix);
        assert_eq!(cursor.advance(b';'), Answer::Prefix);
        assert_eq!(cursor.advance(b'2'), Answer::Prefix);
        assert_eq!(cursor.advance(b'A'), Answer::Match(&"shift_up"));
    }

    #[test]
    #[should_panic(expected = "7-bit encoding")]
    fn test_rejects_high_bytes() {
        let mut trie: ByteTrie<u8> = ByteTrie::new();
        trie.insert(&[0x80], 1); // Should panic - byte >= 128
    }

    #[test]
    fn test_high_byte_lookup_returns_none() {
        let mut trie: ByteTrie<u8> = ByteTrie::new();
        trie.insert(b"a", 1);

        let mut cursor = trie.cursor();
        // Looking up a high byte should return DeadEnd, not panic
        assert_eq!(cursor.advance(0x80), Answer::DeadEnd);
        assert_eq!(cursor.advance(0xFF), Answer::DeadEnd);
    }
}

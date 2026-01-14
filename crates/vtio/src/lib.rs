#![warn(clippy::pedantic)]

pub mod event;
pub mod parser;

mod traits;

pub use crate::parser::TerminalInputParser;
pub use crate::traits::{TerseDebug, TerseDisplay};

/// Re-export of [`better_any::TidExt`] as `AnyEvent` for convenient event downcasting.
///
/// Use this trait to downcast `&dyn vtansi::AnsiEvent` to concrete event types:
///
/// ```ignore
/// use vtio::AnyEvent;
/// use vtio::event::KeyEvent;
///
/// parser.feed_with(input, &mut |event| {
///     if let Some(key_event) = event.downcast_ref::<KeyEvent>() {
///         // Handle key event
///     }
/// });
/// ```
pub use better_any::TidExt as AnyEvent;

#[doc(hidden)]
pub mod __private {
    pub use paste;
}

#[cfg(test)]
mod tests {
    /// Test that reports memory usage for the actual ANSI input trie populated by vtio.
    /// Run with `cargo test -p vtio trie_memory_usage -- --nocapture` to see output.
    ///
    /// Note: Only tests input trie since output registry has known duplicate key issues.
    #[test]
    fn test_trie_memory_usage() {
        use vtansi::registry::{
            ANSI_CONTROL_INPUT_FUNCTION_REGISTRY,
            ANSI_CONTROL_INPUT_FUNCTION_TRIE,
        };

        // Force initialization of the lazy trie
        let input_stats = ANSI_CONTROL_INPUT_FUNCTION_TRIE.memory_stats();

        println!("\n=== ANSI Control Function Trie Memory Usage ===");
        println!(
            "Input registry:  {} entries",
            ANSI_CONTROL_INPUT_FUNCTION_REGISTRY.len()
        );
        println!("Input trie:      {input_stats}");
        println!("================================================\n");

        // Basic sanity checks
        assert!(input_stats.node_count > 0);
    }
}

/// Parse a colon-separated list of decimal numbers from a parameter slice.
///
/// Returns an iterator over the parsed numbers. Empty parts yield None to
/// preserve positional semantics (e.g., `"97::99"` has empty second element).
pub(crate) fn parse_colon_separated(
    bytes: &[u8],
) -> impl Iterator<Item = Option<u32>> + '_ {
    bytes.split(|&b| b == b':').map(|part| {
        if part.is_empty() {
            return None;
        }
        std::str::from_utf8(part)
            .ok()
            .and_then(|s| s.parse::<u32>().ok())
    })
}

/// A lightweight replacement for unic_char_range::CharRange
/// Represents a closed range of Unicode characters
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CharRange {
    start: char,
    end: char,
}
impl CharRange {
    /// Creates a closed character range from start to end (inclusive)
    pub(crate) fn closed(start: char, end: char) -> Self {
        panic!("STUB: not implemented");
    }
    /// Checks if the given character is within this range
    pub(crate) fn contains(&self, c: char) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns an iterator over all valid Unicode scalar values
    /// This includes U+0000 to U+D7FF and U+E000 to U+10FFFF
    /// (excludes surrogate code points U+D800 to U+DFFF)
    pub(crate) fn all() -> CharRangeIter {
        panic!("STUB: not implemented");
    }
}
/// Iterator over all valid Unicode scalar values
pub(crate) struct CharRangeIter {
    current: char,
    done: bool,
}
impl Iterator for CharRangeIter {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_char_range_contains() {
        let range = CharRange::closed('a', 'z');
        assert!(range.contains('a'));
        assert!(range.contains('m'));
        assert!(range.contains('z'));
        assert!(! range.contains('A'));
        assert!(! range.contains('0'));
    }
    #[test]
    fn test_char_range_all() {
        let all_chars: Vec<char> = CharRange::all().take(10).collect();
        assert_eq!(all_chars[0], '\0');
        assert_eq!(all_chars.len(), 10);
    }
    #[test]
    fn test_char_range_all_count() {
        let count = CharRange::all().count();
        assert_eq!(count, 0x10F800);
    }
}

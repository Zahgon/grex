use crate::char_range::CharRange;
use crate::config::RegExpConfig;
use crate::grapheme::Grapheme;
use crate::unicode_tables::{DECIMAL_NUMBER, WHITE_SPACE, WORD};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::ops::Range;
use std::sync::LazyLock;
use unicode_general_category::GeneralCategory as GC;
use unicode_segmentation::UnicodeSegmentation;
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct GraphemeCluster<'a> {
    graphemes: Vec<Grapheme>,
    config: &'a RegExpConfig,
}
impl<'a> GraphemeCluster<'a> {
    pub(crate) fn from(s: &str, config: &'a RegExpConfig) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn from_graphemes(
        graphemes: Vec<Grapheme>,
        config: &'a RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn new(grapheme: Grapheme, config: &'a RegExpConfig) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn convert_to_char_classes(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn convert_repetitions(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn merge(
        first: &GraphemeCluster,
        second: &GraphemeCluster,
        config: &'a RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn graphemes(&self) -> &Vec<Grapheme> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn graphemes_mut(&mut self) -> &mut Vec<Grapheme> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn size(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn char_count(&self, is_non_ascii_char_escaped: bool) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
fn is_digit(c: char) -> bool {
    panic!("STUB: not implemented");
}
fn is_word(c: char) -> bool {
    panic!("STUB: not implemented");
}
fn is_space(c: char) -> bool {
    panic!("STUB: not implemented");
}
fn convert_repetitions(
    graphemes: &[Grapheme],
    repetitions: &mut Vec<Grapheme>,
    config: &RegExpConfig,
) {
    panic!("STUB: not implemented");
}
fn collect_repeated_substrings(
    graphemes: &[Grapheme],
) -> HashMap<Vec<String>, Vec<usize>> {
    panic!("STUB: not implemented");
}
fn create_ranges_of_repetitions(
    repeated_substrings: HashMap<Vec<String>, Vec<usize>>,
    config: &RegExpConfig,
) -> Vec<(Range<usize>, Vec<String>)> {
    panic!("STUB: not implemented");
}
fn coalesce_repetitions(
    ranges_of_repetitions: Vec<(Range<usize>, Vec<String>)>,
) -> Vec<(Range<usize>, Vec<String>)> {
    panic!("STUB: not implemented");
}
fn replace_graphemes_with_repetitions(
    coalesced_repetitions: Vec<(Range<usize>, Vec<String>)>,
    graphemes: &[Grapheme],
    repetitions: &mut Vec<Grapheme>,
    config: &RegExpConfig,
) {
    panic!("STUB: not implemented");
}
fn convert_chars_to_range(chars: &[(char, char)]) -> Vec<CharRange> {
    panic!("STUB: not implemented");
}

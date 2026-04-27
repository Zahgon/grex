use crate::component::Component;
use itertools::Itertools;
use std::fmt::{Display, Formatter, Result};
const CHARS_TO_ESCAPE: [&str; 14] = [
    "(",
    ")",
    "[",
    "]",
    "{",
    "}",
    "+",
    "*",
    "-",
    ".",
    "?",
    "|",
    "^",
    "$",
];
const CHAR_CLASSES: [&str; 6] = ["\\d", "\\s", "\\w", "\\D", "\\S", "\\W"];
#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct Grapheme {
    pub(crate) chars: Vec<String>,
    pub(crate) repetitions: Vec<Grapheme>,
    min: u32,
    max: u32,
    is_capturing_group_enabled: bool,
    is_output_colorized: bool,
    is_verbose_mode_enabled: bool,
}
impl Grapheme {
    pub(crate) fn from(
        s: &str,
        is_capturing_group_enabled: bool,
        is_output_colorized: bool,
        is_verbose_mode_enabled: bool,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn new(
        chars: Vec<String>,
        min: u32,
        max: u32,
        is_capturing_group_enabled: bool,
        is_output_colorized: bool,
        is_verbose_mode_enabled: bool,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn value(&self) -> String {
        panic!("STUB: not implemented");
    }
    pub(crate) fn chars(&self) -> &Vec<String> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn chars_mut(&mut self) -> &mut Vec<String> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn has_repetitions(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn repetitions_mut(&mut self) -> &mut Vec<Grapheme> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn minimum(&self) -> u32 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn maximum(&self) -> u32 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn char_count(&self, is_non_ascii_char_escaped: bool) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn escape_non_ascii_chars(&mut self, use_surrogate_pairs: bool) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn escape_regexp_symbols(
        &mut self,
        is_non_ascii_char_escaped: bool,
        is_astral_code_point_converted_to_surrogate: bool,
    ) {
        panic!("STUB: not implemented");
    }
    fn escape(&self, c: char, use_surrogate_pairs: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn convert_to_surrogate_pair(&self, c: char) -> String {
        panic!("STUB: not implemented");
    }
}
impl Display for Grapheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("STUB: not implemented");
    }
}

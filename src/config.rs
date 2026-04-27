#[derive(Clone, Debug, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) struct RegExpConfig {
    pub(crate) minimum_repetitions: u32,
    pub(crate) minimum_substring_length: u32,
    pub(crate) is_digit_converted: bool,
    pub(crate) is_non_digit_converted: bool,
    pub(crate) is_space_converted: bool,
    pub(crate) is_non_space_converted: bool,
    pub(crate) is_word_converted: bool,
    pub(crate) is_non_word_converted: bool,
    pub(crate) is_repetition_converted: bool,
    pub(crate) is_case_insensitive_matching: bool,
    pub(crate) is_capturing_group_enabled: bool,
    pub(crate) is_non_ascii_char_escaped: bool,
    pub(crate) is_astral_code_point_converted_to_surrogate: bool,
    pub(crate) is_verbose_mode_enabled: bool,
    pub(crate) is_start_anchor_disabled: bool,
    pub(crate) is_end_anchor_disabled: bool,
    pub(crate) is_output_colorized: bool,
}
impl RegExpConfig {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_char_class_feature_enabled(&self) -> bool {
        panic!("STUB: not implemented");
    }
}

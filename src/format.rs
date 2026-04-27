use crate::char_range::CharRange;
use crate::cluster::GraphemeCluster;
use crate::component::Component;
use crate::expression::Expression;
use crate::quantifier::Quantifier;
use itertools::Itertools;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter, Result};
impl Display for Expression<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("STUB: not implemented");
    }
}
fn get_codepoint_position(c: char) -> usize {
    panic!("STUB: not implemented");
}
fn format_alternation(
    f: &mut Formatter<'_>,
    expr: &Expression,
    options: &[Expression],
    is_capturing_group_enabled: bool,
    is_output_colorized: bool,
    is_verbose_mode_enabled: bool,
) -> Result {
    panic!("STUB: not implemented");
}
fn format_character_class(
    f: &mut Formatter<'_>,
    char_set: &BTreeSet<char>,
    is_output_colorized: bool,
) -> Result {
    panic!("STUB: not implemented");
}
fn format_concatenation(
    f: &mut Formatter<'_>,
    expr: &Expression,
    expr1: &Expression,
    expr2: &Expression,
    is_capturing_group_enabled: bool,
    is_output_colorized: bool,
    is_verbose_mode_enabled: bool,
) -> Result {
    panic!("STUB: not implemented");
}
fn format_literal(
    f: &mut Formatter<'_>,
    cluster: &GraphemeCluster,
    is_non_ascii_char_escaped: bool,
    is_astral_code_point_converted_to_surrogate: bool,
) -> Result {
    panic!("STUB: not implemented");
}
fn format_repetition(
    f: &mut Formatter<'_>,
    expr: &Expression,
    expr1: &Expression,
    quantifier: &Quantifier,
    is_capturing_group_enabled: bool,
    is_output_colorized: bool,
    is_verbose_mode_enabled: bool,
) -> Result {
    panic!("STUB: not implemented");
}

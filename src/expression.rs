use crate::cluster::GraphemeCluster;
use crate::config::RegExpConfig;
use crate::dfa::Dfa;
use crate::grapheme::Grapheme;
use crate::quantifier::Quantifier;
use crate::substring::Substring;
use itertools::EitherOrBoth::Both;
use itertools::Itertools;
use ndarray::{Array1, Array2};
use petgraph::prelude::EdgeRef;
use std::cmp::Reverse;
use std::collections::BTreeSet;
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Expression<'a> {
    Alternation(Vec<Expression<'a>>, bool, bool, bool),
    CharacterClass(BTreeSet<char>, bool),
    Concatenation(Box<Expression<'a>>, Box<Expression<'a>>, bool, bool, bool),
    Literal(GraphemeCluster<'a>, bool, bool),
    Repetition(Box<Expression<'a>>, Quantifier, bool, bool, bool),
}
impl<'a> Expression<'a> {
    pub(crate) fn from(dfa: Dfa, config: &'a RegExpConfig) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn new_alternation(
        exprs: Vec<Expression<'a>>,
        config: &RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn new_character_class(
        first_char_set: BTreeSet<char>,
        second_char_set: BTreeSet<char>,
        config: &RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn new_concatenation(
        expr1: Expression<'a>,
        expr2: Expression<'a>,
        config: &RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn new_literal(
        cluster: GraphemeCluster<'a>,
        config: &RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn new_repetition(
        expr: Expression<'a>,
        quantifier: Quantifier,
        config: &RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_single_codepoint(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn precedence(&self) -> u8 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn remove_substring(&mut self, substring: &Substring, length: usize) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn value(&self, substring: Option<&Substring>) -> Option<Vec<Grapheme>> {
        panic!("STUB: not implemented");
    }
    fn repeat_zero_or_more_times(
        expr: &Option<Expression<'a>>,
        config: &'a RegExpConfig,
    ) -> Option<Expression<'a>> {
        panic!("STUB: not implemented");
    }
    fn concatenate(
        a: &Option<Expression<'a>>,
        b: &Option<Expression<'a>>,
        config: &'a RegExpConfig,
    ) -> Option<Expression<'a>> {
        panic!("STUB: not implemented");
    }
    fn union(
        a: &Option<Expression<'a>>,
        b: &Option<Expression<'a>>,
        config: &'a RegExpConfig,
    ) -> Option<Expression<'a>> {
        panic!("STUB: not implemented");
    }
    fn flatten_alternations(
        flattened_options: &mut Vec<Expression<'a>>,
        current_options: Vec<Expression<'a>>,
    ) {
        panic!("STUB: not implemented");
    }
    fn extract_character_set(expr: Expression) -> BTreeSet<char> {
        panic!("STUB: not implemented");
    }
    fn remove_common_substring(
        a: &mut Expression,
        b: &mut Expression,
        substring: Substring,
    ) -> Option<Vec<Grapheme>> {
        panic!("STUB: not implemented");
    }
    fn find_common_substring(
        a: &Expression,
        b: &Expression,
        substring: &Substring,
    ) -> Option<Vec<Grapheme>> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ensure_correct_string_representation_of_alternation_1() {
        let config = RegExpConfig::new();
        let literal1 = Expression::new_literal(
            GraphemeCluster::from("abc", &config),
            &config,
        );
        let literal2 = Expression::new_literal(
            GraphemeCluster::from("def", &config),
            &config,
        );
        let alternation = Expression::new_alternation(vec![literal1, literal2], &config);
        assert_eq!(alternation.to_string(), "abc|def");
    }
    #[test]
    fn ensure_correct_string_representation_of_alternation_2() {
        let config = RegExpConfig::new();
        let literal1 = Expression::new_literal(
            GraphemeCluster::from("a", &config),
            &config,
        );
        let literal2 = Expression::new_literal(
            GraphemeCluster::from("ab", &config),
            &config,
        );
        let literal3 = Expression::new_literal(
            GraphemeCluster::from("abc", &config),
            &config,
        );
        let alternation = Expression::new_alternation(
            vec![literal1, literal2, literal3],
            &config,
        );
        assert_eq!(alternation.to_string(), "abc|ab|a");
    }
    #[test]
    fn ensure_correct_string_representation_of_character_class_1() {
        let config = RegExpConfig::new();
        let char_class = Expression::new_character_class(
            btreeset!['a'],
            btreeset!['b'],
            &config,
        );
        assert_eq!(char_class.to_string(), "[ab]");
    }
    #[test]
    fn ensure_correct_string_representation_of_character_class_2() {
        let config = RegExpConfig::new();
        let char_class = Expression::new_character_class(
            btreeset!['a', 'b'],
            btreeset!['c'],
            &config,
        );
        assert_eq!(char_class.to_string(), "[a-c]");
    }
    #[test]
    fn ensure_correct_string_representation_of_concatenation_1() {
        let config = RegExpConfig::new();
        let literal1 = Expression::new_literal(
            GraphemeCluster::from("abc", &config),
            &config,
        );
        let literal2 = Expression::new_literal(
            GraphemeCluster::from("def", &config),
            &config,
        );
        let concatenation = Expression::new_concatenation(literal1, literal2, &config);
        assert_eq!(concatenation.to_string(), "abcdef");
    }
    #[test]
    fn ensure_correct_string_representation_of_concatenation_2() {
        let config = RegExpConfig::new();
        let literal1 = Expression::new_literal(
            GraphemeCluster::from("abc", &config),
            &config,
        );
        let literal2 = Expression::new_literal(
            GraphemeCluster::from("def", &config),
            &config,
        );
        let repetition = Expression::new_repetition(
            literal1,
            Quantifier::KleeneStar,
            &config,
        );
        let concatenation = Expression::new_concatenation(repetition, literal2, &config);
        assert_eq!(concatenation.to_string(), "(?:abc)*def");
    }
    #[test]
    fn ensure_correct_removal_of_prefix_in_literal() {
        let config = RegExpConfig::new();
        let mut literal = Expression::new_literal(
            GraphemeCluster::from("abcdef", &config),
            &config,
        );
        assert_eq!(
            literal.value(None), Some(vec!["a", "b", "c", "d", "e", "f"] .iter().map(|&
            it | Grapheme::from(it, config.is_capturing_group_enabled, config
            .is_output_colorized, config.is_verbose_mode_enabled)).collect_vec())
        );
        literal.remove_substring(&Substring::Prefix, 2);
        assert_eq!(
            literal.value(None), Some(vec!["c", "d", "e", "f"] .iter().map(|& it |
            Grapheme::from(it, config.is_capturing_group_enabled, config
            .is_output_colorized, config.is_verbose_mode_enabled)).collect_vec())
        );
    }
    #[test]
    fn ensure_correct_removal_of_suffix_in_literal() {
        let config = RegExpConfig::new();
        let mut literal = Expression::new_literal(
            GraphemeCluster::from("abcdef", &config),
            &config,
        );
        assert_eq!(
            literal.value(None), Some(vec!["a", "b", "c", "d", "e", "f"] .iter().map(|&
            it | Grapheme::from(it, config.is_capturing_group_enabled, config
            .is_output_colorized, config.is_verbose_mode_enabled)).collect_vec())
        );
        literal.remove_substring(&Substring::Suffix, 2);
        assert_eq!(
            literal.value(None), Some(vec!["a", "b", "c", "d"] .iter().map(|& it |
            Grapheme::from(it, config.is_capturing_group_enabled, config
            .is_output_colorized, config.is_verbose_mode_enabled)).collect_vec())
        );
    }
    #[test]
    fn ensure_correct_string_representation_of_repetition_1() {
        let config = RegExpConfig::new();
        let literal = Expression::new_literal(
            GraphemeCluster::from("abc", &config),
            &config,
        );
        let repetition = Expression::new_repetition(
            literal,
            Quantifier::KleeneStar,
            &config,
        );
        assert_eq!(repetition.to_string(), "(?:abc)*");
    }
    #[test]
    fn ensure_correct_string_representation_of_repetition_2() {
        let config = RegExpConfig::new();
        let literal = Expression::new_literal(
            GraphemeCluster::from("a", &config),
            &config,
        );
        let repetition = Expression::new_repetition(
            literal,
            Quantifier::QuestionMark,
            &config,
        );
        assert_eq!(repetition.to_string(), "a?");
    }
}

use crate::cluster::GraphemeCluster;
use crate::component::Component;
use crate::config::RegExpConfig;
use crate::dfa::Dfa;
use crate::expression::Expression;
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
pub(crate) struct RegExp<'a> {
    ast: Expression<'a>,
    config: &'a RegExpConfig,
}
impl<'a> RegExp<'a> {
    pub(crate) fn from(
        test_cases: &'a mut Vec<String>,
        config: &'a RegExpConfig,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    fn convert_for_case_insensitive_matching(test_cases: &mut Vec<String>) {
        panic!("STUB: not implemented");
    }
    fn convert_expr_to_regex(expr: &Expression, config: &RegExpConfig) -> Regex {
        panic!("STUB: not implemented");
    }
    fn regex_matches_all_test_cases(regex: &Regex, test_cases: &[String]) -> bool {
        panic!("STUB: not implemented");
    }
    fn sort(test_cases: &mut Vec<String>) {
        panic!("STUB: not implemented");
    }
    fn grapheme_clusters(
        test_cases: &'a [String],
        config: &'a RegExpConfig,
    ) -> Vec<GraphemeCluster<'a>> {
        panic!("STUB: not implemented");
    }
    fn is_each_test_case_matched_after_rotating_alternations(
        regex: &Regex,
        expr: &mut Expression,
        test_cases: &[String],
    ) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Display for RegExp<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("STUB: not implemented");
    }
}
fn indent_regexp(regexp: String, config: &RegExpConfig) -> String {
    panic!("STUB: not implemented");
}

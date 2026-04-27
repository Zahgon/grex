use std::fmt::{Display, Formatter, Result};
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum Quantifier {
    KleeneStar,
    QuestionMark,
}
impl Display for Quantifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("STUB: not implemented");
    }
}

use crate::quantifier::Quantifier;
use std::fmt::{Display, Formatter, Result};
pub(crate) enum Component {
    CapturedLeftParenthesis,
    CapturedParenthesizedExpression(String, bool, bool),
    Caret(bool),
    CharClass(String),
    DollarSign(bool),
    Hyphen,
    IgnoreCaseFlag,
    IgnoreCaseAndVerboseModeFlag,
    LeftBracket,
    Pipe,
    Quantifier(Quantifier, bool),
    Repetition(u32, bool),
    RepetitionRange(u32, u32, bool),
    RightBracket,
    RightParenthesis,
    UncapturedLeftParenthesis,
    UncapturedParenthesizedExpression(String, bool, bool),
    VerboseModeFlag,
}
impl Component {
    pub(crate) fn to_repr(&self, is_output_colorized: bool) -> String {
        panic!("STUB: not implemented");
    }
    pub(crate) fn to_colored_string(&self, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn black_on_bright_yellow(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn bright_yellow_on_black(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn cyan_bold(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn green_bold(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn purple_bold(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn red_bold(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn white_on_bright_blue(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn yellow_bold(value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
    fn color_code(code: &str, value: &str, is_escaped: bool) -> String {
        panic!("STUB: not implemented");
    }
}
impl Display for Component {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        panic!("STUB: not implemented");
    }
}

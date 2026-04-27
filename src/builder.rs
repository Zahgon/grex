use crate::config::RegExpConfig;
use crate::regexp::RegExp;
use itertools::Itertools;
use std::io::ErrorKind;
use std::path::PathBuf;
pub(crate) const MISSING_TEST_CASES_MESSAGE: &str = "No test cases have been provided for regular expression generation";
pub(crate) const MINIMUM_REPETITIONS_MESSAGE: &str = "Quantity of minimum repetitions must be greater than zero";
pub(crate) const MINIMUM_SUBSTRING_LENGTH_MESSAGE: &str = "Minimum substring length must be greater than zero";
/// This struct builds regular expressions from user-provided test cases.
#[derive(Clone)]
#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
pub struct RegExpBuilder {
    pub(crate) test_cases: Vec<String>,
    pub(crate) config: RegExpConfig,
}
impl RegExpBuilder {
    /// Specifies the test cases to build the regular expression from.
    ///
    /// The test cases need not be sorted because `RegExpBuilder` sorts them internally.
    ///
    /// ⚠ Panics if `test_cases` is empty.
    pub fn from<T: Clone + Into<String>>(test_cases: &[T]) -> Self {
        panic!("STUB: not implemented");
    }
    /// Specifies a text file containing test cases to build the regular expression from.
    ///
    /// The test cases need not be sorted because `RegExpBuilder` sorts them internally.
    ///
    /// Each test case needs to be on a separate line.
    /// Lines may be ended with either a newline (`\n`) or
    /// a carriage return with a line feed (`\r\n`).
    /// The final line ending is optional.
    ///
    /// ⚠ Panics if:
    /// - the file cannot be found
    /// - the file's encoding is not valid UTF-8 data
    /// - the file cannot be opened because of conflicting permissions
    pub fn from_file<T: Into<PathBuf>>(file_path: T) -> Self {
        panic!("STUB: not implemented");
    }
    /// Converts any Unicode decimal digit to character class `\d`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_words`](Self::with_conversion_of_words) if both are set.
    /// Decimal digits are converted to `\d`, the remaining word characters to `\w`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_whitespace`](Self::with_conversion_of_non_whitespace) if both are set.
    /// Decimal digits are converted to `\d`, the remaining non-whitespace characters to `\S`.
    pub fn with_conversion_of_digits(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Converts any character which is not a Unicode decimal digit to character class `\D`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_words`](Self::with_conversion_of_non_words) if both are set.
    /// Non-digits which are also non-word characters are converted to `\D`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_whitespace`](Self::with_conversion_of_non_whitespace) if both are set.
    /// Non-digits which are also non-space characters are converted to `\D`.
    pub fn with_conversion_of_non_digits(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Converts any Unicode whitespace character to character class `\s`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_digits`](Self::with_conversion_of_non_digits) if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-digit characters to `\D`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_words`](Self::with_conversion_of_non_words) if both are set.
    /// Whitespace characters are converted to `\s`, the remaining non-word characters to `\W`.
    pub fn with_conversion_of_whitespace(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Converts any character which is not a Unicode whitespace character to character class `\S`.
    pub fn with_conversion_of_non_whitespace(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Converts any Unicode word character to character class `\w`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_digits`](Self::with_conversion_of_non_digits) if both are set.
    /// Word characters are converted to `\w`, the remaining non-digit characters to `\D`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_whitespace`](Self::with_conversion_of_non_whitespace) if both are set.
    /// Word characters are converted to `\w`, the remaining non-space characters to `\S`.
    pub fn with_conversion_of_words(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Converts any character which is not a Unicode word character to character class `\W`.
    ///
    /// This method takes precedence over
    /// [`with_conversion_of_non_whitespace`](Self::with_conversion_of_non_whitespace) if both are set.
    /// Non-words which are also non-space characters are converted to `\W`.
    pub fn with_conversion_of_non_words(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Detects repeated non-overlapping substrings and
    /// to convert them to `{min,max}` quantifier notation.
    pub fn with_conversion_of_repetitions(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Enables case-insensitive matching of test cases
    /// so that letters match both upper and lower case.
    pub fn with_case_insensitive_matching(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Replaces non-capturing groups with capturing ones.
    pub fn with_capturing_groups(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Specifies the minimum quantity of substring repetitions to be converted if
    /// [`with_conversion_of_repetitions`](Self::with_conversion_of_repetitions) is set.
    ///
    /// If the quantity is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// ⚠ Panics if `quantity` is zero.
    pub fn with_minimum_repetitions(&mut self, quantity: u32) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Specifies the minimum length a repeated substring must have in order to be converted if
    /// [`with_conversion_of_repetitions`](Self::with_conversion_of_repetitions) is set.
    ///
    /// If the length is not explicitly set with this method, a default value of 1 will be used.
    ///
    /// ⚠ Panics if `length` is zero.
    pub fn with_minimum_substring_length(&mut self, length: u32) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Converts non-ASCII characters to unicode escape sequences.
    /// The parameter `use_surrogate_pairs` specifies whether to convert astral code planes
    /// (range `U+010000` to `U+10FFFF`) to surrogate pairs.
    pub fn with_escaping_of_non_ascii_chars(
        &mut self,
        use_surrogate_pairs: bool,
    ) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Produces a nicer looking regular expression in verbose mode.
    pub fn with_verbose_mode(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Removes the caret anchor '^' from the resulting regular
    /// expression, thereby allowing to match the test cases also when they do not occur
    /// at the start of a string.
    pub fn without_start_anchor(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Removes the dollar sign anchor '$' from the resulting regular
    /// expression, thereby allowing to match the test cases also when they do not occur
    /// at the end of a string.
    pub fn without_end_anchor(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Removes the caret and dollar sign anchors from the resulting
    /// regular expression, thereby allowing to match the test cases also when they occur
    /// within a larger string that contains other content as well.
    pub fn without_anchors(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Provides syntax highlighting for the resulting regular expression.
    ///
    /// ⚠ This method may only be used if the resulting regular expression is meant to
    /// be printed to the console. The regex string representation returned from enabling
    /// this setting cannot be fed into the [*regex*](https://crates.io/crates/regex) crate.
    #[cfg(feature = "cli")]
    #[doc(hidden)]
    pub fn with_syntax_highlighting(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Builds the actual regular expression using the previously given settings.
    pub fn build(&mut self) -> String {
        panic!("STUB: not implemented");
    }
}

#[cfg(not(target_family = "wasm"))]
mod cli {
    use clap::ArgAction;
    use clap::Parser;
    use grex::RegExpBuilder;
    use itertools::Itertools;
    use std::io::{stdin, BufRead, Error, ErrorKind, IsTerminal, Read};
    use std::path::PathBuf;
    #[derive(Parser)]
    #[command(
        author = "© 2019-today Peter M. Stahl <pemistahl@gmail.com>",
        about = "Licensed under the Apache License, Version 2.0\n\
                 Downloadable from https://crates.io/crates/grex\n\
                 Source code at https://github.com/pemistahl/grex\n\n\
                 grex generates regular expressions from user-provided test cases.",
        version,
        override_usage = "grex [OPTIONS] {INPUT...|--file <FILE>}",
        help_template = "{name} {version}\n{author}\n{about}\n\n{usage-heading} {usage}\n\n{all-args}",
        disable_help_flag = true,
        disable_version_flag = true
    )]
    pub(crate) struct Cli {
        /// One or more test cases separated by blank space
        ///
        /// Use a hyphen `-` to read test cases from standard input.
        ///
        /// Conflicts with --file.
        #[arg(
            value_name = "INPUT",
            allow_hyphen_values = true,
            required_unless_present = "file",
            conflicts_with = "file",
            help_heading = "Input",
            display_order = 1
        )]
        input: Vec<String>,
        /// Reads test cases on separate lines from a file.
        ///
        /// Lines may be ended with either a newline `\n` or a carriage return with a line feed `\r\n`.
        /// The final line ending is optional.
        ///
        /// Use a hyphen `-` to read the filename from standard input.
        ///
        /// Conflicts with INPUT...
        #[arg(
            name = "file",
            value_name = "FILE",
            short,
            long,
            required_unless_present = "input",
            help_heading = "Input",
            display_order = 2
        )]
        file_path: Option<PathBuf>,
        /// Converts any Unicode decimal digit to \d.
        ///
        /// Takes precedence over --words if both are set.
        /// Decimal digits are converted to \d, remaining word characters to \w.
        ///
        /// Takes precedence over --non-spaces if both are set.
        /// Decimal digits are converted to \d, remaining non-space characters to \S.
        #[arg(name = "digits", short, long, help_heading = "Digit Options")]
        is_digit_converted: bool,
        /// Converts any character which is not a Unicode decimal digit to \D.
        ///
        /// Takes precedence over --non-words if both are set.
        /// Non-digits which are also non-word characters are converted to \D.
        ///
        /// Takes precedence over --non-spaces if both are set.
        /// Non-digits which are also non-space characters are converted to \D.
        #[arg(name = "non-digits", short = 'D', long, help_heading = "Digit Options")]
        is_non_digit_converted: bool,
        /// Converts any Unicode whitespace character to \s.
        ///
        /// Takes precedence over --non-digits if both are set.
        /// Whitespace is converted to \s, remaining non-digits to \D.
        ///
        /// Takes precedence over --non-words if both are set.
        /// Whitespace is converted to \s, remaining non-word characters to \W.
        #[arg(name = "spaces", short, long, help_heading = "Whitespace Options")]
        is_space_converted: bool,
        /// Converts any character which is not a Unicode whitespace character to \S
        #[arg(
            name = "non-spaces",
            short = 'S',
            long,
            help_heading = "Whitespace Options"
        )]
        is_non_space_converted: bool,
        /// Converts any Unicode word character to \w.
        ///
        /// Takes precedence over --non-digits if both are set.
        /// Word characters are converted to \w, remaining non-digits to \D.
        ///
        /// Takes precedence over --non-spaces if both are set.
        /// Word characters are converted to \w, remaining non-whitespace to \S.
        #[arg(name = "words", short, long, help_heading = "Word Options")]
        is_word_converted: bool,
        /// Converts any character which is not a Unicode word character to \W.
        ///
        /// Takes precedence over --non-spaces if both are set.
        /// Non-word characters which are also non-whitespace are converted to \W.
        #[arg(name = "non-words", short = 'W', long, help_heading = "Word Options")]
        is_non_word_converted: bool,
        /// Replaces all non-ASCII characters with unicode escape sequences.
        #[arg(name = "escape", short, long, help_heading = "Escaping Options")]
        is_non_ascii_char_escaped: bool,
        /// Converts astral code points to surrogate pairs if --escape is set.
        #[arg(
            name = "with-surrogates",
            long,
            requires = "escape",
            help_heading = "Escaping Options"
        )]
        is_astral_code_point_converted_to_surrogate: bool,
        /// Detects repeated non-overlapping substrings and converts them to {min,max} quantifier notation.
        #[arg(
            name = "repetitions",
            short,
            long,
            help_heading = "Repetition Options",
            display_order = 1
        )]
        is_repetition_converted: bool,
        /// Specifies the minimum quantity of substring repetitions to be converted if --repetitions is set.
        #[arg(
            name = "min-repetitions",
            value_name = "QUANTITY",
            long,
            default_value_t = 1,
            value_parser = repetition_options_parser,
            help_heading = "Repetition Options"
        )]
        minimum_repetitions: u32,
        /// Specifies the minimum length a repeated substring must have
        /// in order to be converted if --repetitions is set.
        #[arg(
            name = "min-substring-length",
            value_name = "LENGTH",
            long,
            default_value_t = 1,
            value_parser = repetition_options_parser,
            help_heading = "Repetition Options"
        )]
        minimum_substring_length: u32,
        /// Removes the caret anchor `^` from the resulting regular expression.
        ///
        /// By default, the caret anchor is added to every generated regular expression
        /// which guarantees that the expression matches the test cases
        /// given as input only at the start of a string.
        ///
        /// This flag removes the anchor, thereby allowing to match the test cases
        /// also when they do not occur at the start of a string.
        #[arg(name = "no-start-anchor", long, help_heading = "Anchor Options")]
        is_caret_anchor_disabled: bool,
        /// Removes the dollar sign anchor `$` from the resulting regular expression.
        ///
        /// By default, the dollar sign anchor is added to every generated regular expression
        /// which guarantees that the expression matches the test cases given as input
        /// only at the end of a string.
        ///
        /// This flag removes the anchor, thereby allowing to match the test cases
        /// also when they do not occur at the end of a string.
        #[arg(name = "no-end-anchor", long, help_heading = "Anchor Options")]
        is_dollar_sign_anchor_disabled: bool,
        /// Removes the caret and dollar sign anchors from the resulting regular expression.
        ///
        /// By default, anchors are added to every generated regular expression
        /// which guarantees that the expression exactly matches only the test cases given as input
        /// and nothing else.
        ///
        /// This flag removes the anchors, thereby allowing to match the test cases
        /// also when they occur within a larger string that contains other content as well.
        #[arg(name = "no-anchors", long, help_heading = "Anchor Options")]
        are_anchors_disabled: bool,
        /// Produces a nicer-looking regular expression in verbose mode.
        #[arg(
            name = "verbose",
            short = 'x',
            long,
            help_heading = "Display Options",
            display_order = 1
        )]
        is_verbose_mode_enabled: bool,
        /// Provides syntax highlighting for the resulting regular expression.
        #[arg(name = "colorize", short, long, help_heading = "Display Options")]
        is_output_colorized: bool,
        /// Performs case-insensitive matching, letters match both upper and lower case.
        #[arg(
            name = "ignore-case",
            short,
            long,
            help_heading = "Miscellaneous Options",
            display_order = 1
        )]
        is_case_ignored: bool,
        /// Replaces non-capturing groups with capturing ones.
        #[arg(
            name = "capture-groups",
            short = 'g',
            long,
            help_heading = "Miscellaneous Options",
            display_order = 2
        )]
        is_group_captured: bool,
        /// Prints help information
        #[arg(
            name = "help",
            short = 'h',
            long,
            action = ArgAction::Help,
            help_heading = "Miscellaneous Options",
            display_order = 3
        )]
        help: Option<String>,
        /// Prints version information
        #[arg(
            name = "version",
            short = 'v',
            long,
            action = ArgAction::Version,
            help_heading = "Miscellaneous Options",
            display_order = 4
        )]
        version: Option<String>,
    }
    pub(crate) fn obtain_input(cli: &Cli) -> Result<Vec<String>, Error> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn handle_input(
        cli: &Cli,
        input: Result<Vec<String>, Error>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        panic!("STUB: not implemented");
    }
    fn repetition_options_parser(value: &str) -> Result<u32, String> {
        panic!("STUB: not implemented");
    }
}
#[cfg(not(target_family = "wasm"))]
fn main() {
    use clap::Parser;
    let cli = cli::Cli::parse();
    if let Err(e) = cli::handle_input(&cli, cli::obtain_input(&cli)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
#[cfg(target_family = "wasm")]
fn main() {}

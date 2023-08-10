/// Source line representation (string + line number) for use in the Nexus scanner.
///
/// # Example
///
/// ```
/// use nexus_rs::source_line::SourceLine;
///
/// let line = "let x;".to_string();
/// let number = 42;
///
/// let sl = SourceLine{ line, Some(number) };
/// ```
#[derive(Clone, Debug)]
pub struct SourceLine {
    pub line: String,
    pub number: Option<usize>,
}

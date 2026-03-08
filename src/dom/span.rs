use ownable::{IntoOwned, ToBorrowed, ToOwned};
use serde::Serialize;
use std::borrow::Cow;

/// Span of the information in the parsed source.
#[derive(Debug, Default, Clone, Serialize, PartialEq, IntoOwned, ToBorrowed, ToOwned)]
#[serde(rename_all = "camelCase")]
pub struct SourceSpan<'a> {
    pub text: Cow<'a, str>,
    pub start_line: usize,
    pub end_line: usize,
    pub start_column: usize,
    pub end_column: usize,
}

impl<'a> SourceSpan<'a> {
    pub fn new(
        text: &'a str,
        start_line: usize,
        end_line: usize,
        start_column: usize,
        end_column: usize,
    ) -> Self {
        Self {
            text: text.into(),
            start_line,
            end_line,
            start_column,
            end_column,
        }
    }
}

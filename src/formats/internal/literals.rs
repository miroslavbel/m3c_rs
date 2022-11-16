//! Raw literals.

/// Label identifier literal.
///
/// Matches the regex `[0-9A-Za-z]{0,3}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LabelIdentifierLiteral {
    data: [u8; 4],
}

/// String literal.
///
/// Matches the regex `[0-9A-Za-z]{0,3}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StringLiteral {
    data: [u8; 4],
}

/// Variable identifier literal.
///
/// Matches the regex `[0-9A-Za-z]{0,3}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableIdentifierLiteral {
    data: [u8; 4],
}

/// Variable value literal.
///
/// The value can be in the range `[-9_999, 99_999]`. Default value is `0`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableValueLiteral {
    data: i32,
}

//! Raw literals.

use std::{error::Error, fmt, iter::Enumerate, str::Chars};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IllegalCharError {
    index: usize,
}

impl IllegalCharError {
    /// Returns the contained illegal char.
    pub fn index(&self) -> usize {
        self.index
    }
}

impl fmt::Display for IllegalCharError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an illegal char was found at index {}", self.index)
    }
}

impl Error for IllegalCharError {}

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

impl LabelIdentifierLiteral {
    /// Greedy creates a new literal from a `&mut Enumerate<Chars>`.
    ///
    /// Consumes the given `enumerate` until the first illegal char is found (if any) and returns
    /// it (if any). Only the first three legal chars aren't ignored.
    pub fn new_from_enumerate(enumerate: &mut Enumerate<Chars>) -> (Self, Option<(usize, char)>) {
        let mut literal = Self { data: [0; 4] };
        let mut index = 0;
        loop {
            let next = enumerate.next();
            match next {
                None => return (literal, None::<(usize, char)>),
                Some(last_item) => {
                    if last_item.1.is_ascii_alphanumeric() {
                        if index < 3 {
                            literal.data[index] = last_item.1 as u8;
                        }
                        index += 1;
                        continue;
                    }
                    return (literal, Some(last_item));
                }
            }
        }
    }
    /// Creates a new literal from `[u8; 4]` ignoring the last element.
    ///
    /// # Errors
    ///
    /// If the first three chars have an illegal char, the [`IllegalCharError`] will be returned.
    pub fn new_from_array(data: [u8; 4]) -> Result<Self, IllegalCharError> {
        let mut internal_data = [0; 4];
        let mut index = 0;
        while index < 3 {
            let ch = data[index];
            if ch.is_ascii_alphanumeric() {
                internal_data[index] = ch;
                index += 1;
                continue;
            } else if ch == 0 {
                return Ok(Self {
                    data: internal_data,
                });
            }
            return Err(IllegalCharError { index });
        }
        Ok(Self {
            data: internal_data,
        })
    }
    /// Returns the underlying data.
    ///
    /// The last element of the array is always zero.
    pub fn data(&self) -> [u8; 4] {
        self.data
    }
}

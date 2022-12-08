//! Raw literals.

use std::{error::Error, fmt, io, iter::Enumerate, str, str::Chars};

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IllegalVariableValueError {}

impl fmt::Display for IllegalVariableValueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an illegal variable value was given")
    }
}

impl Error for IllegalVariableValueError {}

pub trait Literal {
    /// The maximum possible length in chars that a literal can occupy in a `str`.
    const MAX_CHAR_LEN: usize;
    /// Greedy creates a new literal from a `&mut Enumerate<Chars>`.
    ///
    /// Consumes the given `enumerate` until the first illegal char is found (if any) and returns
    /// it (if any). Only the first [`MAX_CHAR_LEN`] legal chars aren't ignored.
    ///
    /// [`MAX_CHAR_LEN`]: Self::MAX_CHAR_LEN
    fn new_from_enumerate(enumerate: &mut Enumerate<Chars>) -> (Self, Option<(usize, char)>)
    where
        Self: Sized;
    /// Dumps this literal to the given `String`.
    fn dumps_to(&self, s: &mut String);
    /// Writes this literal to the given `writer`.
    ///
    /// Internally uses the [`write_all`] method.
    ///
    /// # Errors
    ///
    /// See the [`write_all`]'s `Errors` sections.
    ///
    /// [`write_all`]: io::Write::write_all
    fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write;
}

/// Label identifier literal.
///
/// Matches the regex `[0-9A-Za-z]{0,3}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct LabelIdentifierLiteral {
    data: [u8; 4],
}

impl LabelIdentifierLiteral {
    /// Creates a new literal from `[u8; 4]` ignoring the last element.
    ///
    /// # Errors
    ///
    /// If the first three chars have an illegal char, the [`IllegalCharError`] will be returned.
    pub fn new_from_array(data: [u8; 4]) -> Result<Self, IllegalCharError> {
        let mut internal_data = [0; 4];
        let mut index = 0;
        while index < Self::MAX_CHAR_LEN {
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
    /// Returns the length of this literal.
    pub fn len(&self) -> usize {
        self.data.iter().position(|&b| b == 0).unwrap()
    }
    /// Checks if this literal is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data[0] == 0
    }
    /// Returns the underlying data.
    ///
    /// The last element of the array is always zero.
    pub fn data(&self) -> [u8; 4] {
        self.data
    }
}

impl Literal for LabelIdentifierLiteral {
    const MAX_CHAR_LEN: usize = 3;
    fn new_from_enumerate(enumerate: &mut Enumerate<Chars>) -> (Self, Option<(usize, char)>) {
        let mut literal = Self { data: [0; 4] };
        let mut index = 0;
        loop {
            let next = enumerate.next();
            match next {
                None => return (literal, None::<(usize, char)>),
                Some(last_item) => {
                    if last_item.1.is_ascii_alphanumeric() {
                        if index < Self::MAX_CHAR_LEN {
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
    fn dumps_to(&self, s: &mut String) {
        if !self.is_empty() {
            s.push_str(str::from_utf8(&self.data[0..self.len()]).unwrap());
        }
    }
    fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        if !self.is_empty() {
            return writer.write_all(&self.data[0..self.len()]);
        }
        Ok(())
    }
}

/// String literal.
///
/// Matches the regex `[0-9A-Za-z]{0,3}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StringLiteral {
    data: [u8; 4],
}

impl StringLiteral {
    /// Creates a new literal from `[u8; 4]` ignoring the last element.
    ///
    /// # Errors
    ///
    /// If the first three chars have an illegal char, the [`IllegalCharError`] will be returned.
    pub fn new_from_array(data: [u8; 4]) -> Result<Self, IllegalCharError> {
        let mut internal_data = [0; 4];
        let mut index = 0;
        while index < Self::MAX_CHAR_LEN {
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
    /// Returns the length of this literal.
    pub fn len(&self) -> usize {
        self.data.iter().position(|&b| b == 0).unwrap()
    }
    /// Checks if this literal is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data[0] == 0
    }
    /// Returns the underlying data.
    ///
    /// The last element of the array is always zero.
    pub fn data(&self) -> [u8; 4] {
        self.data
    }
}

impl Literal for StringLiteral {
    const MAX_CHAR_LEN: usize = 3;
    fn new_from_enumerate(enumerate: &mut Enumerate<Chars>) -> (Self, Option<(usize, char)>) {
        let mut literal = Self { data: [0; 4] };
        let mut index = 0;
        loop {
            let next = enumerate.next();
            match next {
                None => return (literal, None::<(usize, char)>),
                Some(last_item) => {
                    if last_item.1.is_ascii_alphanumeric() {
                        if index < Self::MAX_CHAR_LEN {
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
    fn dumps_to(&self, s: &mut String) {
        if !self.is_empty() {
            s.push_str(str::from_utf8(&self.data[0..self.len()]).unwrap());
        }
    }
    fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        if !self.is_empty() {
            return writer.write_all(&self.data[0..self.len()]);
        }
        Ok(())
    }
}

/// Variable identifier literal.
///
/// Matches the regex `[0-9A-Za-z]{0,3}`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableIdentifierLiteral {
    data: [u8; 4],
}

impl VariableIdentifierLiteral {
    /// Creates a new literal from `[u8; 4]` ignoring the last element.
    ///
    /// # Errors
    ///
    /// If the first three chars have an illegal char, the [`IllegalCharError`] will be returned.
    pub fn new_from_array(data: [u8; 4]) -> Result<Self, IllegalCharError> {
        let mut internal_data = [0; 4];
        let mut index = 0;
        while index < Self::MAX_CHAR_LEN {
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
    /// Returns the length of this literal.
    pub fn len(&self) -> usize {
        self.data.iter().position(|&b| b == 0).unwrap()
    }
    /// Checks if this literal is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data[0] == 0
    }
    /// Returns the underlying data.
    ///
    /// The last element of the array is always zero.
    pub fn data(&self) -> [u8; 4] {
        self.data
    }
}

impl Literal for VariableIdentifierLiteral {
    const MAX_CHAR_LEN: usize = 3;
    fn new_from_enumerate(enumerate: &mut Enumerate<Chars>) -> (Self, Option<(usize, char)>) {
        let mut literal = Self { data: [0; 4] };
        let mut index = 0;
        loop {
            let next = enumerate.next();
            match next {
                None => return (literal, None::<(usize, char)>),
                Some(last_item) => {
                    if last_item.1.is_ascii_alphanumeric() {
                        if index < Self::MAX_CHAR_LEN {
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
    fn dumps_to(&self, s: &mut String) {
        if !self.is_empty() {
            s.push_str(str::from_utf8(&self.data[0..self.len()]).unwrap());
        }
    }
    fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        if !self.is_empty() {
            return writer.write_all(&self.data[0..self.len()]);
        }
        Ok(())
    }
}

/// Variable value literal.
///
/// The value can be in the range `[-9_999, 99_999]`. Default value is `0`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct VariableValueLiteral {
    data: i32,
}

impl VariableValueLiteral {
    /// Creates a new literal from `i32`.
    ///
    /// # Errors
    ///
    /// If the given value isn't in the range `[-9_999, 99_999]`, the [`IllegalVariableValueError`] will be returned.
    pub fn new_from_value(val: i32) -> Result<Self, IllegalVariableValueError> {
        match val {
            -9_999..=99_999 => Ok(Self { data: val }),
            _ => Err(IllegalVariableValueError {}),
        }
    }
    /// Returns the value.
    pub fn data(&self) -> i32 {
        self.data
    }
}

impl Literal for VariableValueLiteral {
    const MAX_CHAR_LEN: usize = 5;
    fn new_from_enumerate(enumerate: &mut Enumerate<Chars>) -> (Self, Option<(usize, char)>) {
        let mut buf = [0; Self::MAX_CHAR_LEN];
        let mut is_positive = true;
        let mut next_char = None;
        let mut index = 0;
        let mut buf_index = 0;
        // read to buf
        loop {
            let next = enumerate.next();
            match next {
                Some(last_item) => {
                    if last_item.1.is_ascii_digit() {
                        buf[buf_index] = last_item.1 as u8 - b'0';
                        buf_index += 1;
                    } else if last_item.1 == '-' && index == 0 {
                        is_positive = false;
                    } else {
                        next_char = Some(last_item);
                        break;
                    }
                    index += 1;
                }
                None => break,
            }
        }
        // parse buf
        let mut value = 0;
        let mut pow = 0;
        while buf_index > 0 {
            value += i32::from(buf[buf_index - 1]) * 10_i32.pow(pow);
            pow += 1;
            buf_index -= 1;
        }
        (
            Self {
                data: if is_positive { value } else { -value },
            },
            next_char,
        )
    }
    fn dumps_to(&self, s: &mut String) {
        s.push_str(self.data.to_string().as_str());
    }
    fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(self.data.to_string().as_bytes())
    }
}

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod label_identifier_literal {

        use super::super::{LabelIdentifierLiteral, Literal};

        #[test]
        fn new_from_string() {
            let s = "012:";
            let last_char = Some((3, ':'));
            let expected_literal = (
                LabelIdentifierLiteral::new_from_array([b'0', b'1', b'2', 0]).unwrap(),
                last_char,
            );
            let actual_literal =
                LabelIdentifierLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_one_chars() {
            let s = "0:";
            let last_char = Some((1, ':'));
            let expected_literal = (
                LabelIdentifierLiteral::new_from_array([b'0', 0, 0, 0]).unwrap(),
                last_char,
            );
            let actual_literal =
                LabelIdentifierLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_empty() {
            let s = "";
            let last_char = None;
            let expected_literal = (
                LabelIdentifierLiteral::new_from_array([0, 0, 0, 0]).unwrap(),
                last_char,
            );
            let actual_literal =
                LabelIdentifierLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }
    }

    #[cfg(test)]
    mod variable_value_literal {

        use super::super::{Literal, VariableValueLiteral};

        #[test]
        fn new_from_enumerate_empty_string() {
            let s = "";
            let last_char = None;
            let expected_literal = (VariableValueLiteral::new_from_value(0).unwrap(), last_char);
            let actual_literal =
                VariableValueLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_enumerate_only_last_char() {
            let s = ")";
            let last_char = Some((0, ')'));
            let expected_literal = (VariableValueLiteral::new_from_value(0).unwrap(), last_char);
            let actual_literal =
                VariableValueLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_enumerate_minus_one() {
            let s = "-1)";
            let last_char = Some((2, ')'));
            let expected_literal = (VariableValueLiteral::new_from_value(-1).unwrap(), last_char);
            let actual_literal =
                VariableValueLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_enumerate_min() {
            let s = "-9999)";
            let last_char = Some((5, ')'));
            let expected_literal = (
                VariableValueLiteral::new_from_value(-9_999).unwrap(),
                last_char,
            );
            let actual_literal =
                VariableValueLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_enumerate_max() {
            let s = "99999)";
            let last_char = Some((5, ')'));
            let expected_literal = (
                VariableValueLiteral::new_from_value(99_999).unwrap(),
                last_char,
            );
            let actual_literal =
                VariableValueLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }

        #[test]
        fn new_from_enumerate_max_and_no_last_item() {
            let s = "99999";
            let last_char = None;
            let expected_literal = (
                VariableValueLiteral::new_from_value(99_999).unwrap(),
                last_char,
            );
            let actual_literal =
                VariableValueLiteral::new_from_enumerate(&mut s.chars().enumerate());
            assert_eq!(expected_literal, actual_literal);
        }
    }
}

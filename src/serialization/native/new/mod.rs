//! Serializer and deserializer for New Text format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::{error::Error, fmt, iter::Enumerate, str::Chars};

use crate::formats::internal::{
    Instruction, InstructionId, InstructionPosition, InstructionPositionOverflowError, Program,
};

// region: errors

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MagicNotFoundError {}

impl MagicNotFoundError {
    const DETAILS: &'static str = "the magic was not found: found EOF";
}

impl fmt::Display for MagicNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::DETAILS)
    }
}

impl Error for MagicNotFoundError {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct IllegalMagicError {
    illegal_magic: char,
}

impl IllegalMagicError {
    /// Returns the contained illegal magic.
    pub fn illegal_magic(&self) -> char {
        self.illegal_magic
    }
}

impl fmt::Display for IllegalMagicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an illegal magic was found \"{}\"", self.illegal_magic)
    }
}

impl Error for IllegalMagicError {}

#[derive(Copy, Clone, Debug)]
enum CheckMagicErrors {
    MagicNotFoundError(MagicNotFoundError),
    IllegalMagicError(IllegalMagicError),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct UnknownInstruction {
    index: usize,
}

impl UnknownInstruction {}

impl fmt::Display for UnknownInstruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "found an unknown instruction at index {}", self.index)
    }
}

impl Error for UnknownInstruction {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeserializeErrors {
    MagicNotFoundError(MagicNotFoundError),
    IllegalMagicError(IllegalMagicError),
    UnknownInstruction(UnknownInstruction),
    InstructionPositionOverflowError(InstructionPositionOverflowError),
}

impl From<CheckMagicErrors> for DeserializeErrors {
    fn from(error: CheckMagicErrors) -> Self {
        match error {
            CheckMagicErrors::MagicNotFoundError(e) => DeserializeErrors::MagicNotFoundError(e),
            CheckMagicErrors::IllegalMagicError(e) => DeserializeErrors::IllegalMagicError(e),
        }
    }
}

// endregion: errors

/// A structure that deserializes New Text format into [Internal format](crate::formats::internal).
#[derive(Debug)]
pub struct TextFormatDeserializer<'p, 'e> {
    enumeration: Enumerate<Chars<'e>>,
    position: InstructionPosition,
    program: &'p mut Program,
    index: usize,
}

impl<'p, 'e> TextFormatDeserializer<'p, 'e> {
    /// Creates a new [`TextFormatDeserializer`] from a `&str`.
    pub fn new_from_str(program: &'p mut Program, s: &'e str) -> Self {
        Self {
            enumeration: s.chars().enumerate(),
            position: InstructionPosition::default(),
            program,
            index: 0,
        }
    }
    pub fn deserialize(&mut self) -> Result<(), DeserializeErrors> {
        let magic = self.check_magic();
        if let Err(e) = magic {
            Err(e.into())
        } else {
            self.program.reset();
            loop {
                let parsed = self.parse_next();
                match parsed {
                    Ok(None) => break Ok(()),
                    Ok(Some(instruction)) => {
                        self.program[self.position] = instruction;
                        let result = self.position.move_forward();
                        match result {
                            Err(e) => {
                                break Err(DeserializeErrors::InstructionPositionOverflowError(e))
                            }
                            Ok(_) => continue,
                        }
                    }
                    Err(e) => break Err(DeserializeErrors::UnknownInstruction(e)),
                }
            }
        }
    }
    fn parse_next(&mut self) -> Result<Option<Instruction>, UnknownInstruction> {
        let first_char = self.enumeration.next();
        match first_char {
            None => Ok(None),
            Some((index, first_char)) => {
                self.index = index;
                match first_char {
                    '<' => {
                        let res = self.parse_less_than_sign();
                        match res {
                            Ok(ins) => Ok(Some(ins)),
                            Err(e) => Err(e),
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }
    fn check_magic(&mut self) -> Result<(), CheckMagicErrors> {
        let magic = self.enumeration.next();
        match magic {
            None => Err(CheckMagicErrors::MagicNotFoundError(MagicNotFoundError {})),
            Some((_, magic)) => match magic {
                '$' => Ok(()),
                _ => Err(CheckMagicErrors::IllegalMagicError(IllegalMagicError {
                    illegal_magic: magic,
                })),
            },
        }
    }
    fn get_next_char(&mut self) -> Result<char, UnknownInstruction> {
        let res = self.enumeration.next();
        match res {
            None => Err(UnknownInstruction { index: self.index }),
            Some((_, ch)) => Ok(ch),
        }
    }
    fn parse_less_than_sign(&mut self) -> Result<Instruction, UnknownInstruction> {
        let second_char = self.get_next_char();
        match second_char {
            Err(e) => Err(e),
            Ok(second_char) => match second_char {
                '|' => Ok(Instruction::new_simple(InstructionId::Return).unwrap()),
                '-' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            '|' => Ok(Instruction::new_simple(InstructionId::Return1).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                '=' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            '|' => Ok(Instruction::new_simple(InstructionId::ReturnF).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                _ => Err(UnknownInstruction { index: self.index }),
            },
        }
    }
}

// region: tests

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod text_format_deserializer_tests {
        use super::super::{
            DeserializeErrors, IllegalMagicError, MagicNotFoundError, TextFormatDeserializer,
        };
        use crate::formats::internal::{Instruction, InstructionId, Program};

        #[test]
        fn deserialize_empty_string() {
            let s = "";
            let mut program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut program, s);
            let result = de.deserialize().unwrap_err();
            assert_eq!(
                DeserializeErrors::MagicNotFoundError(MagicNotFoundError {}),
                result,
            );
        }

        #[test]
        fn deserialize_string_with_wrong_magic() {
            let s = "0&&&&&";
            let mut program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut program, s);
            let result = de.deserialize().unwrap_err();
            match result {
                DeserializeErrors::IllegalMagicError(IllegalMagicError { illegal_magic: '0' }) => {}
                _ => panic!(),
            }
        }

        #[test]
        fn deserialize_string_with_returns() {
            // expected_program
            let s = "$<|<-|<=|";
            let mut expected_program = Program::default();
            expected_program[0] = Instruction::new_simple(InstructionId::Return).unwrap();
            expected_program[1] = Instruction::new_simple(InstructionId::Return1).unwrap();
            expected_program[2] = Instruction::new_simple(InstructionId::ReturnF).unwrap();
            // actual_program
            let mut actual_program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, s);
            de.deserialize().unwrap();
            // asserts
            assert_eq!(expected_program, actual_program);
        }
    }
}

// endregion: tests

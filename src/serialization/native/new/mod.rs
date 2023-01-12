//! Serializer and deserializer for New Text format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::{error::Error, fmt, iter::Enumerate, str::Chars};

use crate::formats::internal::literals::{
    LabelIdentifierLiteral, Literal, LiteralType, StringLiteral, VariableIdentifierLiteral,
    VariableValueLiteral,
};
use crate::formats::internal::{
    Instruction, InstructionKind, InstructionPosition, InstructionPositionOverflowError, Program,
};

mod data;
use data::{Node, DATA};

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

impl From<MagicNotFoundError> for CheckMagicErrors {
    fn from(error: MagicNotFoundError) -> Self {
        CheckMagicErrors::MagicNotFoundError(error)
    }
}

impl From<IllegalMagicError> for CheckMagicErrors {
    fn from(error: IllegalMagicError) -> Self {
        CheckMagicErrors::IllegalMagicError(error)
    }
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
pub struct LiteralIsTooLong {
    literal_index: usize,
}

impl LiteralIsTooLong {}

impl fmt::Display for LiteralIsTooLong {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "found too long literal at index {}", self.literal_index)
    }
}

impl Error for LiteralIsTooLong {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum ParseNextErrors {
    UnknownInstruction(UnknownInstruction),
    LiteralIsTooLong(LiteralIsTooLong),
}

impl From<UnknownInstruction> for ParseNextErrors {
    fn from(error: UnknownInstruction) -> Self {
        ParseNextErrors::UnknownInstruction(error)
    }
}

impl From<LiteralIsTooLong> for ParseNextErrors {
    fn from(error: LiteralIsTooLong) -> Self {
        ParseNextErrors::LiteralIsTooLong(error)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeserializeErrors {
    MagicNotFoundError(MagicNotFoundError),
    IllegalMagicError(IllegalMagicError),
    UnknownInstruction(UnknownInstruction),
    LiteralIsTooLong(LiteralIsTooLong),
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

impl From<ParseNextErrors> for DeserializeErrors {
    fn from(error: ParseNextErrors) -> Self {
        match error {
            ParseNextErrors::UnknownInstruction(e) => DeserializeErrors::UnknownInstruction(e),
            ParseNextErrors::LiteralIsTooLong(e) => DeserializeErrors::LiteralIsTooLong(e),
        }
    }
}

impl From<InstructionPositionOverflowError> for DeserializeErrors {
    fn from(error: InstructionPositionOverflowError) -> Self {
        DeserializeErrors::InstructionPositionOverflowError(error)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Command {
    OneStepForward,
    ThreeStepsForward,
    GoToNextRow,
    GoToNextPage,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum InstructionOrCommand {
    Command(Command),
    Instruction(Instruction),
}

impl From<Command> for Option<InstructionOrCommand> {
    fn from(command: Command) -> Self {
        Some(InstructionOrCommand::Command(command))
    }
}

impl From<Instruction> for Option<InstructionOrCommand> {
    fn from(instruction: Instruction) -> Self {
        Some(InstructionOrCommand::Instruction(instruction))
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
        self.check_magic()?;
        self.program.reset();
        loop {
            match self.parse_next()? {
                None => break Ok(()),
                Some(InstructionOrCommand::Instruction(instruction)) => {
                    self.program[self.position] = instruction;
                    self.position.move_forward()?;
                    continue;
                }
                Some(InstructionOrCommand::Command(command)) => match command {
                    Command::OneStepForward => {
                        self.position.move_forward()?;
                        continue;
                    }
                    Command::ThreeStepsForward => {
                        self.position.move_three_steps_forward()?;
                        continue;
                    }
                    Command::GoToNextRow => {
                        self.position.move_to_next_row()?;
                        continue;
                    }
                    Command::GoToNextPage => {
                        self.position.move_to_next_page()?;
                        continue;
                    }
                },
            }
        }
    }
    fn parse_next(&mut self) -> Result<Option<InstructionOrCommand>, ParseNextErrors> {
        let first_char = self.enumeration.next();
        match first_char {
            None => Ok(None),
            Some((index, first_char)) => {
                self.index = index;

                // "registers"
                let mut label: Option<LabelIdentifierLiteral> = None;
                let mut string: Option<StringLiteral> = None;
                let mut name: Option<VariableIdentifierLiteral> = None;
                let mut value: Option<VariableValueLiteral> = None;
                let mut next_char: Option<char> = None;

                match &DATA.binary_search_by_key(&first_char, |&(ch, _)| ch) {
                    Err(_) => return Err(UnknownInstruction { index: self.index }.into()),
                    Ok(x) => {
                        let mut the_node = &DATA[*x].1;
                        loop {
                            match the_node {
                                Node::Command(command) => {
                                    return Ok(Some(InstructionOrCommand::Command(*command)))
                                }
                                Node::Id(id) => {
                                    return match id.kind() {
                                        InstructionKind::Simple => {
                                            Ok(Instruction::new_simple(*id).unwrap().into())
                                        }
                                        InstructionKind::Label => {
                                            Ok(Instruction::new_label(*id, label.unwrap())
                                                .unwrap()
                                                .into())
                                        }
                                        InstructionKind::String => {
                                            Ok(Instruction::new_string(*id, string.unwrap())
                                                .unwrap()
                                                .into())
                                        }
                                        InstructionKind::VarCmp => Ok(Instruction::new_var_cmp(
                                            *id,
                                            name.unwrap(),
                                            value.unwrap(),
                                        )
                                        .unwrap()
                                        .into()),
                                    }
                                }
                                Node::Literal((literal, node)) => {
                                    match literal {
                                        LiteralType::LabelIdentifierLiteral => {
                                            let (literal, next_char_) =
                                                LabelIdentifierLiteral::new_from_enumerate(
                                                    &mut self.enumeration,
                                                );
                                            label = Some(literal);
                                            match next_char_ {
                                                None => {}
                                                Some((_, b)) => {
                                                    next_char = Some(b);
                                                }
                                            }
                                        }
                                        LiteralType::StringLiteral => {
                                            let (literal, next_char_) =
                                                StringLiteral::new_from_enumerate(
                                                    &mut self.enumeration,
                                                );
                                            string = Some(literal);
                                            match next_char_ {
                                                None => {}
                                                Some((_, b)) => {
                                                    next_char = Some(b);
                                                }
                                            }
                                        }
                                        LiteralType::VariableIdentifierLiteral => {
                                            let (literal, next_char_) =
                                                VariableIdentifierLiteral::new_from_enumerate(
                                                    &mut self.enumeration,
                                                );
                                            name = Some(literal);
                                            match next_char_ {
                                                None => {}
                                                Some((_, b)) => {
                                                    next_char = Some(b);
                                                }
                                            }
                                        }
                                        LiteralType::VariableValueLiteral => {
                                            let (literal, next_char_) =
                                                VariableValueLiteral::new_from_enumerate(
                                                    &mut self.enumeration,
                                                );
                                            value = Some(literal);
                                            match next_char_ {
                                                None => {}
                                                Some((_, b)) => {
                                                    next_char = Some(b);
                                                }
                                            }
                                        }
                                    }
                                    the_node = &node[0]; // in this case only one node can be
                                    continue;
                                }
                                Node::Chars(current) => {
                                    let next_char = match next_char {
                                        None => self.get_next_char()?,
                                        Some(c) => c,
                                    };
                                    match current.binary_search_by_key(&next_char, |&(ch, _)| ch) {
                                        Err(_) => {
                                            return Err(
                                                UnknownInstruction { index: self.index }.into()
                                            )
                                        }
                                        Ok(x) => {
                                            the_node = &current[x].1;
                                            continue;
                                        }
                                    }
                                }
                            };
                        }
                    }
                }
            }
        }
    }
    fn check_magic(&mut self) -> Result<(), CheckMagicErrors> {
        let magic = self.enumeration.next();
        match magic {
            None => Err(MagicNotFoundError {}.into()),
            Some((_, magic)) => match magic {
                '$' => Ok(()),
                _ => Err(IllegalMagicError {
                    illegal_magic: magic,
                }
                .into()),
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
}

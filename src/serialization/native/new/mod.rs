//! Serializer and deserializer for New Text format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::{error::Error, fmt, iter::Enumerate, str::Chars};

use crate::formats::internal::literals::LabelIdentifierLiteral;
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Command {
    GoToNextRow,
    GoToNextPage,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum InstructionOrCommand {
    Command(Command),
    Instruction(Instruction),
}

impl From<Instruction> for Result<Option<InstructionOrCommand>, ParseNextErrors> {
    fn from(instruction: Instruction) -> Self {
        Ok(Some(InstructionOrCommand::Instruction(instruction)))
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
                    Ok(Some(InstructionOrCommand::Instruction(instruction))) => {
                        self.program[self.position] = instruction;
                        let result = self.position.move_forward();
                        match result {
                            Err(e) => {
                                break Err(DeserializeErrors::InstructionPositionOverflowError(e))
                            }
                            Ok(_) => continue,
                        }
                    }
                    Ok(Some(InstructionOrCommand::Command(command))) => match command {
                        Command::GoToNextRow => {
                            let result = self.position.move_to_next_row();
                            match result {
                                Err(e) => {
                                    break Err(DeserializeErrors::InstructionPositionOverflowError(
                                        e,
                                    ))
                                }
                                Ok(_) => continue,
                            }
                        }
                        Command::GoToNextPage => {
                            let result = self.position.move_to_next_page();
                            match result {
                                Err(e) => {
                                    break Err(DeserializeErrors::InstructionPositionOverflowError(
                                        e,
                                    ))
                                }
                                Ok(_) => continue,
                            }
                        }
                    },
                    Err(e) => break Err(e.into()),
                }
            }
        }
    }
    fn parse_next(&mut self) -> Result<Option<InstructionOrCommand>, ParseNextErrors> {
        let first_char = self.enumeration.next();
        match first_char {
            None => Ok(None),
            Some((index, first_char)) => {
                self.index = index;
                match first_char {
                    // Commands
                    '\n' => Ok(Some(InstructionOrCommand::Command(Command::GoToNextRow))),
                    '~' => Ok(Some(InstructionOrCommand::Command(Command::GoToNextPage))),
                    // Instructions
                    '!' => {
                        let second_char = self.get_next_char();
                        match second_char {
                            Err(e) => Err(e.into()),
                            Ok(second_char) => match second_char {
                                '?' => {
                                    let literal = LabelIdentifierLiteral::new_from_enumerate(
                                        &mut self.enumeration,
                                    );
                                    match literal {
                                        (literal, Some((next_index, '<'))) => {
                                            if next_index > self.index + 2 + 3 {
                                                Err(ParseNextErrors::LiteralIsTooLong(
                                                    LiteralIsTooLong {
                                                        literal_index: self.index + 2,
                                                    },
                                                ))
                                            } else {
                                                Ok(Some(InstructionOrCommand::Instruction(
                                                    Instruction::new_label(
                                                        InstructionId::IfGoTo,
                                                        literal,
                                                    )
                                                    .unwrap(),
                                                )))
                                            }
                                        }
                                        (_, Some((_, _)) | None) => {
                                            Err(ParseNextErrors::UnknownInstruction(
                                                UnknownInstruction { index: self.index },
                                            ))
                                        }
                                    }
                                }
                                _ => Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                })),
                            },
                        }
                    }
                    '#' => {
                        let second_char = self.get_next_char();
                        match second_char {
                            Err(e) => Err(e.into()),
                            Ok(second_char) => match second_char {
                                'E' => Ok(Some(InstructionOrCommand::Instruction(
                                    Instruction::new_simple(InstructionId::End).unwrap(),
                                ))),
                                'R' => {
                                    let literal = LabelIdentifierLiteral::new_from_enumerate(
                                        &mut self.enumeration,
                                    );
                                    match literal {
                                        (literal, Some((next_index, '<'))) => {
                                            if next_index > self.index + 2 + 3 {
                                                Err(ParseNextErrors::LiteralIsTooLong(
                                                    LiteralIsTooLong {
                                                        literal_index: self.index + 2,
                                                    },
                                                ))
                                            } else {
                                                Ok(Some(InstructionOrCommand::Instruction(
                                                    Instruction::new_label(
                                                        InstructionId::OnResp,
                                                        literal,
                                                    )
                                                    .unwrap(),
                                                )))
                                            }
                                        }
                                        (_, Some((_, _)) | None) => {
                                            Err(ParseNextErrors::UnknownInstruction(
                                                UnknownInstruction { index: self.index },
                                            ))
                                        }
                                    }
                                }
                                'S' => Ok(Some(InstructionOrCommand::Instruction(
                                    Instruction::new_simple(InstructionId::Start).unwrap(),
                                ))),
                                _ => Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                })),
                            },
                        }
                    }
                    ',' => Instruction::new_simple(InstructionId::Back).unwrap().into(),
                    '-' => {
                        let second_char = self.get_next_char();
                        match second_char {
                            Err(e) => Err(e.into()),
                            Ok(second_char) => match second_char {
                                '>' => {
                                    let literal = LabelIdentifierLiteral::new_from_enumerate(
                                        &mut self.enumeration,
                                    );
                                    match literal {
                                        (literal, Some((next_index, '>'))) => {
                                            if next_index > self.index + 2 + 3 {
                                                Err(ParseNextErrors::LiteralIsTooLong(
                                                    LiteralIsTooLong {
                                                        literal_index: self.index + 2,
                                                    },
                                                ))
                                            } else {
                                                Ok(Some(InstructionOrCommand::Instruction(
                                                    Instruction::new_label(
                                                        InstructionId::GoSub1,
                                                        literal,
                                                    )
                                                    .unwrap(),
                                                )))
                                            }
                                        }
                                        (_, Some((_, _)) | None) => {
                                            Err(ParseNextErrors::UnknownInstruction(
                                                UnknownInstruction { index: self.index },
                                            ))
                                        }
                                    }
                                }
                                _ => Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                })),
                            },
                        }
                    }
                    ':' => {
                        let second_char = self.get_next_char();
                        match second_char {
                            Err(e) => Err(e.into()),
                            Ok(second_char) => match second_char {
                                '>' => {
                                    let literal = LabelIdentifierLiteral::new_from_enumerate(
                                        &mut self.enumeration,
                                    );
                                    match literal {
                                        (literal, Some((next_index, '>'))) => {
                                            if next_index > self.index + 2 + 3 {
                                                Err(ParseNextErrors::LiteralIsTooLong(
                                                    LiteralIsTooLong {
                                                        literal_index: self.index + 2,
                                                    },
                                                ))
                                            } else {
                                                Ok(Some(InstructionOrCommand::Instruction(
                                                    Instruction::new_label(
                                                        InstructionId::GoSub,
                                                        literal,
                                                    )
                                                    .unwrap(),
                                                )))
                                            }
                                        }
                                        (_, Some((_, _)) | None) => {
                                            Err(ParseNextErrors::UnknownInstruction(
                                                UnknownInstruction { index: self.index },
                                            ))
                                        }
                                    }
                                }
                                _ => Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                })),
                            },
                        }
                    }
                    '<' => {
                        let res = self.parse_less_than_sign();
                        match res {
                            Ok(ins) => Ok(Some(InstructionOrCommand::Instruction(ins))),
                            Err(e) => Err(ParseNextErrors::UnknownInstruction(e)),
                        }
                    }
                    '=' => {
                        let res = self.parse_equals_sign();
                        match res {
                            Ok(ins) => Ok(Some(InstructionOrCommand::Instruction(ins))),
                            Err(e) => Err(e),
                        }
                    }
                    '>' => {
                        let literal =
                            LabelIdentifierLiteral::new_from_enumerate(&mut self.enumeration);
                        match literal {
                            (literal, Some((next_index, '|'))) => {
                                if next_index > self.index + 1 + 3 {
                                    Err(ParseNextErrors::LiteralIsTooLong(LiteralIsTooLong {
                                        literal_index: self.index + 1,
                                    }))
                                } else {
                                    Ok(Some(InstructionOrCommand::Instruction(
                                        Instruction::new_label(InstructionId::GoTo, literal)
                                            .unwrap(),
                                    )))
                                }
                            }
                            (_, Some((_, _)) | None) => {
                                Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                }))
                            }
                        }
                    }
                    '?' => {
                        let literal =
                            LabelIdentifierLiteral::new_from_enumerate(&mut self.enumeration);
                        match literal {
                            (literal, Some((next_index, '<'))) => {
                                if next_index > self.index + 1 + 3 {
                                    Err(ParseNextErrors::LiteralIsTooLong(LiteralIsTooLong {
                                        literal_index: self.index + 1,
                                    }))
                                } else {
                                    Ok(Some(InstructionOrCommand::Instruction(
                                        Instruction::new_label(InstructionId::IfNotGoTo, literal)
                                            .unwrap(),
                                    )))
                                }
                            }
                            (_, Some((_, _)) | None) => {
                                Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                }))
                            }
                        }
                    }
                    '[' => {
                        let res = self.parse_left_square_bracket();
                        match res {
                            Ok(ins) => Ok(Some(InstructionOrCommand::Instruction(ins))),
                            Err(e) => Err(ParseNextErrors::UnknownInstruction(e)),
                        }
                    }
                    '^' => {
                        let res = self.parse_circumflex_accent();
                        match res {
                            Ok(ins) => Ok(Some(InstructionOrCommand::Instruction(ins))),
                            Err(e) => Err(ParseNextErrors::UnknownInstruction(e)),
                        }
                    }
                    'a' => Instruction::new_simple(InstructionId::LookA)
                        .unwrap()
                        .into(),
                    'b' => Instruction::new_simple(InstructionId::ActionBuild)
                        .unwrap()
                        .into(),
                    'd' => Instruction::new_simple(InstructionId::LookD)
                        .unwrap()
                        .into(),
                    'g' => Instruction::new_simple(InstructionId::ActionGeo)
                        .unwrap()
                        .into(),
                    'h' => Instruction::new_simple(InstructionId::ActionHeal)
                        .unwrap()
                        .into(),
                    'q' => Instruction::new_simple(InstructionId::ActionQuadro)
                        .unwrap()
                        .into(),
                    'r' => Instruction::new_simple(InstructionId::ActionRoad)
                        .unwrap()
                        .into(),
                    's' => Instruction::new_simple(InstructionId::LookS)
                        .unwrap()
                        .into(),
                    'w' => Instruction::new_simple(InstructionId::LookW)
                        .unwrap()
                        .into(),
                    'z' => Instruction::new_simple(InstructionId::Digg).unwrap().into(),
                    '|' => {
                        let literal =
                            LabelIdentifierLiteral::new_from_enumerate(&mut self.enumeration);
                        match literal {
                            (literal, Some((next_index, ':'))) => {
                                if next_index > self.index + 1 + 3 {
                                    Err(ParseNextErrors::LiteralIsTooLong(LiteralIsTooLong {
                                        literal_index: self.index + 1,
                                    }))
                                } else {
                                    Ok(Some(InstructionOrCommand::Instruction(
                                        Instruction::new_label(InstructionId::Label, literal)
                                            .unwrap(),
                                    )))
                                }
                            }
                            (_, Some((_, _)) | None) => {
                                Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                    index: self.index,
                                }))
                            }
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
    fn parse_equals_sign(&mut self) -> Result<Instruction, ParseNextErrors> {
        let second_char = self.get_next_char();
        match second_char {
            Err(e) => Err(e.into()),
            Ok(second_char) => match second_char {
                '>' => {
                    let literal = LabelIdentifierLiteral::new_from_enumerate(&mut self.enumeration);
                    match literal {
                        (literal, Some((next_index, '>'))) => {
                            if next_index > self.index + 2 + 3 {
                                Err(ParseNextErrors::LiteralIsTooLong(LiteralIsTooLong {
                                    literal_index: self.index + 1,
                                }))
                            } else {
                                Ok(Instruction::new_label(InstructionId::GoSubF, literal).unwrap())
                            }
                        }
                        (_, Some((_, _)) | None) => {
                            Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                index: self.index,
                            }))
                        }
                    }
                }
                'A' => Ok(Instruction::new_simple(InstructionId::CcAcid).unwrap()),
                'B' => Ok(Instruction::new_simple(InstructionId::CccBlackRock).unwrap()),
                'G' => Ok(Instruction::new_simple(InstructionId::CcGun).unwrap()),
                'K' => Ok(Instruction::new_simple(InstructionId::CccRedRock).unwrap()),
                'R' => Ok(Instruction::new_simple(InstructionId::CccRoad).unwrap()),
                'a' => Ok(Instruction::new_simple(InstructionId::CcAlive).unwrap()),
                'b' => Ok(Instruction::new_simple(InstructionId::CcBolder).unwrap()),
                'c' => Ok(Instruction::new_simple(InstructionId::CcCrystall).unwrap()),
                'd' => Ok(Instruction::new_simple(InstructionId::CcDead).unwrap()),
                'e' => Ok(Instruction::new_simple(InstructionId::CcEmpty).unwrap()),
                'f' => Ok(Instruction::new_simple(InstructionId::CcGravity).unwrap()),
                'g' => Ok(Instruction::new_simple(InstructionId::CccGreenBlock).unwrap()),
                'h' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e.into()),
                        Ok(third_char) => match third_char {
                            'p' => {
                                let fourth_char = self.get_next_char();
                                match fourth_char {
                                    Err(e) => Err(e.into()),
                                    Ok(fourth_char) => match fourth_char {
                                        '-' => {
                                            Ok(Instruction::new_simple(InstructionId::CbHp)
                                                .unwrap())
                                        }
                                        '5' => {
                                            let fifth_char = self.get_next_char();
                                            match fifth_char {
                                                Err(e) => Err(e.into()),
                                                Ok(fifth_char) => match fifth_char {
                                                    '0' => Ok(Instruction::new_simple(
                                                        InstructionId::CbHp50,
                                                    )
                                                    .unwrap()),
                                                    _ => Err(ParseNextErrors::UnknownInstruction(
                                                        UnknownInstruction { index: self.index },
                                                    )),
                                                },
                                            }
                                        }
                                        _ => Err(ParseNextErrors::UnknownInstruction(
                                            UnknownInstruction { index: self.index },
                                        )),
                                    },
                                }
                            }
                            _ => Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                                index: self.index,
                            })),
                        },
                    }
                }
                'k' => Ok(Instruction::new_simple(InstructionId::CcRock).unwrap()),
                'n' => Ok(Instruction::new_simple(InstructionId::CcNotEmpty).unwrap()),
                'o' => Ok(Instruction::new_simple(InstructionId::CccOpor).unwrap()),
                'q' => Ok(Instruction::new_simple(InstructionId::CccQuadro).unwrap()),
                'r' => Ok(Instruction::new_simple(InstructionId::CccRedBlock).unwrap()),
                's' => Ok(Instruction::new_simple(InstructionId::CcSand).unwrap()),
                'x' => Ok(Instruction::new_simple(InstructionId::CccBox).unwrap()),
                'y' => Ok(Instruction::new_simple(InstructionId::CccYellowBlock).unwrap()),
                _ => Err(ParseNextErrors::UnknownInstruction(UnknownInstruction {
                    index: self.index,
                })),
            },
        }
    }
    fn parse_left_square_bracket(&mut self) -> Result<Instruction, UnknownInstruction> {
        let second_char = self.get_next_char();
        match second_char {
            Err(e) => Err(e),
            Ok(second_char) => match second_char {
                'A' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            'S' => {
                                let fourth_char = self.get_next_char();
                                match fourth_char {
                                    Err(e) => Err(e),
                                    Ok(fourth_char) => match fourth_char {
                                        ']' => {
                                            Ok(Instruction::new_simple(InstructionId::CellAs)
                                                .unwrap())
                                        }
                                        _ => Err(UnknownInstruction { index: self.index }),
                                    },
                                }
                            }
                            ']' => Ok(Instruction::new_simple(InstructionId::CellA).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'D' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            'W' => {
                                let fourth_char = self.get_next_char();
                                match fourth_char {
                                    Err(e) => Err(e),
                                    Ok(fourth_char) => match fourth_char {
                                        ']' => {
                                            Ok(Instruction::new_simple(InstructionId::CellDw)
                                                .unwrap())
                                        }
                                        _ => Err(UnknownInstruction { index: self.index }),
                                    },
                                }
                            }
                            ']' => Ok(Instruction::new_simple(InstructionId::CellD).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'F' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => Ok(Instruction::new_simple(InstructionId::CellF).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'S' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            'D' => {
                                let fourth_char = self.get_next_char();
                                match fourth_char {
                                    Err(e) => Err(e),
                                    Ok(fourth_char) => match fourth_char {
                                        ']' => {
                                            Ok(Instruction::new_simple(InstructionId::CellSd)
                                                .unwrap())
                                        }
                                        _ => Err(UnknownInstruction { index: self.index }),
                                    },
                                }
                            }
                            ']' => Ok(Instruction::new_simple(InstructionId::CellS).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'W' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            'A' => {
                                let fourth_char = self.get_next_char();
                                match fourth_char {
                                    Err(e) => Err(e),
                                    Ok(fourth_char) => match fourth_char {
                                        ']' => {
                                            Ok(Instruction::new_simple(InstructionId::CellWa)
                                                .unwrap())
                                        }
                                        _ => Err(UnknownInstruction { index: self.index }),
                                    },
                                }
                            }
                            ']' => Ok(Instruction::new_simple(InstructionId::CellW).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'a' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => Ok(Instruction::new_simple(InstructionId::CellAa).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'd' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => Ok(Instruction::new_simple(InstructionId::CellDd).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'f' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => Ok(Instruction::new_simple(InstructionId::CellFf).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'l' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => {
                                Ok(Instruction::new_simple(InstructionId::CellLeftHand).unwrap())
                            }
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'r' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => {
                                Ok(Instruction::new_simple(InstructionId::CellRightHand).unwrap())
                            }
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                's' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => Ok(Instruction::new_simple(InstructionId::CellSs).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                'w' => {
                    let third_char = self.get_next_char();
                    match third_char {
                        Err(e) => Err(e),
                        Ok(third_char) => match third_char {
                            ']' => Ok(Instruction::new_simple(InstructionId::CellWw).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                    }
                }
                _ => Err(UnknownInstruction { index: self.index }),
            },
        }
    }
    fn parse_circumflex_accent(&mut self) -> Result<Instruction, UnknownInstruction> {
        let second_char = self.get_next_char();
        match second_char {
            Err(e) => Err(e),
            Ok(second_char) => match second_char {
                'F' => Ok(Instruction::new_simple(InstructionId::MoveF).unwrap()),
                'W' => Ok(Instruction::new_simple(InstructionId::MoveW).unwrap()),
                'D' => Ok(Instruction::new_simple(InstructionId::MoveD).unwrap()),
                'S' => Ok(Instruction::new_simple(InstructionId::MoveS).unwrap()),
                'A' => Ok(Instruction::new_simple(InstructionId::MoveA).unwrap()),
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

        use crate::formats::internal::literals::LabelIdentifierLiteral;
        use crate::formats::internal::{Instruction, InstructionId, InstructionPosition, Program};

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
        fn deserialize_simple_instructions() {
            let s = "$<|<-|<=|^F^W^D^S^Aadswzghrbq,[F][W][WA][D][DW][S][SD][A][AS][r][l][f][w][d][s][a]=G=n=e=f=c=a=b=s=k=d=A=B=K=g=y=r=o=q=x=R=hp50=hp-#S#E<|";
            // expected_program
            let mut expected_program = Program::default();
            //     returns
            expected_program[0] = Instruction::new_simple(InstructionId::Return).unwrap();
            expected_program[1] = Instruction::new_simple(InstructionId::Return1).unwrap();
            expected_program[2] = Instruction::new_simple(InstructionId::ReturnF).unwrap();
            //     moves
            expected_program[3] = Instruction::new_simple(InstructionId::MoveF).unwrap();
            expected_program[4] = Instruction::new_simple(InstructionId::MoveW).unwrap();
            expected_program[5] = Instruction::new_simple(InstructionId::MoveD).unwrap();
            expected_program[6] = Instruction::new_simple(InstructionId::MoveS).unwrap();
            expected_program[7] = Instruction::new_simple(InstructionId::MoveA).unwrap();
            //     looks
            expected_program[8] = Instruction::new_simple(InstructionId::LookA).unwrap();
            expected_program[9] = Instruction::new_simple(InstructionId::LookD).unwrap();
            expected_program[10] = Instruction::new_simple(InstructionId::LookS).unwrap();
            expected_program[11] = Instruction::new_simple(InstructionId::LookW).unwrap();
            //     one char staff
            expected_program[12] = Instruction::new_simple(InstructionId::Digg).unwrap();
            expected_program[13] = Instruction::new_simple(InstructionId::ActionGeo).unwrap();
            expected_program[14] = Instruction::new_simple(InstructionId::ActionHeal).unwrap();
            expected_program[15] = Instruction::new_simple(InstructionId::ActionRoad).unwrap();
            expected_program[16] = Instruction::new_simple(InstructionId::ActionBuild).unwrap();
            expected_program[17] = Instruction::new_simple(InstructionId::ActionQuadro).unwrap();
            expected_program[18] = Instruction::new_simple(InstructionId::Back).unwrap();
            //     cells
            expected_program[19] = Instruction::new_simple(InstructionId::CellF).unwrap();
            expected_program[20] = Instruction::new_simple(InstructionId::CellW).unwrap();
            expected_program[21] = Instruction::new_simple(InstructionId::CellWa).unwrap();
            expected_program[22] = Instruction::new_simple(InstructionId::CellD).unwrap();
            expected_program[23] = Instruction::new_simple(InstructionId::CellDw).unwrap();
            expected_program[24] = Instruction::new_simple(InstructionId::CellS).unwrap();
            expected_program[25] = Instruction::new_simple(InstructionId::CellSd).unwrap();
            expected_program[26] = Instruction::new_simple(InstructionId::CellA).unwrap();
            expected_program[27] = Instruction::new_simple(InstructionId::CellAs).unwrap();
            expected_program[28] = Instruction::new_simple(InstructionId::CellRightHand).unwrap();
            expected_program[29] = Instruction::new_simple(InstructionId::CellLeftHand).unwrap();
            expected_program[30] = Instruction::new_simple(InstructionId::CellFf).unwrap();
            expected_program[31] = Instruction::new_simple(InstructionId::CellWw).unwrap();
            expected_program[32] = Instruction::new_simple(InstructionId::CellDd).unwrap();
            expected_program[33] = Instruction::new_simple(InstructionId::CellSs).unwrap();
            expected_program[34] = Instruction::new_simple(InstructionId::CellAa).unwrap();
            //     cc
            expected_program[35] = Instruction::new_simple(InstructionId::CcGun).unwrap();
            expected_program[36] = Instruction::new_simple(InstructionId::CcNotEmpty).unwrap();
            expected_program[37] = Instruction::new_simple(InstructionId::CcEmpty).unwrap();
            expected_program[38] = Instruction::new_simple(InstructionId::CcGravity).unwrap();
            expected_program[39] = Instruction::new_simple(InstructionId::CcCrystall).unwrap();
            expected_program[40] = Instruction::new_simple(InstructionId::CcAlive).unwrap();
            expected_program[41] = Instruction::new_simple(InstructionId::CcBolder).unwrap();
            expected_program[42] = Instruction::new_simple(InstructionId::CcSand).unwrap();
            expected_program[43] = Instruction::new_simple(InstructionId::CcRock).unwrap();
            expected_program[44] = Instruction::new_simple(InstructionId::CcDead).unwrap();
            expected_program[45] = Instruction::new_simple(InstructionId::CcAcid).unwrap();
            //     ccc
            expected_program[46] = Instruction::new_simple(InstructionId::CccBlackRock).unwrap();
            expected_program[47] = Instruction::new_simple(InstructionId::CccRedRock).unwrap();
            expected_program[48] = Instruction::new_simple(InstructionId::CccGreenBlock).unwrap();
            expected_program[49] = Instruction::new_simple(InstructionId::CccYellowBlock).unwrap();
            expected_program[50] = Instruction::new_simple(InstructionId::CccRedBlock).unwrap();
            expected_program[51] = Instruction::new_simple(InstructionId::CccOpor).unwrap();
            expected_program[52] = Instruction::new_simple(InstructionId::CccQuadro).unwrap();
            expected_program[53] = Instruction::new_simple(InstructionId::CccBox).unwrap();
            expected_program[54] = Instruction::new_simple(InstructionId::CccRoad).unwrap();
            //     cb_hp
            expected_program[55] = Instruction::new_simple(InstructionId::CbHp50).unwrap();
            expected_program[56] = Instruction::new_simple(InstructionId::CbHp).unwrap();
            //     start & end
            expected_program[57] = Instruction::new_simple(InstructionId::Start).unwrap();
            expected_program[58] = Instruction::new_simple(InstructionId::End).unwrap();
            //     for tail check
            expected_program[59] = Instruction::new_simple(InstructionId::Return).unwrap();
            // actual_program
            let mut actual_program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, s);
            de.deserialize().unwrap();
            // asserts
            assert_eq!(expected_program, actual_program);
        }

        #[test]
        fn deserialize_commands() {
            let s = "$^W\n^A~^D";
            // expected_program
            let mut expected_program = Program::default();
            expected_program[InstructionPosition::new(0, 0, 0).unwrap()] =
                Instruction::new_simple(InstructionId::MoveW).unwrap();
            expected_program[InstructionPosition::new(0, 1, 0).unwrap()] =
                Instruction::new_simple(InstructionId::MoveA).unwrap();
            expected_program[InstructionPosition::new(1, 0, 0).unwrap()] =
                Instruction::new_simple(InstructionId::MoveD).unwrap();
            // actual_program
            let mut actual_program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, s);
            de.deserialize().unwrap();
            // asserts
            assert_eq!(expected_program, actual_program);
        }

        #[test]
        fn deserialize_literals() {
            let s = "$|:|hi:|012:>abc|:>zxc>->s12>=>sbf>!?if<?ifn<#Rrsp<";
            // expected_program
            let mut expected_program = Program::default();
            //     labels
            //         label
            expected_program[0] = Instruction::new_label(
                InstructionId::Label,
                LabelIdentifierLiteral::new_from_array([0; 4]).unwrap(),
            )
            .unwrap();
            expected_program[1] = Instruction::new_label(
                InstructionId::Label,
                LabelIdentifierLiteral::new_from_array([b'h', b'i', 0, 0]).unwrap(),
            )
            .unwrap();
            expected_program[2] = Instruction::new_label(
                InstructionId::Label,
                LabelIdentifierLiteral::new_from_array([b'0', b'1', b'2', 0]).unwrap(),
            )
            .unwrap();
            //         go
            expected_program[3] = Instruction::new_label(
                InstructionId::GoTo,
                LabelIdentifierLiteral::new_from_array([b'a', b'b', b'c', 0]).unwrap(),
            )
            .unwrap();
            expected_program[4] = Instruction::new_label(
                InstructionId::GoSub,
                LabelIdentifierLiteral::new_from_array([b'z', b'x', b'c', 0]).unwrap(),
            )
            .unwrap();
            expected_program[5] = Instruction::new_label(
                InstructionId::GoSub1,
                LabelIdentifierLiteral::new_from_array([b's', b'1', b'2', 0]).unwrap(),
            )
            .unwrap();
            expected_program[6] = Instruction::new_label(
                InstructionId::GoSubF,
                LabelIdentifierLiteral::new_from_array([b's', b'b', b'f', 0]).unwrap(),
            )
            .unwrap();
            //         if[not]goto
            expected_program[7] = Instruction::new_label(
                InstructionId::IfGoTo,
                LabelIdentifierLiteral::new_from_array([b'i', b'f', 0, 0]).unwrap(),
            )
            .unwrap();
            expected_program[8] = Instruction::new_label(
                InstructionId::IfNotGoTo,
                LabelIdentifierLiteral::new_from_array([b'i', b'f', b'n', 0]).unwrap(),
            )
            .unwrap();
            //         resp
            expected_program[9] = Instruction::new_label(
                InstructionId::OnResp,
                LabelIdentifierLiteral::new_from_array([b'r', b's', b'p', 0]).unwrap(),
            )
            .unwrap();
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

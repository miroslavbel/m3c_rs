//! Serializer and deserializer for New Text format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::{error::Error, fmt, iter::Enumerate, str::Chars};

use crate::formats::internal::literals::{
    LabelIdentifierLiteral, Literal, StringLiteral, VariableIdentifierLiteral, VariableValueLiteral,
};
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
                match first_char {
                    // Commands
                    ' ' => Ok(Command::OneStepForward.into()),
                    '_' => Ok(Command::ThreeStepsForward.into()),
                    '\n' => Ok(Command::GoToNextRow.into()),
                    '~' => Ok(Command::GoToNextPage.into()),
                    // Instructions
                    '!' => match self.get_next_char()? {
                        '?' => {
                            let (literal, _) =
                                self.parse_literal::<LabelIdentifierLiteral>(&['<'], 2)?;
                            Ok(Instruction::new_label(InstructionId::IfGoTo, literal)
                                .unwrap()
                                .into())
                        }
                        '{' => {
                            let (literal, _) = self.parse_literal::<StringLiteral>(&['}'], 2)?;
                            Ok(Instruction::new_string(InstructionId::DebugBreak, literal)
                                .unwrap()
                                .into())
                        }
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    '#' => match self.get_next_char()? {
                        'E' => Ok(Instruction::new_simple(InstructionId::End).unwrap().into()),
                        'R' => {
                            let (literal, _) =
                                self.parse_literal::<LabelIdentifierLiteral>(&['<'], 2)?;
                            Ok(Instruction::new_label(InstructionId::OnResp, literal)
                                .unwrap()
                                .into())
                        }
                        'S' => Ok(Instruction::new_simple(InstructionId::Start)
                            .unwrap()
                            .into()),
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    '(' => Ok(self.parse_left_parenthesis()?.into()),
                    ',' => Ok(Instruction::new_simple(InstructionId::Back).unwrap().into()),
                    '-' => match self.get_next_char()? {
                        '>' => {
                            let (literal, _) =
                                self.parse_literal::<LabelIdentifierLiteral>(&['>'], 2)?;
                            Ok(Instruction::new_label(InstructionId::GoSub1, literal)
                                .unwrap()
                                .into())
                        }
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    ':' => match self.get_next_char()? {
                        '>' => {
                            let (literal, _) =
                                self.parse_literal::<LabelIdentifierLiteral>(&['>'], 2)?;
                            Ok(Instruction::new_label(InstructionId::GoSub, literal)
                                .unwrap()
                                .into())
                        }
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    '<' => Ok(self.parse_less_than_sign()?.into()),
                    '=' => Ok(self.parse_equals_sign()?.into()),
                    '>' => {
                        let (literal, _) =
                            self.parse_literal::<LabelIdentifierLiteral>(&['|'], 1)?;
                        Ok(Instruction::new_label(InstructionId::GoTo, literal)
                            .unwrap()
                            .into())
                    }
                    '?' => {
                        let (literal, _) =
                            self.parse_literal::<LabelIdentifierLiteral>(&['<'], 1)?;
                        Ok(Instruction::new_label(InstructionId::IfNotGoTo, literal)
                            .unwrap()
                            .into())
                    }
                    'A' => Ok(self.parse_latin_capital_letter_a()?.into()),
                    'B' => Ok(self.parse_latin_capital_letter_b()?.into()),
                    'C' => Ok(self.parse_latin_capital_letter_c()?.into()),
                    'D' => Ok(self.parse_latin_capital_letter_d()?.into()),
                    'F' => Ok(self.parse_latin_capital_letter_f()?.into()),
                    'G' => match self.get_next_char()? {
                        'E' => match self.get_next_char()? {
                            'O' => match self.get_next_char()? {
                                ';' => Ok(Instruction::new_simple(InstructionId::ActionGeopack)
                                    .unwrap()
                                    .into()),
                                _ => Err(UnknownInstruction { index: self.index }.into()),
                            },
                            _ => Err(UnknownInstruction { index: self.index }.into()),
                        },
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    'H' => Ok(self.parse_latin_capital_letter_h()?.into()),
                    'M' => Ok(self.parse_latin_capital_letter_m()?.into()),
                    'N' => match self.get_next_char()? {
                        'A' => match self.get_next_char()? {
                            'N' => match self.get_next_char()? {
                                'O' => match self.get_next_char()? {
                                    ';' => Ok(Instruction::new_simple(InstructionId::ActionNano)
                                        .unwrap()
                                        .into()),
                                    _ => Err(UnknownInstruction { index: self.index }.into()),
                                },
                                _ => Err(UnknownInstruction { index: self.index }.into()),
                            },
                            _ => Err(UnknownInstruction { index: self.index }.into()),
                        },
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    'O' => match self.get_next_char()? {
                        'R' => Ok(Instruction::new_simple(InstructionId::BoolModeOr)
                            .unwrap()
                            .into()),
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    'P' => match self.get_next_char()? {
                        'O' => match self.get_next_char()? {
                            'L' => match self.get_next_char()? {
                                'Y' => match self.get_next_char()? {
                                    ';' => Ok(Instruction::new_simple(InstructionId::ActionPoly)
                                        .unwrap()
                                        .into()),
                                    _ => Err(UnknownInstruction { index: self.index }.into()),
                                },
                                _ => Err(UnknownInstruction { index: self.index }.into()),
                            },
                            _ => Err(UnknownInstruction { index: self.index }.into()),
                        },
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    'R' => Ok(self.parse_latin_capital_letter_r()?.into()),
                    'U' => match self.get_next_char()? {
                        'P' => match self.get_next_char()? {
                            ';' => Ok(Instruction::new_simple(InstructionId::ActionUp)
                                .unwrap()
                                .into()),
                            _ => Err(UnknownInstruction { index: self.index }.into()),
                        },
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    'V' => match self.get_next_char()? {
                        'B' => match self.get_next_char()? {
                            ';' => Ok(Instruction::new_simple(InstructionId::ActionWb)
                                .unwrap()
                                .into()),
                            _ => Err(UnknownInstruction { index: self.index }.into()),
                        },
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    'Z' => match self.get_next_char()? {
                        'Z' => match self.get_next_char()? {
                            ';' => Ok(Instruction::new_simple(InstructionId::ActionZm)
                                .unwrap()
                                .into()),
                            _ => Err(UnknownInstruction { index: self.index }.into()),
                        },
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    '[' => Ok(self.parse_left_square_bracket()?.into()),
                    '^' => Ok(self.parse_circumflex_accent()?.into()),
                    'a' => Ok(Instruction::new_simple(InstructionId::LookA)
                        .unwrap()
                        .into()),
                    'b' => Ok(Instruction::new_simple(InstructionId::ActionBuild)
                        .unwrap()
                        .into()),
                    'd' => Ok(Instruction::new_simple(InstructionId::LookD)
                        .unwrap()
                        .into()),
                    'g' => Ok(Instruction::new_simple(InstructionId::ActionGeo)
                        .unwrap()
                        .into()),
                    'h' => Ok(Instruction::new_simple(InstructionId::ActionHeal)
                        .unwrap()
                        .into()),
                    'i' => Ok(self.parse_latin_small_letter_i()?.into()),
                    'q' => Ok(Instruction::new_simple(InstructionId::ActionQuadro)
                        .unwrap()
                        .into()),
                    'r' => Ok(Instruction::new_simple(InstructionId::ActionRoad)
                        .unwrap()
                        .into()),
                    's' => Ok(Instruction::new_simple(InstructionId::LookS)
                        .unwrap()
                        .into()),
                    'w' => Ok(Instruction::new_simple(InstructionId::LookW)
                        .unwrap()
                        .into()),
                    'z' => Ok(Instruction::new_simple(InstructionId::Digg).unwrap().into()),
                    '{' => {
                        let (literal, _) = self.parse_literal::<StringLiteral>(&['}'], 1)?;
                        Ok(Instruction::new_string(InstructionId::DebugSet, literal)
                            .unwrap()
                            .into())
                    }
                    '|' => {
                        let (literal, _) =
                            self.parse_literal::<LabelIdentifierLiteral>(&[':'], 1)?;
                        Ok(Instruction::new_label(InstructionId::Label, literal)
                            .unwrap()
                            .into())
                    }
                    _ => todo!(),
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
    fn parse_literal<L: Literal>(
        &mut self,
        possible_next_chars: &[char],
        offset: usize,
    ) -> Result<(L, char), ParseNextErrors> {
        let literal = Literal::new_from_enumerate(&mut self.enumeration);
        match literal {
            (literal, Some((next_index, next_char)))
                if possible_next_chars.contains(&next_char) =>
            {
                if next_index > self.index + offset + L::MAX_CHAR_LEN {
                    Err(LiteralIsTooLong {
                        literal_index: self.index + offset,
                    }
                    .into())
                } else {
                    Ok((literal, next_char))
                }
            }
            (_, Some((_, _)) | None) => Err(UnknownInstruction { index: self.index }.into()),
        }
    }
    fn parse_left_parenthesis(&mut self) -> Result<Instruction, ParseNextErrors> {
        let (identifier, next_char) =
            self.parse_literal::<VariableIdentifierLiteral>(&['=', '<', '>'], 1)?;
        let (value, _) =
            self.parse_literal::<VariableValueLiteral>(&[')'], 1 + identifier.len() + 1)?;
        match next_char {
            '<' => Ok(Instruction::new_var_cmp(InstructionId::VarLess, identifier, value).unwrap()),
            '=' => {
                Ok(Instruction::new_var_cmp(InstructionId::VarEqual, identifier, value).unwrap())
            }
            '>' => Ok(Instruction::new_var_cmp(InstructionId::VarMore, identifier, value).unwrap()),
            _ => unreachable!(),
        }
    }
    fn parse_less_than_sign(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            '|' => Ok(Instruction::new_simple(InstructionId::Return).unwrap()),
            '-' => match self.get_next_char()? {
                '|' => Ok(Instruction::new_simple(InstructionId::Return1).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            '=' => match self.get_next_char()? {
                '|' => Ok(Instruction::new_simple(InstructionId::ReturnF).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_equals_sign(&mut self) -> Result<Instruction, ParseNextErrors> {
        match self.get_next_char()? {
            '>' => {
                let (literal, _) = self.parse_literal::<LabelIdentifierLiteral>(&['>'], 2)?;
                Ok(Instruction::new_label(InstructionId::GoSubF, literal).unwrap())
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
            'h' => match self.get_next_char()? {
                'p' => match self.get_next_char()? {
                    '-' => Ok(Instruction::new_simple(InstructionId::CbHp).unwrap()),
                    '5' => match self.get_next_char()? {
                        '0' => Ok(Instruction::new_simple(InstructionId::CbHp50).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }.into()),
                    },
                    _ => Err(UnknownInstruction { index: self.index }.into()),
                },
                _ => Err(UnknownInstruction { index: self.index }.into()),
            },
            'k' => Ok(Instruction::new_simple(InstructionId::CcRock).unwrap()),
            'n' => Ok(Instruction::new_simple(InstructionId::CcNotEmpty).unwrap()),
            'o' => Ok(Instruction::new_simple(InstructionId::CccOpor).unwrap()),
            'q' => Ok(Instruction::new_simple(InstructionId::CccQuadro).unwrap()),
            'r' => Ok(Instruction::new_simple(InstructionId::CccRedBlock).unwrap()),
            's' => Ok(Instruction::new_simple(InstructionId::CcSand).unwrap()),
            'x' => Ok(Instruction::new_simple(InstructionId::CccBox).unwrap()),
            'y' => Ok(Instruction::new_simple(InstructionId::CccYellowBlock).unwrap()),
            _ => Err(UnknownInstruction { index: self.index }.into()),
        }
    }
    fn parse_latin_capital_letter_a(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'G' => match self.get_next_char()? {
                'R' => match self.get_next_char()? {
                    '+' => Ok(Instruction::new_simple(InstructionId::ModeAgrOn).unwrap()),
                    '-' => Ok(Instruction::new_simple(InstructionId::ModeAgrOff).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }.into()),
                },
                _ => Err(UnknownInstruction { index: self.index }.into()),
            },
            'N' => match self.get_next_char()? {
                'D' => Ok(Instruction::new_simple(InstructionId::BoolModeAnd).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }.into()),
            },
            'U' => match self.get_next_char()? {
                'T' => match self.get_next_char()? {
                    '+' => Ok(Instruction::new_simple(InstructionId::ModeAutodiggOn).unwrap()),
                    '-' => Ok(Instruction::new_simple(InstructionId::ModeAutodiggOff).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }.into()),
                },
                _ => Err(UnknownInstruction { index: self.index }.into()),
            },
            _ => Err(UnknownInstruction { index: self.index }.into()),
        }
    }
    fn parse_latin_capital_letter_b(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            '1' => match self.get_next_char()? {
                ';' => Ok(Instruction::new_simple(InstructionId::ActionB1).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            '2' => match self.get_next_char()? {
                ';' => Ok(Instruction::new_simple(InstructionId::ActionB3).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            '3' => match self.get_next_char()? {
                ';' => Ok(Instruction::new_simple(InstructionId::ActionB2).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'E' => match self.get_next_char()? {
                'E' => match self.get_next_char()? {
                    'P' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::ActionBibika).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'U' => match self.get_next_char()? {
                'I' => match self.get_next_char()? {
                    'L' => match self.get_next_char()? {
                        'D' => match self.get_next_char()? {
                            ';' => Ok(Instruction::new_simple(InstructionId::StdBuild).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_capital_letter_c(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            '1' => match self.get_next_char()? {
                '9' => match self.get_next_char()? {
                    '0' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::ActionC190).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'C' => match self.get_next_char()? {
                'W' => match self.get_next_char()? {
                    ';' => Ok(Instruction::new_simple(InstructionId::RotateCcw).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'R' => match self.get_next_char()? {
                'A' => match self.get_next_char()? {
                    'F' => match self.get_next_char()? {
                        'T' => match self.get_next_char()? {
                            ';' => Ok(Instruction::new_simple(InstructionId::ActionCraft).unwrap()),
                            _ => Err(UnknownInstruction { index: self.index }),
                        },
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'W' => match self.get_next_char()? {
                ';' => Ok(Instruction::new_simple(InstructionId::RotateCw).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_capital_letter_d(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'I' => match self.get_next_char()? {
                'G' => match self.get_next_char()? {
                    'G' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::StdDigg).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_capital_letter_f(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'L' => match self.get_next_char()? {
                'I' => match self.get_next_char()? {
                    'P' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::ProgFlip).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'I' => match self.get_next_char()? {
                'L' => match self.get_next_char()? {
                    'L' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::FillGun).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_capital_letter_h(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'E' => match self.get_next_char()? {
                'A' => match self.get_next_char()? {
                    'L' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::StdHeal).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'a' => match self.get_next_char()? {
                'n' => match self.get_next_char()? {
                    'd' => match self.get_next_char()? {
                        '+' => Ok(Instruction::new_simple(InstructionId::HandModeOn).unwrap()),
                        '-' => Ok(Instruction::new_simple(InstructionId::HandModeOff).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_capital_letter_m(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'I' => match self.get_next_char()? {
                'N' => match self.get_next_char()? {
                    'E' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::StdMine).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_capital_letter_r(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'A' => match self.get_next_char()? {
                'N' => match self.get_next_char()? {
                    'D' => match self.get_next_char()? {
                        ';' => Ok(Instruction::new_simple(InstructionId::ActionRandom).unwrap()),
                        _ => Err(UnknownInstruction { index: self.index }),
                    },
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'E' => match self.get_next_char()? {
                'M' => match self.get_next_char()? {
                    ';' => Ok(Instruction::new_simple(InstructionId::ActionRembot).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_left_square_bracket(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'A' => match self.get_next_char()? {
                'S' => match self.get_next_char()? {
                    ']' => Ok(Instruction::new_simple(InstructionId::CellAs).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }),
                },

                ']' => Ok(Instruction::new_simple(InstructionId::CellA).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'D' => match self.get_next_char()? {
                'W' => match self.get_next_char()? {
                    ']' => Ok(Instruction::new_simple(InstructionId::CellDw).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                ']' => Ok(Instruction::new_simple(InstructionId::CellD).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'F' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellF).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'S' => match self.get_next_char()? {
                'D' => match self.get_next_char()? {
                    ']' => Ok(Instruction::new_simple(InstructionId::CellSd).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                ']' => Ok(Instruction::new_simple(InstructionId::CellS).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'W' => match self.get_next_char()? {
                'A' => match self.get_next_char()? {
                    ']' => Ok(Instruction::new_simple(InstructionId::CellWa).unwrap()),
                    _ => Err(UnknownInstruction { index: self.index }),
                },
                ']' => Ok(Instruction::new_simple(InstructionId::CellW).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'a' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellAa).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'd' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellDd).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'f' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellFf).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'l' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellLeftHand).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'r' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellRightHand).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            's' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellSs).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            'w' => match self.get_next_char()? {
                ']' => Ok(Instruction::new_simple(InstructionId::CellWw).unwrap()),
                _ => Err(UnknownInstruction { index: self.index }),
            },
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_circumflex_accent(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'F' => Ok(Instruction::new_simple(InstructionId::MoveF).unwrap()),
            'W' => Ok(Instruction::new_simple(InstructionId::MoveW).unwrap()),
            'D' => Ok(Instruction::new_simple(InstructionId::MoveD).unwrap()),
            'S' => Ok(Instruction::new_simple(InstructionId::MoveS).unwrap()),
            'A' => Ok(Instruction::new_simple(InstructionId::MoveA).unwrap()),
            _ => Err(UnknownInstruction { index: self.index }),
        }
    }
    fn parse_latin_small_letter_i(&mut self) -> Result<Instruction, UnknownInstruction> {
        match self.get_next_char()? {
            'a' => Ok(Instruction::new_simple(InstructionId::InvDirA).unwrap()),
            'd' => Ok(Instruction::new_simple(InstructionId::InvDirD).unwrap()),
            's' => Ok(Instruction::new_simple(InstructionId::InvDirS).unwrap()),
            'w' => Ok(Instruction::new_simple(InstructionId::InvDirW).unwrap()),
            _ => Err(UnknownInstruction { index: self.index }),
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

        use crate::formats::internal::literals::{
            LabelIdentifierLiteral, StringLiteral, VariableIdentifierLiteral, VariableValueLiteral,
        };
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
            let s = concat!(
                "$<|<-|<=|",
                "^F^W^D^S^A",
                "adswzghrbq,",
                "[F][W][WA][D][DW][S][SD][A][AS][r][l][f][w][d][s][a]",
                "=G=n=e=f=c=a=b=s=k=d=A",
                "=B=K=g=y=r=o=q=x=R",
                "=hp50=hp-",
                "#S#E",
                "B1;B3;B2;BEEP;RAND;VB;GEO;ZZ;POLY;C190;CRAFT;UP;NANO;REM;",
                "BUILD;DIGG;HEAL;MINE;",
                "AUT+AUT-AGR+AGR-",
                "ANDOR",
                "CCW;CW;",
                "FLIP;",
                "FILL;",
                "iaidisiw",
                "Hand+Hand-",
                "<|"
            );
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
            //     actions
            expected_program[59] = Instruction::new_simple(InstructionId::ActionB1).unwrap();
            expected_program[60] = Instruction::new_simple(InstructionId::ActionB2).unwrap();
            expected_program[61] = Instruction::new_simple(InstructionId::ActionB3).unwrap();
            expected_program[62] = Instruction::new_simple(InstructionId::ActionBibika).unwrap();
            expected_program[63] = Instruction::new_simple(InstructionId::ActionRandom).unwrap();
            expected_program[64] = Instruction::new_simple(InstructionId::ActionWb).unwrap();
            expected_program[65] = Instruction::new_simple(InstructionId::ActionGeopack).unwrap();
            expected_program[66] = Instruction::new_simple(InstructionId::ActionZm).unwrap();
            expected_program[67] = Instruction::new_simple(InstructionId::ActionPoly).unwrap();
            expected_program[68] = Instruction::new_simple(InstructionId::ActionC190).unwrap();
            expected_program[69] = Instruction::new_simple(InstructionId::ActionCraft).unwrap();
            expected_program[70] = Instruction::new_simple(InstructionId::ActionUp).unwrap();
            expected_program[71] = Instruction::new_simple(InstructionId::ActionNano).unwrap();
            expected_program[72] = Instruction::new_simple(InstructionId::ActionRembot).unwrap();
            //     std
            expected_program[73] = Instruction::new_simple(InstructionId::StdBuild).unwrap();
            expected_program[74] = Instruction::new_simple(InstructionId::StdDigg).unwrap();
            expected_program[75] = Instruction::new_simple(InstructionId::StdHeal).unwrap();
            expected_program[76] = Instruction::new_simple(InstructionId::StdMine).unwrap();
            //     mode
            expected_program[77] = Instruction::new_simple(InstructionId::ModeAutodiggOn).unwrap();
            expected_program[78] = Instruction::new_simple(InstructionId::ModeAutodiggOff).unwrap();
            expected_program[79] = Instruction::new_simple(InstructionId::ModeAgrOn).unwrap();
            expected_program[80] = Instruction::new_simple(InstructionId::ModeAgrOff).unwrap();
            //     bool_mode
            expected_program[81] = Instruction::new_simple(InstructionId::BoolModeAnd).unwrap();
            expected_program[82] = Instruction::new_simple(InstructionId::BoolModeOr).unwrap();
            //     rotates
            expected_program[83] = Instruction::new_simple(InstructionId::RotateCcw).unwrap();
            expected_program[84] = Instruction::new_simple(InstructionId::RotateCw).unwrap();
            //     prog_flip
            expected_program[85] = Instruction::new_simple(InstructionId::ProgFlip).unwrap();
            //     fill_gun
            expected_program[86] = Instruction::new_simple(InstructionId::FillGun).unwrap();
            //     inv
            expected_program[87] = Instruction::new_simple(InstructionId::InvDirA).unwrap();
            expected_program[88] = Instruction::new_simple(InstructionId::InvDirD).unwrap();
            expected_program[89] = Instruction::new_simple(InstructionId::InvDirS).unwrap();
            expected_program[90] = Instruction::new_simple(InstructionId::InvDirW).unwrap();
            //     hand
            expected_program[91] = Instruction::new_simple(InstructionId::HandModeOn).unwrap();
            expected_program[92] = Instruction::new_simple(InstructionId::HandModeOff).unwrap();
            //     for tail check
            expected_program[93] = Instruction::new_simple(InstructionId::Return).unwrap();
            // actual_program
            let mut actual_program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, s);
            de.deserialize().unwrap();
            // asserts
            assert_eq!(expected_program, actual_program);
        }

        #[test]
        fn deserialize_commands() {
            let s = "$^W\n^A~^D_^F ^S";
            // expected_program
            let mut expected_program = Program::default();
            expected_program[InstructionPosition::new(0, 0, 0).unwrap()] =
                Instruction::new_simple(InstructionId::MoveW).unwrap();
            expected_program[InstructionPosition::new(0, 1, 0).unwrap()] =
                Instruction::new_simple(InstructionId::MoveA).unwrap();
            expected_program[InstructionPosition::new(1, 0, 0).unwrap()] =
                Instruction::new_simple(InstructionId::MoveD).unwrap();
            expected_program[InstructionPosition::new(1, 0, 4).unwrap()] =
                Instruction::new_simple(InstructionId::MoveF).unwrap();
            expected_program[InstructionPosition::new(1, 0, 6).unwrap()] =
                Instruction::new_simple(InstructionId::MoveS).unwrap();
            // actual_program
            let mut actual_program = Program::default();
            let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, s);
            de.deserialize().unwrap();
            // asserts
            assert_eq!(expected_program, actual_program);
        }

        #[test]
        fn deserialize_literals() {
            let s = concat!(
                "$",
                "|:|hi:|012:",
                ">abc|:>zxc>->s12>=>sbf>",
                "!?if<?ifn<",
                "#Rrsp<",
                "(va0<0)(a=99999)(va2>-5)",
                "{dst}!{bp}"
            );
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
            //         var_cmp
            expected_program[10] = Instruction::new_var_cmp(
                InstructionId::VarLess,
                VariableIdentifierLiteral::new_from_array([b'v', b'a', b'0', 0]).unwrap(),
                VariableValueLiteral::new_from_value(0).unwrap(),
            )
            .unwrap();
            expected_program[11] = Instruction::new_var_cmp(
                InstructionId::VarEqual,
                VariableIdentifierLiteral::new_from_array([b'a', 0, 0, 0]).unwrap(),
                VariableValueLiteral::new_from_value(99999).unwrap(),
            )
            .unwrap();
            expected_program[12] = Instruction::new_var_cmp(
                InstructionId::VarMore,
                VariableIdentifierLiteral::new_from_array([b'v', b'a', b'2', 0]).unwrap(),
                VariableValueLiteral::new_from_value(-5).unwrap(),
            )
            .unwrap();
            expected_program[13] = Instruction::new_string(
                InstructionId::DebugSet,
                StringLiteral::new_from_array([b'd', b's', b't', 0]).unwrap(),
            )
            .unwrap();
            expected_program[14] = Instruction::new_string(
                InstructionId::DebugBreak,
                StringLiteral::new_from_array([b'b', b'p', 0, 0]).unwrap(),
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

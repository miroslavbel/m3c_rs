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
    Instruction, InstructionId, InstructionKind, InstructionPosition,
    InstructionPositionOverflowError, Program,
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
    ///
    /// # Errors
    pub fn deserialize_v2(&mut self) -> Result<(), DeserializeErrors> {
        self.check_magic()?;
        self.program.reset();
        loop {
            match self.parse_next_v2()? {
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
    fn parse_next_v2(&mut self) -> Result<Option<InstructionOrCommand>, ParseNextErrors> {
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

        use crate::formats::internal::Program;

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
    }
}

// endregion: tests

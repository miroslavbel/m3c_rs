//! Serializer and deserializer for New Text format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::{
    error::Error,
    fmt, io,
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::formats::internal::{
    literals::{
        LabelIdentifierLiteral, Literal, LiteralType, StringLiteral, VariableIdentifierLiteral,
        VariableValueLiteral,
    },
    InstructionData, InstructionId,
};
use crate::formats::internal::{
    Instruction, InstructionKind, InstructionPosition, InstructionPositionOverflowError, Program,
};
use crate::formats::native::new::diagnostics::{Diagnostics, NoMagicFound, UnknownToken};
use crate::utils::{CharPosition, EnumerateWithPosition};

mod data;
use data::{NTF2INode, NTF2I};

use self::data::{I2NTFNode, I2NTF};

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

                match &NTF2I.binary_search_by_key(&first_char, |&(ch, _)| ch) {
                    Err(_) => return Err(UnknownInstruction { index: self.index }.into()),
                    Ok(x) => {
                        let mut the_node = &NTF2I[*x].1;
                        loop {
                            match the_node {
                                NTF2INode::Command(command) => {
                                    return Ok(Some(InstructionOrCommand::Command(*command)))
                                }
                                NTF2INode::Id(id) => {
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
                                NTF2INode::Literal((literal, node)) => {
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
                                NTF2INode::Chars(current) => {
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

pub struct TextFormatDeserializerV2<'s> {
    // original source
    source: &'s str,
    // an iterator over the chars (and its position on the `source`)
    source_iter: Peekable<EnumerateWithPosition<'s>>,
    // points to char pretending to be the fisrt token char
    token_start: (CharPosition, char),
    // position of the last valid char in the token
    last_char: CharPosition,
    // tracks the start (`.0`) and the end (`.1`) of the illegal char sequence. Value `None` stands
    // for no sequence detected
    illegal_chars: Option<(CharPosition, CharPosition)>,
    // instruction position at program
    ins_pos: InstructionPosition,
    // diagnostics
    diagnostics: Vec<Diagnostics>,
}

impl<'s> TextFormatDeserializerV2<'s> {
    pub fn new(str: &'s str) -> Self {
        Self {
            source: str,
            // FIXME it will be reassigned in `reset`. Maybe use MaybeUninit?
            source_iter: EnumerateWithPosition::new(str).peekable(),
            // NOTE: every time updates before read in `parse_next_token`
            token_start: (CharPosition::default(), '\0'),
            // NOTE: every time updates before read in `parse_next_token`
            last_char: CharPosition::default(),
            // NOTE: every time updates before read in `parse_next_token`
            illegal_chars: None,
            ins_pos: InstructionPosition::default(),
            diagnostics: vec![],
        }
    }
    pub fn deserialize(&mut self, program: &mut Program) -> Vec<Diagnostics> {
        // resets
        self.reset();
        program.reset();

        self.check_magic();

        loop {
            match self.parse_next_token() {
                None => {
                    return std::mem::take(&mut self.diagnostics);
                }
                Some(InstructionOrCommand::Instruction(ins)) => {
                    program[self.ins_pos] = ins;
                    match self.ins_pos.move_forward() {
                        Ok(_) => {
                            continue;
                        }
                        Err(_) => {
                            // TODO add to `self.diagnostics`
                            return std::mem::take(&mut self.diagnostics);
                        }
                    }
                }
                Some(InstructionOrCommand::Command(_)) => {
                    todo!()
                }
            }
        }
    }
    /// Resets the state.
    ///
    /// Doesn't reset values below (`parse_next_token` is responsible for it):
    /// + `token_start`
    /// + `last_char`
    /// + `illegal_chars`
    fn reset(&mut self) {
        self.source_iter = EnumerateWithPosition::new(self.source).peekable();
        self.ins_pos = InstructionPosition::default();
        // TODO: what about diagnostic?
    }
    /// Peeks one char from `source_iter`. If it's a valid magic advances the iterator. If not
    /// pushes `NoMagicFound` diagnostic.
    fn check_magic(&mut self) {
        match self.source_iter.peek() {
            Some((_, ch)) if ch == &'$' => {
                self.source_iter.next();
            }
            None | Some(_) => {
                self.diagnostics.push(NoMagicFound::new().into());
            }
        }
    }
    /// Parses the next token.
    ///
    /// Consumes `self.source_iter` until whole token is read or the end of the iterator is
    /// reached. If there is an illegal char sequence before the token or the iterator's end, add
    /// `UnknownToken` diagnostic.
    fn parse_next_token(&mut self) -> Option<InstructionOrCommand> {
        self.illegal_chars = None;

        // Here it reads the char for the really *first* time. So `self.illegal_chars` is `None`.
        // So no need to call `self.next_char`
        self.token_start = self.source_iter.next()?;
        self.last_char = self.token_start.0;

        // loop to find first valid char to start token with
        loop {
            match &NTF2I.binary_search_by_key(&self.token_start.1, |&(ch, _)| ch) {
                // that isn't a valid char to start token with. It needs to update
                // `self.illegal_chars` and set up the new value to the `self.token_start`
                Err(_) => {
                    if let Some((_, end)) = &mut self.illegal_chars {
                        // just update the end of illegal char sequences
                        *end = self.token_start.0;
                    } else {
                        // that's really *first* illegal char. Initialize `illegal_chars`
                        self.illegal_chars = Some((self.token_start.0, self.token_start.0));
                    }

                    // set up the next char to `self.token_start` and run iteration
                    self.token_start = self.next_char()?;
                    continue;
                }

                // That's a valid char to start token with
                Ok(i) => {
                    let mut node = &NTF2I[*i].1;

                    // loop over nodes util full token will be read
                    loop {
                        match node {
                            NTF2INode::Command(command) => {
                                if let Some((start, end)) = self.illegal_chars {
                                    self.diagnostics.push(UnknownToken::new(start, end).into())
                                }

                                return Some(InstructionOrCommand::Command(*command));
                            }
                            NTF2INode::Id(id) => {
                                if let Some((start, end)) = self.illegal_chars {
                                    self.diagnostics.push(UnknownToken::new(start, end).into())
                                }

                                return match id.kind() {
                                    InstructionKind::Simple => {
                                        Instruction::new_simple(*id).unwrap().into()
                                    }
                                    _ => todo!(),
                                };
                            }
                            NTF2INode::Literal(_) => {
                                todo!()
                            }
                            NTF2INode::Chars(current) => {
                                let next_char = self.next_char()?;
                                match current.binary_search_by_key(&next_char.1, |&(ch, _)| ch) {
                                    // Means we have unknown char at the middle of token (second or further char)
                                    Err(_) => {
                                        // here `end` updated by `self.last_char` because `next_char` can be
                                        // a valid char to start token with
                                        if let Some((_, end)) = &mut self.illegal_chars {
                                            *end = self.last_char;
                                        } else {
                                            self.illegal_chars =
                                                Some((self.token_start.0, self.last_char));
                                        }

                                        // set up the next char to `self.token_start` and run the main iteration
                                        self.token_start = next_char;
                                        break;
                                    }
                                    Ok(x) => {
                                        self.last_char = next_char.0;
                                        node = &current[x].1;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /// Just wrapper around `self.source_iter.next()`.
    ///
    /// If the result of `self.source_iter.next()` is `None` pushes `UnknownToken` diagnostic.
    fn next_char(&mut self) -> Option<(CharPosition, char)> {
        match self.source_iter.next() {
            Some(x) => Some(x),
            None => {
                if let Some((start, _)) = self.illegal_chars {
                    self.diagnostics
                        .push(UnknownToken::new(start, self.last_char).into())
                } else {
                    self.diagnostics
                        .push(UnknownToken::new(self.token_start.0, self.last_char).into())
                }
                None
            }
        }
    }
}

/// A structure that serializes New Text format from [Internal format](crate::formats::internal).
#[derive(Debug, Clone, Copy)]
pub struct TextFormatSerializer<'p> {
    program: &'p Program,
}

impl<'p> TextFormatSerializer<'p> {
    /// Creates a new [`TextFormatSerializer`].
    pub fn new(program: &'p Program) -> Self {
        Self { program: program }
    }
    /// Serializes program from [Internal format](crate::formats::internal).
    ///
    /// It is guaranteed that all bytes written to the given `writer` are valid ASCII characters.
    ///
    /// # Errors
    ///
    /// As this function internally uses the [`io::Write::write_all`] method, see the error section
    /// of this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use m3c::formats::internal::{Instruction, InstructionId, Program};
    /// use m3c::serialization::native::new::TextFormatSerializer;
    ///
    /// // buffer for writing
    /// let mut buf = vec![];
    ///
    /// // build a simple program
    /// let mut program = Program::default();
    /// program[0] = Instruction::new_simple(InstructionId::MoveW).unwrap();
    /// program[1] = Instruction::new_simple(InstructionId::MoveS).unwrap();
    ///
    /// // serialize it to NewTextFormat
    /// let mut se = TextFormatSerializer::new(&program);
    /// se.serialize(&mut buf).unwrap();
    ///
    /// assert_eq!(b"$^W^S", &buf[..])
    /// ```
    pub fn serialize<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        // write magic
        writer.write_all(b"$")?;

        // Here it isn't necessary to check if the first instruction is really not `Empty`,
        // because this is the beginning of the program
        let mut last_not_empty = InstructionPosition::default();

        for (pos, ins) in self.program.instruction_positions() {
            match ins.id() {
                InstructionId::Empty => {
                    continue;
                }
                _ => {
                    last_not_empty.write_delta(pos, writer)?;
                    last_not_empty = pos;

                    let i = I2NTF.binary_search_by_key(&ins.id(), |&(a, _)| a).unwrap();
                    let (_, node_list) = I2NTF[i];

                    for node in node_list {
                        match node {
                            I2NTFNode::Chars(x) => {
                                writer.write_all(x)?;
                            }
                            I2NTFNode::Literal(lt) => match ins.data() {
                                InstructionData::Label(literal)
                                    if matches!(lt, LiteralType::LabelIdentifierLiteral) =>
                                {
                                    literal.write_all(writer)?;
                                }
                                InstructionData::String(literal)
                                    if matches!(lt, LiteralType::StringLiteral) =>
                                {
                                    literal.write_all(writer)?;
                                }
                                InstructionData::VarCmp((literal, _))
                                    if matches!(lt, LiteralType::VariableIdentifierLiteral) =>
                                {
                                    literal.write_all(writer)?;
                                }
                                InstructionData::VarCmp((_, literal))
                                    if matches!(lt, LiteralType::VariableValueLiteral) =>
                                {
                                    literal.write_all(writer)?;
                                }
                                _ => {
                                    unreachable!("the I2NTF data is corrupt")
                                }
                            },
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl InstructionPosition {
    /// Writes the delta between `self` and `other`.
    ///
    /// # Safety
    ///
    /// `other` must be equal or greater than `self`.
    fn write_delta<W>(&self, other: Self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        if other.index() - self.index() <= 1 {
            return Ok(());
        }
        if self.page() == other.page() {
            if self.row() == other.row() {
                let delta = other.column() - self.column();
                match delta {
                    0..=1 => return Ok(()),
                    _ => InstructionPosition::write_empty_columns(delta - 1, writer),
                }
            } else {
                let empty_rows = other.row() - self.row() - 1;
                InstructionPosition::write_empty_rows(empty_rows, writer)?;
                InstructionPosition::write_empty_columns(other.column(), writer)
            }
        } else {
            InstructionPosition::write_page_delta(other.page() - self.page(), writer)?;
            InstructionPosition::write_empty_rows(other.row(), writer)?;
            InstructionPosition::write_empty_columns(other.column(), writer)
        }
    }
    fn write_page_delta<W>(mut n: u8, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        while n > 1 {
            writer.write_all(b"~\n")?;
            n -= 1;
        }
        writer.write_all(b"~")
    }
    fn write_empty_rows<W>(mut n: u8, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(b"\n")?;
        if n > 10 {
            return Self::write_empty_rows(10, writer);
        }
        while n > 4 {
            let mut buf = [b'.', b'0', b'.'];
            if n < 10 {
                buf[1] = n + b'0';
            }
            writer.write_all(&buf)?;
            n -= n - 1;
        }
        while n > 1 {
            writer.write_all(b".")?;
            n -= 1;
        }
        while n > 0 {
            writer.write_all(b"\n")?;
            n -= 1;
        }
        Ok(())
    }
    fn write_empty_columns<W>(mut n: u8, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        while n > 2 {
            writer.write_all(b"_")?;
            n -= 3;
        }
        while n > 0 {
            writer.write_all(b" ")?;
            n -= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::formats::internal::InstructionPosition;

    #[test]
    fn columns() {
        let mut buf = vec![];
        // 0
        InstructionPosition::write_empty_columns(0, &mut buf).unwrap();
        assert_eq!("".as_bytes(), buf);
        buf.clear();
        // 1
        InstructionPosition::write_empty_columns(1, &mut buf).unwrap();
        assert_eq!(" ".as_bytes(), buf);
        buf.clear();
        // 2
        InstructionPosition::write_empty_columns(2, &mut buf).unwrap();
        assert_eq!("  ".as_bytes(), buf);
        buf.clear();
        // 3
        InstructionPosition::write_empty_columns(3, &mut buf).unwrap();
        assert_eq!("_".as_bytes(), buf);
        buf.clear();
        // 4
        InstructionPosition::write_empty_columns(4, &mut buf).unwrap();
        assert_eq!("_ ".as_bytes(), buf);
        buf.clear();
        // 5
        InstructionPosition::write_empty_columns(5, &mut buf).unwrap();
        assert_eq!("_  ".as_bytes(), buf);
        buf.clear();
        // 6
        InstructionPosition::write_empty_columns(6, &mut buf).unwrap();
        assert_eq!("__".as_bytes(), buf);
        buf.clear();
    }

    #[test]
    fn rows() {
        let mut buf = vec![];
        // 0
        InstructionPosition::write_empty_rows(0, &mut buf).unwrap();
        assert_eq!("\n".as_bytes(), buf);
        buf.clear();
        // 1
        InstructionPosition::write_empty_rows(1, &mut buf).unwrap();
        assert_eq!("\n\n".as_bytes(), buf);
        buf.clear();
        // 2
        InstructionPosition::write_empty_rows(2, &mut buf).unwrap();
        assert_eq!("\n.\n".as_bytes(), buf);
        buf.clear();
        // 3
        InstructionPosition::write_empty_rows(3, &mut buf).unwrap();
        assert_eq!("\n..\n".as_bytes(), buf);
        buf.clear();
        // 4
        InstructionPosition::write_empty_rows(4, &mut buf).unwrap();
        assert_eq!("\n...\n".as_bytes(), buf);
        buf.clear();
        // 5
        InstructionPosition::write_empty_rows(5, &mut buf).unwrap();
        assert_eq!("\n.5.\n".as_bytes(), buf);
        buf.clear();
        // 6
        InstructionPosition::write_empty_rows(6, &mut buf).unwrap();
        assert_eq!("\n.6.\n".as_bytes(), buf);
        buf.clear();
        // 10
        InstructionPosition::write_empty_rows(10, &mut buf).unwrap();
        assert_eq!("\n.0.\n".as_bytes(), buf);
        buf.clear();
        // 11
        InstructionPosition::write_empty_rows(11, &mut buf).unwrap();
        assert_eq!("\n\n.0.\n".as_bytes(), buf);
        buf.clear();
    }

    #[test]
    fn pages() {
        let mut buf = vec![];
        // 0
        InstructionPosition::write_page_delta(0, &mut buf).unwrap();
        assert_eq!("~".as_bytes(), buf);
        buf.clear();
        // 1
        InstructionPosition::write_page_delta(1, &mut buf).unwrap();
        assert_eq!("~".as_bytes(), buf);
        buf.clear();
        // 2
        InstructionPosition::write_page_delta(2, &mut buf).unwrap();
        assert_eq!("~\n~".as_bytes(), buf);
        buf.clear();
    }
}

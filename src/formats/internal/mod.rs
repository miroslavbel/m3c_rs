//! An internal raw program representation.

pub mod literals;

use literals::{
    LabelIdentifierLiteral, StringLiteral, VariableIdentifierLiteral, VariableValueLiteral,
};
use std::error::Error;
use std::fmt;
use std::ops::{Index, IndexMut};

// region: errors

#[derive(Copy, Clone, Debug)]
pub struct UnsupportedInstructionId {}

impl UnsupportedInstructionId {
    const DETAILS: &'static str = "unsupported instruction id for this operation";
}

impl fmt::Display for UnsupportedInstructionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::DETAILS)
    }
}

impl Error for UnsupportedInstructionId {}

#[derive(Copy, Clone, Debug)]
pub struct InstructionPositionConstructionError {}

impl InstructionPositionConstructionError {
    const DETAILS: &'static str = "one of the given page, row, or column has an invalid value";
}

impl fmt::Display for InstructionPositionConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::DETAILS)
    }
}

impl Error for InstructionPositionConstructionError {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InstructionPositionOverflowError {}

impl InstructionPositionOverflowError {
    const DETAILS: &'static str = "operation was not performed, as it results in overflow";
}

impl fmt::Display for InstructionPositionOverflowError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Self::DETAILS)
    }
}

impl Error for InstructionPositionOverflowError {}

// endregion: errors

// region: instruction_id

/// Instruction's ids.
///
/// This enumeration doesn't contain ids for `LAST` and `RESTART` instructions.
///
/// All values are taken from official client. Values are in range `[0-182]`. There is no ids for
/// values `13, 34, 41-42, 55-56, 61-73, 75, 78-118, 121-122, 124-130, 150-155`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InstructionId {
    Empty = 0,
    Back = 1,

    Start = 2,
    End = 3,

    MoveW = 4,
    MoveA = 5,
    MoveS = 6,
    MoveD = 7,

    Digg = 8,

    LookW = 9,
    LookA = 10,
    LookS = 11,
    LookD = 12,

    MoveF = 14,

    RotateCcw = 15,
    RotateCw = 16,

    ActionBuild = 17,
    ActionGeo = 18,
    ActionRoad = 19,
    ActionHeal = 20,
    ActionQuadro = 21,
    ActionRandom = 22,
    ActionBibika = 23,

    GoTo = 24,
    GoSub = 25,
    GoSub1 = 26,
    Return = 27,
    Return1 = 28,

    CellWa = 29,
    CellSd = 30,
    CellW = 31,
    CellDw = 32,
    CellA = 33,
    CellD = 35,
    CellAs = 36,
    CellS = 37,

    BoolModeOr = 38,
    BoolModeAns = 39,

    Label = 40,

    CcNotEmpty = 43,
    CcEmpty = 44,
    CcGravity = 45,
    CcCrystall = 46,
    CcAlive = 47,
    CcBolder = 48,
    CcSand = 49,
    CcRock = 50,
    CcDead = 51,
    CccRedRock = 52,
    CccBlackRock = 53,
    CcAcid = 54,
    CccQuadro = 57,
    CccRoad = 58,
    CccRedBlock = 59,
    CccYellowBlock = 60,
    CccBox = 74,
    CccOpor = 76,
    CccGreenBlock = 77,

    VarMore = 119,
    VarLess = 120,
    VarEqual = 123,

    CellWw = 131,
    CellAa = 132,
    CellSs = 133,
    CellDd = 134,
    CellF = 135,
    CellFf = 136,

    GoSubF = 137,
    ReturnF = 138,

    IfNotGoTo = 139,
    IfGoTo = 140,

    StdDigg = 141,
    StdBuild = 142,
    StdHeal = 143,
    ProgFlip = 144,
    StdMine = 145,

    CcGun = 146,
    FillGun = 147,

    CbHp = 148,
    CbHp50 = 149,

    CellRightHand = 156,
    CellLeftHand = 157,

    ModeAutodiggOn = 158,
    ModeAutodiggOff = 159,
    ModeAgrOn = 160,
    ModeAgrOff = 161,

    ActionB1 = 162,
    ActionB3 = 163,
    ActionB2 = 164,
    ActionWb = 165,

    OnResp = 166,

    ActionGeopack = 167,
    ActionZm = 168,
    ActionC190 = 169,
    ActionPoly = 170,
    ActionUp = 171,
    ActionCraft = 172,
    ActionNano = 173,
    ActionRembot = 174,

    InvDirW = 175,
    InvDirA = 176,
    InvDirS = 177,
    InvDirD = 178,

    HandModeON = 179,
    HandModeOFF = 180,

    DebugBreak = 181,
    DebugSet = 182,
}

impl InstructionId {
    /// Returns the appropriate [`InstructionKind`].
    pub fn kind(self) -> InstructionKind {
        match self {
            Self::DebugBreak | Self::DebugSet => InstructionKind::String,

            Self::VarMore | Self::VarLess | Self::VarEqual => InstructionKind::VarCmp,

            Self::GoTo
            | Self::GoSub
            | Self::GoSub1
            | Self::Label
            | Self::GoSubF
            | Self::IfNotGoTo
            | Self::IfGoTo
            | Self::OnResp => InstructionKind::Label,

            _ => InstructionKind::Simple,
        }
    }
}

impl Default for InstructionId {
    /// Returns [Empty](Self::Empty).
    fn default() -> Self {
        Self::Empty
    }
}

/// Instructions' kind by containing (or not) additional info (except the [`InstructionId`]).
///
/// Each instruction refers to one of these kind.
pub enum InstructionKind {
    /// Instructions of this kind don't contain any additional info.
    Simple,
    /// Instructions of this kind contain a [label literal](LabelIdentifierLiteral).
    Label,
    /// Instructions of this kind contain a [variable literal](VariableIdentifierLiteral) and a
    /// [value literal](VariableValueLiteral).
    VarCmp,
    /// Instructions of this kind contain a [string literal](StringLiteral).
    String,
}

impl From<InstructionId> for InstructionKind {
    /// Returns the appropriate [`InstructionKind`].
    fn from(instruction_id: InstructionId) -> Self {
        instruction_id.kind()
    }
}

// endregion: instruction_id

// region: instruction

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct VarCmpInstructionData {
    variable_identifier: VariableIdentifierLiteral,
    variable_value: VariableValueLiteral,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum InstructionData {
    Simple,
    Label(LabelIdentifierLiteral),
    VarCmp(VarCmpInstructionData),
    String(StringLiteral),
}

/// Program instruction.
///
/// Each instruction refers to one of the [`InstructionId`] and to one of the [`InstructionKind`].
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    id: InstructionId,
    data: InstructionData,
}

impl Instruction {
    /// Constructs a new instruction of the [`Simple`](InstructionKind::Simple) kind with the given
    /// `instruction_id`.
    ///
    /// # Errors
    /// If the given `instruction_id` is not appropriate to the [`Simple`](InstructionKind::Simple)
    /// kind an [`UnsupportedInstructionId`] will be returned.
    pub fn new_simple(instruction_id: InstructionId) -> Result<Self, UnsupportedInstructionId> {
        match instruction_id.kind() {
            InstructionKind::Simple => Ok(Instruction {
                id: instruction_id,
                data: InstructionData::Simple,
            }),
            _ => Err(UnsupportedInstructionId {}),
        }
    }
    /// Constructs a new instruction of the [`Label`](InstructionKind::Label) kind with the given
    /// `instruction_id`.
    ///
    /// # Errors
    /// If the given `instruction_id` is not appropriate to the [`Label`](InstructionKind::Label)
    /// kind an [`UnsupportedInstructionId`] will be returned.
    pub fn new_label(
        instruction_id: InstructionId,
        label: LabelIdentifierLiteral,
    ) -> Result<Self, UnsupportedInstructionId> {
        match instruction_id.kind() {
            InstructionKind::Label => Ok(Instruction {
                id: instruction_id,
                data: InstructionData::Label(label),
            }),
            _ => Err(UnsupportedInstructionId {}),
        }
    }
    /// Returns the [instruction id](InstructionId).
    pub fn id(&self) -> InstructionId {
        self.id
    }
    /// Returns the [instruction kind](InstructionKind).
    pub fn kind(&self) -> InstructionKind {
        self.id.kind()
    }
}

impl Default for Instruction {
    /// Constructs the [Empty](InstructionId::Empty) instruction.
    fn default() -> Self {
        Self {
            id: InstructionId::default(),
            data: InstructionData::Simple,
        }
    }
}

// endregion: instruction

// region: instruction_position

/// Describe an instruction position at the program.
#[derive(Copy, Clone, Debug)]
pub struct InstructionPosition {
    page: u8,
    row: u8,
    column: u8,
}

impl InstructionPosition {
    /// Constructs a new instance with the given `page`, `row` and `column`.
    ///
    /// # Errors
    /// If one of the given `page`, `row`, or `column` has an invalid value an
    /// [`InstructionPositionConstructionError`] will be returned.
    pub fn new(
        page: u8,
        row: u8,
        column: u8,
    ) -> Result<Self, InstructionPositionConstructionError> {
        if page as usize >= Program::PAGES_PER_PROGRAM
            || row as usize >= Program::ROWS_PER_PAGE
            || column as usize >= Program::INSTRUCTIONS_PER_ROW
        {
            Err(InstructionPositionConstructionError {})
        } else {
            Ok(Self { page, row, column })
        }
    }
    /// Returns the "flat index".
    pub fn index(&self) -> usize {
        self.page as usize * Program::ROWS_PER_PAGE * Program::INSTRUCTIONS_PER_ROW
            + self.row as usize * Program::INSTRUCTIONS_PER_ROW
            + self.column as usize
    }
    /// Returns the index of the page.
    pub fn page(&self) -> u8 {
        self.page
    }
    /// Returns the index of the row on page.
    pub fn row(&self) -> u8 {
        self.row
    }
    /// Returns the index of the column in the row.
    pub fn column(&self) -> u8 {
        self.column
    }
    /// Moves this [`InstructionPosition`] to the next position.
    ///
    /// # Errors
    /// If this [`InstructionPosition`] already points to the last position in the program, an
    /// [`InstructionPositionOverflowError`] will be returned and the operation will not be performed.
    pub fn move_forward(&mut self) -> Result<(), InstructionPositionOverflowError> {
        if self.column as usize == Program::INSTRUCTIONS_PER_ROW - 1 {
            self.move_to_next_row()
        } else {
            self.column += 1;
            Ok(())
        }
    }
    /// Moves this [`InstructionPosition`] to the beginning of next row.
    ///
    /// # Errors
    /// If this [`InstructionPosition`] already points to an instruction in the last row on
    /// page, an [`InstructionPositionOverflowError`] will be returned and the operation will not
    /// be performed.
    pub fn move_to_next_row(&mut self) -> Result<(), InstructionPositionOverflowError> {
        if self.row as usize == Program::ROWS_PER_PAGE - 1 {
            self.move_to_next_page()
        } else {
            self.row += 1;
            self.column = 0;
            Ok(())
        }
    }
    /// Moves this [`InstructionPosition`] to the beginning of next page.
    ///
    /// # Errors
    /// If this [`InstructionPosition`] already points to an instruction on the last page, an
    /// [`InstructionPositionOverflowError`] will be returned and the operation will not be performed.
    pub fn move_to_next_page(&mut self) -> Result<(), InstructionPositionOverflowError> {
        if self.page as usize == Program::PAGES_PER_PROGRAM - 1 {
            Err(InstructionPositionOverflowError {})
        } else {
            self.page += 1;
            self.row = 0;
            self.column = 0;
            Ok(())
        }
    }
}

impl Default for InstructionPosition {
    /// Constructs an instance pointed to the first instruction in program.
    fn default() -> Self {
        Self {
            page: 0,
            row: 0,
            column: 0,
        }
    }
}

// endregion: instruction_position

// region: program

/// Raw program representation.
///
/// Each program contains [3072](Self::INSTRUCTIONS_PER_PROGRAM) [instructions](Instruction):
/// * each program contains [16](Self::PAGES_PER_PROGRAM) pages
/// * each page contains [12](Self::ROWS_PER_PAGE) rows
/// * each row contains [16](Self::INSTRUCTIONS_PER_ROW) instructions
#[derive(Debug, PartialEq, Eq)]
pub struct Program {
    instructions: Box<[Instruction; Self::INSTRUCTIONS_PER_PROGRAM]>,
}

impl Program {
    /// Number of pages per program.
    pub const PAGES_PER_PROGRAM: usize = 16;
    /// Number of rows per page.
    pub const ROWS_PER_PAGE: usize = 12;
    /// Number of instructions per row.
    pub const INSTRUCTIONS_PER_ROW: usize = 16;
    /// Number of instruction per program.
    pub const INSTRUCTIONS_PER_PROGRAM: usize =
        Self::PAGES_PER_PROGRAM * Self::ROWS_PER_PAGE * Self::INSTRUCTIONS_PER_ROW;
    /// Resets all instructions of this [`Program`] to [`InstructionId::Empty`].
    pub fn reset(&mut self) {
        self.instructions.fill(Instruction::default());
    }
}

impl Default for Program {
    /// Constructs the program where each instruction is the [Empty](InstructionId::Empty)
    /// instruction.
    fn default() -> Self {
        Self {
            instructions: Box::new([Instruction::default(); Self::INSTRUCTIONS_PER_PROGRAM]),
        }
    }
}

impl Index<usize> for Program {
    type Output = Instruction;
    fn index(&self, index: usize) -> &Self::Output {
        &self.instructions[index]
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.instructions[index]
    }
}

impl Index<InstructionPosition> for Program {
    type Output = Instruction;
    /// Performs the indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// Will never panic because [`InstructionPosition`] always points to a valid instruction position.
    fn index(&self, position: InstructionPosition) -> &Self::Output {
        &self.instructions[position.index()]
    }
}

impl IndexMut<InstructionPosition> for Program {
    /// Performs the mutable indexing (`container[index]`) operation.
    ///
    /// # Panics
    ///
    /// Will never panic because [`InstructionPosition`] always points to a valid instruction position.
    fn index_mut(&mut self, position: InstructionPosition) -> &mut Self::Output {
        &mut self.instructions[position.index()]
    }
}

// endregion: program

// region: test

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod instruction_position {

        use super::super::InstructionPosition;

        #[test]
        fn move_forward_through_row() {
            let mut instruction_position = InstructionPosition::new(1, 2, 15).unwrap();
            instruction_position.move_forward().unwrap();
            assert_eq!(1, instruction_position.page());
            assert_eq!(3, instruction_position.row());
            assert_eq!(0, instruction_position.column());
        }

        #[test]
        fn move_forward_through_page() {
            let mut instruction_position = InstructionPosition::new(1, 11, 15).unwrap();
            instruction_position.move_forward().unwrap();
            assert_eq!(2, instruction_position.page());
            assert_eq!(0, instruction_position.row());
            assert_eq!(0, instruction_position.column());
        }

        #[test]
        #[should_panic]
        fn move_forward_panic() {
            let mut instruction_position = InstructionPosition::new(15, 11, 15).unwrap();
            instruction_position.move_forward().unwrap();
        }
    }
}

// endregion: test

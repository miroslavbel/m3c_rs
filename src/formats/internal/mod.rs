//! An internal raw program representation.

pub mod literals;

use literals::{
    LabelIdentifierLiteral, StringLiteral, VariableIdentifierLiteral, VariableValueLiteral,
};

// region: instruction_id

/// Instruction's ids.
///
/// This enumeration doesn't contain ids for `LAST` and `RESTART` instructions.
///
/// All values are taken from official client. Values are in range `[0-182]`. There is no ids for
/// values `13, 34, 41-42, 55-56, 61-73, 75, 78-118, 121-122, 124-130, 150-155`.
#[derive(Copy, Clone)]
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

// endregion: instruction_id

// region: instruction

#[derive(Copy, Clone)]
struct VarCmpInstructionData {
    variable_identificator: VariableIdentifierLiteral,
    variable_value: VariableValueLiteral,
}

#[derive(Copy, Clone)]
enum InstructionData {
    Simple,
    Label(LabelIdentifierLiteral),
    VarCmp(VarCmpInstructionData),
    String(StringLiteral),
}

/// Program instruction.
///
/// Each instruction has one of the [InstructionId].
pub struct Instruction {
    id: InstructionId,
    data: InstructionData,
}

impl Instruction {
    /// Returns the [instruction id].
    ///
    /// [instruction id]: [InstructionId]
    pub fn id(&self) -> InstructionId {
        self.id
    }
}

// endregion: instruction

// region: program

/// Raw program representation.
///
/// Each program contains [3072](Self::INSTRUCTIONS_PER_PROGRAM) [instructions](Instruction):
/// * each program contains [16](Self::PAGES_PER_PROGRAM) pages
/// * each page contains [12](Self::ROWS_PER_PAGE) rows
/// * each row contains [16](Self::INSTRUCTIONS_PER_ROW) instructions
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
}

// endregion: program

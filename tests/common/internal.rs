use m3c::formats::internal::literals::{
    LabelIdentifierLiteral, StringLiteral, VariableIdentifierLiteral, VariableValueLiteral,
};
use m3c::formats::internal::{Instruction, InstructionId, InstructionPosition, Program};

/// Returns empty program.
pub fn empty() -> Program {
    Program::default()
}

/// Returns program with only one instuction `MoveF` at the start.
pub fn only_move_w() -> Program {
    let mut program = Program::default();
    program[0] = Instruction::new_simple(InstructionId::MoveW).unwrap();
    program
}

/// Returns program with instructions `MoveW`, `MoveS`, `MoveF` at the start.
pub fn moves_wsf() -> Program {
    let mut program = Program::default();
    program[0] = Instruction::new_simple(InstructionId::MoveW).unwrap();
    program[1] = Instruction::new_simple(InstructionId::MoveS).unwrap();
    program[2] = Instruction::new_simple(InstructionId::MoveF).unwrap();
    program
}

/// Returns a program which contains all `Simple` kind instructions.
pub fn all_simple() -> Program {
    let mut program = Program::default();
    //     returns
    program[0] = Instruction::new_simple(InstructionId::Return).unwrap();
    program[1] = Instruction::new_simple(InstructionId::Return1).unwrap();
    program[2] = Instruction::new_simple(InstructionId::ReturnF).unwrap();
    //     moves
    program[3] = Instruction::new_simple(InstructionId::MoveF).unwrap();
    program[4] = Instruction::new_simple(InstructionId::MoveW).unwrap();
    program[5] = Instruction::new_simple(InstructionId::MoveD).unwrap();
    program[6] = Instruction::new_simple(InstructionId::MoveS).unwrap();
    program[7] = Instruction::new_simple(InstructionId::MoveA).unwrap();
    //     looks
    program[8] = Instruction::new_simple(InstructionId::LookA).unwrap();
    program[9] = Instruction::new_simple(InstructionId::LookD).unwrap();
    program[10] = Instruction::new_simple(InstructionId::LookS).unwrap();
    program[11] = Instruction::new_simple(InstructionId::LookW).unwrap();
    //     one char staff
    program[12] = Instruction::new_simple(InstructionId::Digg).unwrap();
    program[13] = Instruction::new_simple(InstructionId::ActionGeo).unwrap();
    program[14] = Instruction::new_simple(InstructionId::ActionHeal).unwrap();
    program[15] = Instruction::new_simple(InstructionId::ActionRoad).unwrap();
    program[16] = Instruction::new_simple(InstructionId::ActionBuild).unwrap();
    program[17] = Instruction::new_simple(InstructionId::ActionQuadro).unwrap();
    program[18] = Instruction::new_simple(InstructionId::Back).unwrap();
    //     cells
    program[19] = Instruction::new_simple(InstructionId::CellF).unwrap();
    program[20] = Instruction::new_simple(InstructionId::CellW).unwrap();
    program[21] = Instruction::new_simple(InstructionId::CellWa).unwrap();
    program[22] = Instruction::new_simple(InstructionId::CellD).unwrap();
    program[23] = Instruction::new_simple(InstructionId::CellDw).unwrap();
    program[24] = Instruction::new_simple(InstructionId::CellS).unwrap();
    program[25] = Instruction::new_simple(InstructionId::CellSd).unwrap();
    program[26] = Instruction::new_simple(InstructionId::CellA).unwrap();
    program[27] = Instruction::new_simple(InstructionId::CellAs).unwrap();
    program[28] = Instruction::new_simple(InstructionId::CellRightHand).unwrap();
    program[29] = Instruction::new_simple(InstructionId::CellLeftHand).unwrap();
    program[30] = Instruction::new_simple(InstructionId::CellFf).unwrap();
    program[31] = Instruction::new_simple(InstructionId::CellWw).unwrap();
    program[32] = Instruction::new_simple(InstructionId::CellDd).unwrap();
    program[33] = Instruction::new_simple(InstructionId::CellSs).unwrap();
    program[34] = Instruction::new_simple(InstructionId::CellAa).unwrap();
    //     cc
    program[35] = Instruction::new_simple(InstructionId::CcGun).unwrap();
    program[36] = Instruction::new_simple(InstructionId::CcNotEmpty).unwrap();
    program[37] = Instruction::new_simple(InstructionId::CcEmpty).unwrap();
    program[38] = Instruction::new_simple(InstructionId::CcGravity).unwrap();
    program[39] = Instruction::new_simple(InstructionId::CcCrystall).unwrap();
    program[40] = Instruction::new_simple(InstructionId::CcAlive).unwrap();
    program[41] = Instruction::new_simple(InstructionId::CcBolder).unwrap();
    program[42] = Instruction::new_simple(InstructionId::CcSand).unwrap();
    program[43] = Instruction::new_simple(InstructionId::CcRock).unwrap();
    program[44] = Instruction::new_simple(InstructionId::CcDead).unwrap();
    program[45] = Instruction::new_simple(InstructionId::CcAcid).unwrap();
    //     ccc
    program[46] = Instruction::new_simple(InstructionId::CccBlackRock).unwrap();
    program[47] = Instruction::new_simple(InstructionId::CccRedRock).unwrap();
    program[48] = Instruction::new_simple(InstructionId::CccGreenBlock).unwrap();
    program[49] = Instruction::new_simple(InstructionId::CccYellowBlock).unwrap();
    program[50] = Instruction::new_simple(InstructionId::CccRedBlock).unwrap();
    program[51] = Instruction::new_simple(InstructionId::CccOpor).unwrap();
    program[52] = Instruction::new_simple(InstructionId::CccQuadro).unwrap();
    program[53] = Instruction::new_simple(InstructionId::CccBox).unwrap();
    program[54] = Instruction::new_simple(InstructionId::CccRoad).unwrap();
    //     cb_hp
    program[55] = Instruction::new_simple(InstructionId::CbHp50).unwrap();
    program[56] = Instruction::new_simple(InstructionId::CbHp).unwrap();
    //     start & end
    program[57] = Instruction::new_simple(InstructionId::Start).unwrap();
    program[58] = Instruction::new_simple(InstructionId::End).unwrap();
    //     actions
    program[59] = Instruction::new_simple(InstructionId::ActionB1).unwrap();
    program[60] = Instruction::new_simple(InstructionId::ActionB2).unwrap();
    program[61] = Instruction::new_simple(InstructionId::ActionB3).unwrap();
    program[62] = Instruction::new_simple(InstructionId::ActionBibika).unwrap();
    program[63] = Instruction::new_simple(InstructionId::ActionRandom).unwrap();
    program[64] = Instruction::new_simple(InstructionId::ActionWb).unwrap();
    program[65] = Instruction::new_simple(InstructionId::ActionGeopack).unwrap();
    program[66] = Instruction::new_simple(InstructionId::ActionZm).unwrap();
    program[67] = Instruction::new_simple(InstructionId::ActionPoly).unwrap();
    program[68] = Instruction::new_simple(InstructionId::ActionC190).unwrap();
    program[69] = Instruction::new_simple(InstructionId::ActionCraft).unwrap();
    program[70] = Instruction::new_simple(InstructionId::ActionUp).unwrap();
    program[71] = Instruction::new_simple(InstructionId::ActionNano).unwrap();
    program[72] = Instruction::new_simple(InstructionId::ActionRembot).unwrap();
    //     std
    program[73] = Instruction::new_simple(InstructionId::StdBuild).unwrap();
    program[74] = Instruction::new_simple(InstructionId::StdDigg).unwrap();
    program[75] = Instruction::new_simple(InstructionId::StdHeal).unwrap();
    program[76] = Instruction::new_simple(InstructionId::StdMine).unwrap();
    //     mode
    program[77] = Instruction::new_simple(InstructionId::ModeAutodiggOn).unwrap();
    program[78] = Instruction::new_simple(InstructionId::ModeAutodiggOff).unwrap();
    program[79] = Instruction::new_simple(InstructionId::ModeAgrOn).unwrap();
    program[80] = Instruction::new_simple(InstructionId::ModeAgrOff).unwrap();
    //     bool_mode
    program[81] = Instruction::new_simple(InstructionId::BoolModeAnd).unwrap();
    program[82] = Instruction::new_simple(InstructionId::BoolModeOr).unwrap();
    //     rotates
    program[83] = Instruction::new_simple(InstructionId::RotateCcw).unwrap();
    program[84] = Instruction::new_simple(InstructionId::RotateCw).unwrap();
    //     prog_flip
    program[85] = Instruction::new_simple(InstructionId::ProgFlip).unwrap();
    //     fill_gun
    program[86] = Instruction::new_simple(InstructionId::FillGun).unwrap();
    //     inv
    program[87] = Instruction::new_simple(InstructionId::InvDirA).unwrap();
    program[88] = Instruction::new_simple(InstructionId::InvDirD).unwrap();
    program[89] = Instruction::new_simple(InstructionId::InvDirS).unwrap();
    program[90] = Instruction::new_simple(InstructionId::InvDirW).unwrap();
    //     hand
    program[91] = Instruction::new_simple(InstructionId::HandModeOn).unwrap();
    program[92] = Instruction::new_simple(InstructionId::HandModeOff).unwrap();
    //     for tail check
    program[93] = Instruction::new_simple(InstructionId::Return).unwrap();
    //
    program
}

/// Returns a program for testing commands.
pub fn commands() -> Program {
    let mut program = Program::default();
    program[InstructionPosition::new(0, 0, 0).unwrap()] =
        Instruction::new_simple(InstructionId::MoveW).unwrap();
    program[InstructionPosition::new(0, 1, 0).unwrap()] =
        Instruction::new_simple(InstructionId::MoveA).unwrap();
    program[InstructionPosition::new(1, 0, 0).unwrap()] =
        Instruction::new_simple(InstructionId::MoveD).unwrap();
    program[InstructionPosition::new(1, 0, 4).unwrap()] =
        Instruction::new_simple(InstructionId::MoveF).unwrap();
    program[InstructionPosition::new(1, 0, 6).unwrap()] =
        Instruction::new_simple(InstructionId::MoveS).unwrap();
    program
}

/// Returns a program for testing all not-`Simple` kind instruction.
pub fn literals() -> Program {
    let mut program = Program::default();
    program[0] = Instruction::new_label(
        InstructionId::Label,
        LabelIdentifierLiteral::new_from_array([0; 4]).unwrap(),
    )
    .unwrap();
    program[1] = Instruction::new_label(
        InstructionId::Label,
        LabelIdentifierLiteral::new_from_array([b'h', b'i', 0, 0]).unwrap(),
    )
    .unwrap();
    program[2] = Instruction::new_label(
        InstructionId::Label,
        LabelIdentifierLiteral::new_from_array([b'0', b'1', b'2', 0]).unwrap(),
    )
    .unwrap();
    //         go
    program[3] = Instruction::new_label(
        InstructionId::GoTo,
        LabelIdentifierLiteral::new_from_array([b'a', b'b', b'c', 0]).unwrap(),
    )
    .unwrap();
    program[4] = Instruction::new_label(
        InstructionId::GoSub,
        LabelIdentifierLiteral::new_from_array([b'z', b'x', b'c', 0]).unwrap(),
    )
    .unwrap();
    program[5] = Instruction::new_label(
        InstructionId::GoSub1,
        LabelIdentifierLiteral::new_from_array([b's', b'1', b'2', 0]).unwrap(),
    )
    .unwrap();
    program[6] = Instruction::new_label(
        InstructionId::GoSubF,
        LabelIdentifierLiteral::new_from_array([b's', b'b', b'f', 0]).unwrap(),
    )
    .unwrap();
    //         if[not]goto
    program[7] = Instruction::new_label(
        InstructionId::IfGoTo,
        LabelIdentifierLiteral::new_from_array([b'i', b'f', 0, 0]).unwrap(),
    )
    .unwrap();
    program[8] = Instruction::new_label(
        InstructionId::IfNotGoTo,
        LabelIdentifierLiteral::new_from_array([b'i', b'f', b'n', 0]).unwrap(),
    )
    .unwrap();
    //         resp
    program[9] = Instruction::new_label(
        InstructionId::OnResp,
        LabelIdentifierLiteral::new_from_array([b'r', b's', b'p', 0]).unwrap(),
    )
    .unwrap();
    //         var_cmp
    program[10] = Instruction::new_var_cmp(
        InstructionId::VarLess,
        VariableIdentifierLiteral::new_from_array([b'v', b'a', b'0', 0]).unwrap(),
        VariableValueLiteral::new_from_value(0).unwrap(),
    )
    .unwrap();
    program[11] = Instruction::new_var_cmp(
        InstructionId::VarEqual,
        VariableIdentifierLiteral::new_from_array([b'a', 0, 0, 0]).unwrap(),
        VariableValueLiteral::new_from_value(99999).unwrap(),
    )
    .unwrap();
    program[12] = Instruction::new_var_cmp(
        InstructionId::VarMore,
        VariableIdentifierLiteral::new_from_array([b'v', b'a', b'2', 0]).unwrap(),
        VariableValueLiteral::new_from_value(-5).unwrap(),
    )
    .unwrap();
    program[13] = Instruction::new_string(
        InstructionId::DebugSet,
        StringLiteral::new_from_array([b'd', b's', b't', 0]).unwrap(),
    )
    .unwrap();
    program[14] = Instruction::new_string(
        InstructionId::DebugBreak,
        StringLiteral::new_from_array([b'b', b'p', 0, 0]).unwrap(),
    )
    .unwrap();
    program
}

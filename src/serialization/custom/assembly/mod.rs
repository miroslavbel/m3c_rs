//! Serializer and deserializer for Assembly format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use crate::formats::internal::InstructionId;

static FULLY_EMPTY_STRING: &str = "";

static INSTRUCTIONS_NAMES: [&str; 183] = [
    "EMPTY",
    "BACK",
    "START",
    "END",
    "MOVE_W",
    "MOVE_A",
    "MOVE_S",
    "MOVE_D",
    "DIGG",
    "LOOK_W",
    "LOOK_A",
    "LOOK_S",
    "LOOK_D",
    FULLY_EMPTY_STRING, // 13, `LAST`
    "MOVE_F",
    "ROTATE_CCW",
    "ROTATE_CW",
    "ACTION_BUILD",
    "ACTION_GEO",
    "ACTION_ROAD",
    "ACTION_HEAL",
    "ACTION_QUADRO",
    "ACTION_RANDOM",
    "ACTION_BIBIKA",
    "GOTO",
    "GOSUB",
    "GOSUB1",
    "RETURN",
    "RETURN1",
    "CELL_WA",
    "CELL_SD",
    "CELL_W",
    "CELL_DW",
    "CELL_A",
    FULLY_EMPTY_STRING, // 34
    "CELL_D",
    "CELL_AS",
    "CELL_S",
    "BOOLMODE_OR",
    "BOOLMODE_AND",
    "LABEL",
    FULLY_EMPTY_STRING, // 41
    FULLY_EMPTY_STRING, // 42
    "CC_NOTEMPTY",
    "CC_EMPTY",
    "CC_GRAVITY",
    "CC_CRYSTALL",
    "CC_ALIVE",
    "CC_BOLDER",
    "CC_SAND",
    "CC_ROCK",
    "CC_DEAD",
    "CCC_REDROCK",
    "CCC_BLACKROCK",
    "CC_ACID",
    FULLY_EMPTY_STRING, // 55
    FULLY_EMPTY_STRING, // 56
    "CCC_QUADRO",
    "CCC_ROAD",
    "CCC_REDBLOCK",
    "CCC_YELLOWBLOCK",
    FULLY_EMPTY_STRING, // 61
    FULLY_EMPTY_STRING, // 62
    FULLY_EMPTY_STRING, // 63
    FULLY_EMPTY_STRING, // 64
    FULLY_EMPTY_STRING, // 65
    FULLY_EMPTY_STRING, // 66
    FULLY_EMPTY_STRING, // 67
    FULLY_EMPTY_STRING, // 68
    FULLY_EMPTY_STRING, // 69
    FULLY_EMPTY_STRING, // 70
    FULLY_EMPTY_STRING, // 71
    FULLY_EMPTY_STRING, // 72
    FULLY_EMPTY_STRING, // 73
    "CCC_BOX",
    FULLY_EMPTY_STRING, // 75
    "CCC_OPOR",
    "CCC_GREENBLOCK",
    FULLY_EMPTY_STRING, // 78
    FULLY_EMPTY_STRING, // 79
    FULLY_EMPTY_STRING, // 80
    FULLY_EMPTY_STRING, // 81
    FULLY_EMPTY_STRING, // 82
    FULLY_EMPTY_STRING, // 83
    FULLY_EMPTY_STRING, // 84
    FULLY_EMPTY_STRING, // 85
    FULLY_EMPTY_STRING, // 86
    FULLY_EMPTY_STRING, // 87
    FULLY_EMPTY_STRING, // 88
    FULLY_EMPTY_STRING, // 89
    FULLY_EMPTY_STRING, // 90
    FULLY_EMPTY_STRING, // 91
    FULLY_EMPTY_STRING, // 92
    FULLY_EMPTY_STRING, // 93
    FULLY_EMPTY_STRING, // 94
    FULLY_EMPTY_STRING, // 95
    FULLY_EMPTY_STRING, // 96
    FULLY_EMPTY_STRING, // 97
    FULLY_EMPTY_STRING, // 98
    FULLY_EMPTY_STRING, // 99
    FULLY_EMPTY_STRING, // 100
    FULLY_EMPTY_STRING, // 101
    FULLY_EMPTY_STRING, // 102
    FULLY_EMPTY_STRING, // 103
    FULLY_EMPTY_STRING, // 104
    FULLY_EMPTY_STRING, // 105
    FULLY_EMPTY_STRING, // 106
    FULLY_EMPTY_STRING, // 107
    FULLY_EMPTY_STRING, // 108
    FULLY_EMPTY_STRING, // 109
    FULLY_EMPTY_STRING, // 110
    FULLY_EMPTY_STRING, // 111
    FULLY_EMPTY_STRING, // 112
    FULLY_EMPTY_STRING, // 113
    FULLY_EMPTY_STRING, // 114
    FULLY_EMPTY_STRING, // 115
    FULLY_EMPTY_STRING, // 116
    FULLY_EMPTY_STRING, // 117
    FULLY_EMPTY_STRING, // 118
    "VAR_MORE",
    "VAR_LESS",
    FULLY_EMPTY_STRING, // 121
    FULLY_EMPTY_STRING, // 122
    "VAR_EQUAL",
    FULLY_EMPTY_STRING, // 124
    FULLY_EMPTY_STRING, // 125
    FULLY_EMPTY_STRING, // 126
    FULLY_EMPTY_STRING, // 127
    FULLY_EMPTY_STRING, // 128
    FULLY_EMPTY_STRING, // 129
    FULLY_EMPTY_STRING, // 130
    "CELL_WW",
    "CELL_AA",
    "CELL_SS",
    "CELL_DD",
    "CELL_F",
    "CELL_FF",
    "GOSUBF",
    "RETURNF",
    "IF_NOT_GOTO",
    "IF_GOTO",
    "STD_DIGG",
    "STD_BUILD",
    "STD_HEAL",
    "PROG_FLIP",
    "STD_MINE",
    "CC_GUN",
    "FILL_GUN",
    "CB_HP",
    "CB_HP50",
    FULLY_EMPTY_STRING, // 150
    FULLY_EMPTY_STRING, // 151
    FULLY_EMPTY_STRING, // 152
    FULLY_EMPTY_STRING, // 153
    FULLY_EMPTY_STRING, // 154
    FULLY_EMPTY_STRING, // 155
    "CELL_RIGHT_HAND",
    "CELL_LEFT_HAND",
    "MODE_AUTODIGG_ON",
    "MODE_AUTODIGG_OFF",
    "MODE_AGR_ON",
    "MODE_AGR_OFF",
    "ACTION_B1",
    "ACTION_B3",
    "ACTION_B2",
    "ACTION_WB",
    "ON_RESP",
    "ACTION_GEOPACK",
    "ACTION_ZM",
    "ACTION_C190",
    "ACTION_POLY",
    "ACTION_UP",
    "ACTION_CRAFT",
    "ACTION_NANO",
    "ACTION_REMBOT",
    "INVDIR_W",
    "INVDIR_A",
    "INVDIR_S",
    "INVDIR_D",
    "HANDMODE_ON",
    "HANDMODE_OFF",
    "DEBUG_BREAK",
    "DEBUG_SET",
    // 200, `RESTART`
];

impl InstructionId {
    /// Returns the identifier from the native client for this [`InstructionId`].
    fn client_identifier(self) -> &'static str {
        INSTRUCTIONS_NAMES[self as usize]
    }
}

#[cfg(test)]
mod tests {
    use crate::formats::internal::InstructionId;

    #[test]
    fn instruction_id_client_identifier() {
        assert_eq!("EMPTY", InstructionId::Empty.client_identifier());
        assert_eq!("LOOK_D", InstructionId::LookD.client_identifier());
        assert_eq!("MOVE_F", InstructionId::MoveF.client_identifier());
        assert_eq!("CELL_A", InstructionId::CellA.client_identifier());
        assert_eq!("CELL_D", InstructionId::CellD.client_identifier());
        assert_eq!("LABEL", InstructionId::Label.client_identifier());
        assert_eq!("CC_NOTEMPTY", InstructionId::CcNotEmpty.client_identifier());
        assert_eq!("CC_ACID", InstructionId::CcAcid.client_identifier());
        assert_eq!("CCC_QUADRO", InstructionId::CccQuadro.client_identifier());
        assert_eq!(
            "CCC_YELLOWBLOCK",
            InstructionId::CccYellowBlock.client_identifier()
        );
        assert_eq!("CCC_BOX", InstructionId::CccBox.client_identifier());
        assert_eq!("CCC_OPOR", InstructionId::CccOpor.client_identifier());
        assert_eq!(
            "CCC_GREENBLOCK",
            InstructionId::CccGreenBlock.client_identifier()
        );
        assert_eq!("VAR_MORE", InstructionId::VarMore.client_identifier());
        assert_eq!("VAR_LESS", InstructionId::VarLess.client_identifier());
        assert_eq!("VAR_EQUAL", InstructionId::VarEqual.client_identifier());
        assert_eq!("CELL_WW", InstructionId::CellWw.client_identifier());
        assert_eq!("CB_HP50", InstructionId::CbHp50.client_identifier());
        assert_eq!(
            "CELL_RIGHT_HAND",
            InstructionId::CellRightHand.client_identifier()
        );
        assert_eq!("DEBUG_SET", InstructionId::DebugSet.client_identifier());
    }
}

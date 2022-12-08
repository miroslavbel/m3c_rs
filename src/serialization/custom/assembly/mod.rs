//! Serializer and deserializer for Assembly format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::io;

use crate::formats::internal::{
    literals::Literal, Instruction, InstructionData, InstructionId, InstructionPosition, Program,
};

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
    /// Writes the identifier from the native client for this [`InstructionId`] to the given
    /// `writer`.
    fn write<W>(self, writer: &mut W) -> io::Result<()>
    where
        W: io::Write,
    {
        writer.write_all(INSTRUCTIONS_NAMES[self as usize].as_bytes())
    }
}

impl Instruction {
    /// Dumps this instruction to the given `String`.
    ///
    /// The returned string will be prefixed by the given `indent` if this instruction
    /// [id](InstructionId) is not equal to [`Label`](InstructionId::Label).
    fn dumps_to(&self, s: &mut String, indent: &str) {
        let id = self.id();
        let data = self.data();
        if id == InstructionId::Label {
            match data {
                InstructionData::Label(label) => {
                    label.dumps_to(s);
                    s.push(':');
                }
                _ => unreachable!(),
            }
        } else {
            s.push_str(indent);
            s.push_str(id.client_identifier());
            match data {
                InstructionData::Simple => {}
                InstructionData::Label(label) => {
                    s.push(' ');
                    label.dumps_to(s);
                }
                InstructionData::String(string_literal) => {
                    s.push_str(" '");
                    string_literal.dumps_to(s);
                    s.push('\'');
                }
                InstructionData::VarCmp((identifier, value)) => {
                    s.push(' ');
                    identifier.dumps_to(s);
                    s.push_str(", ");
                    value.dumps_to(s);
                }
            }
        }
    }
}

impl InstructionPosition {
    /// Dumps this position to the given `String`.
    fn dumps_to(self, s: &mut String, hide_column: bool) {
        let page = self.page();
        let row = self.row();
        if page < 10 {
            s.push(' ');
        }
        s.push_str(page.to_string().as_str());
        s.push(':');
        if row < 10 {
            s.push(' ');
        }
        s.push_str(row.to_string().as_str());
        if !hide_column {
            let column = self.column();
            s.push(':');
            if column < 10 {
                s.push(' ');
            }
            s.push_str(column.to_string().as_str());
        }
    }
}

/// A structure for serializing [`Program`] into human-readable assembly-like format.
#[derive(Debug, Clone, Copy)]
pub struct Serializer<'p> {
    program: &'p Program,
}

impl<'p> Serializer<'p> {
    #[cfg(windows)]
    const LINE_SEPARATOR: &'static str = "\r\n";
    #[cfg(not(windows))]
    const LINE_SEPARATOR: &'static str = "\n";
    const NEW_PAGE_WARN: &'static str = "the new PAGE started below";
    const NEW_ROW_WARN: &'static str = "the new ROW started below";
    /// Creates a new [`Serializer`] from a `&Program`.
    #[must_use]
    pub fn new(program: &'p Program) -> Self {
        Self { program }
    }
    /// Serializes to the given `String` with the given `indent`.
    pub fn serialize_to(&self, s: &mut String, indent: &str) {
        let mut instruction_positions = self.program.instruction_positions();

        // don't write that this's beginnig of the page (and a whole program)
        let first_elem = instruction_positions.next();
        match first_elem {
            Some((_, ins)) => {
                ins.dumps_to(s, indent);
                s.push_str(Self::LINE_SEPARATOR);
            }
            None => unreachable!("The program should always have the first instruction"),
        }

        for (position, instruction) in instruction_positions {
            if position.column() == 0 {
                s.push_str(Self::LINE_SEPARATOR);
                s.push_str("; (");
                position.dumps_to(s, true);
                s.push_str(") ");
                if position.row() == 0 {
                    s.push_str(Self::NEW_PAGE_WARN);
                } else {
                    s.push_str(Self::NEW_ROW_WARN);
                }
                s.push_str(Self::LINE_SEPARATOR);
            }
            instruction.dumps_to(s, indent);
            s.push_str(Self::LINE_SEPARATOR);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::formats::internal::literals::{
        LabelIdentifierLiteral, VariableIdentifierLiteral, VariableValueLiteral,
    };
    use crate::formats::internal::{Instruction, InstructionId, InstructionPosition};

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

    #[test]
    fn instruction_dumps_to() {
        let mut s = String::new();
        let indent = "    ";

        Instruction::new_simple(InstructionId::MoveA)
            .unwrap()
            .dumps_to(&mut s, indent);
        s.push('\n');
        Instruction::new_label(
            InstructionId::Label,
            LabelIdentifierLiteral::new_from_array([b'l', b'a', b'b', 0]).unwrap(),
        )
        .unwrap()
        .dumps_to(&mut s, indent);
        s.push('\n');
        Instruction::new_simple(InstructionId::MoveW)
            .unwrap()
            .dumps_to(&mut s, indent);
        s.push('\n');
        Instruction::new_label(
            InstructionId::Label,
            LabelIdentifierLiteral::new_from_array([0, 0, 0, 0]).unwrap(),
        )
        .unwrap()
        .dumps_to(&mut s, indent);
        s.push('\n');
        Instruction::new_simple(InstructionId::CcCrystall)
            .unwrap()
            .dumps_to(&mut s, indent);
        s.push('\n');
        Instruction::new_label(
            InstructionId::IfGoTo,
            LabelIdentifierLiteral::new_from_array([b'l', b'a', b'b', 0]).unwrap(),
        )
        .unwrap()
        .dumps_to(&mut s, indent);
        s.push('\n');
        Instruction::new_var_cmp(
            InstructionId::VarLess,
            VariableIdentifierLiteral::new_from_array([b'x', 0, 0, 0]).unwrap(),
            VariableValueLiteral::new_from_value(42).unwrap(),
        )
        .unwrap()
        .dumps_to(&mut s, indent);
        s.push('\n');
        // TODO check string kind too
        assert_eq!(
            format_args!(
                concat!(
                    "{indent}MOVE_A\n",
                    "lab:\n",
                    "{indent}MOVE_W\n",
                    ":\n",
                    "{indent}CC_CRYSTALL\n",
                    "{indent}IF_GOTO lab\n",
                    "{indent}VAR_LESS x, 42\n",
                ),
                indent = indent
            )
            .to_string(),
            s
        );
    }

    #[test]
    fn instruction_position_dumps_to() {
        let mut s = String::new();

        let ip1 = InstructionPosition::new(1, 2, 3).unwrap();
        let ip2 = InstructionPosition::new(10, 11, 12).unwrap();

        ip1.dumps_to(&mut s, false);
        ip2.dumps_to(&mut s, false);
        ip2.dumps_to(&mut s, true);

        assert_eq!(concat!(" 1: 2: 3", "10:11:12", "10:11"), s);
    }
}

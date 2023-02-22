use crate::formats::internal::literals::LiteralType;
use crate::formats::internal::InstructionId;

use super::Command;

#[derive(Debug, Clone, Copy)]
pub(super) enum I2NTFNode {
    Chars(&'static [u8]),
    Literal(LiteralType),
}

#[derive(Debug, Clone, Copy)]
pub(super) enum NTF2INode {
    Command(Command),
    Id(InstructionId),
    Chars(&'static [(char, Self)]),
    Literal(&'static (LiteralType, &'static [Self])),
}

/// Maps [`InstructionId`] to the New Text Format.
pub(super) static I2NTF: [(InstructionId, &'static [I2NTFNode]); 107] = [
    // DO NOT EDIT. THE DATA IS SORTED.
    (InstructionId::Empty, &[I2NTFNode::Chars(b" ")]),
    (InstructionId::Back, &[I2NTFNode::Chars(b",")]),
    (InstructionId::Start, &[I2NTFNode::Chars(b"#S")]),
    (InstructionId::End, &[I2NTFNode::Chars(b"#E")]),
    (InstructionId::MoveW, &[I2NTFNode::Chars(b"^W")]),
    (InstructionId::MoveA, &[I2NTFNode::Chars(b"^A")]),
    (InstructionId::MoveS, &[I2NTFNode::Chars(b"^S")]),
    (InstructionId::MoveD, &[I2NTFNode::Chars(b"^D")]),
    (InstructionId::Digg, &[I2NTFNode::Chars(b"z")]),
    (InstructionId::LookW, &[I2NTFNode::Chars(b"w")]),
    (InstructionId::LookA, &[I2NTFNode::Chars(b"a")]),
    (InstructionId::LookS, &[I2NTFNode::Chars(b"s")]),
    (InstructionId::LookD, &[I2NTFNode::Chars(b"d")]),
    (InstructionId::MoveF, &[I2NTFNode::Chars(b"^F")]),
    (InstructionId::RotateCcw, &[I2NTFNode::Chars(b"CCW;")]),
    (InstructionId::RotateCw, &[I2NTFNode::Chars(b"^CW;")]),
    (InstructionId::ActionBuild, &[I2NTFNode::Chars(b"b")]),
    (InstructionId::ActionGeo, &[I2NTFNode::Chars(b"g")]),
    (InstructionId::ActionRoad, &[I2NTFNode::Chars(b"r")]),
    (InstructionId::ActionHeal, &[I2NTFNode::Chars(b"h")]),
    (InstructionId::ActionQuadro, &[I2NTFNode::Chars(b"q")]),
    (InstructionId::ActionRandom, &[I2NTFNode::Chars(b"RAND;")]),
    (InstructionId::ActionBibika, &[I2NTFNode::Chars(b"BEEP;")]),
    (
        InstructionId::GoTo,
        &[
            I2NTFNode::Chars(b">"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b"|"),
        ],
    ),
    (
        InstructionId::GoSub,
        &[
            I2NTFNode::Chars(b":>"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b">"),
        ],
    ),
    (
        InstructionId::GoSub1,
        &[
            I2NTFNode::Chars(b"->"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b">"),
        ],
    ),
    (InstructionId::Return, &[I2NTFNode::Chars(b"<|")]),
    (InstructionId::Return1, &[I2NTFNode::Chars(b"<-|")]),
    (InstructionId::CellWa, &[I2NTFNode::Chars(b"[WA]")]),
    (InstructionId::CellSd, &[I2NTFNode::Chars(b"[SD]")]),
    (InstructionId::CellW, &[I2NTFNode::Chars(b"[W]")]),
    (InstructionId::CellDw, &[I2NTFNode::Chars(b"[DW]")]),
    (InstructionId::CellA, &[I2NTFNode::Chars(b"[A]")]),
    (InstructionId::CellD, &[I2NTFNode::Chars(b"[D]")]),
    (InstructionId::CellAs, &[I2NTFNode::Chars(b"[AS]")]),
    (InstructionId::CellS, &[I2NTFNode::Chars(b"[S]")]),
    (InstructionId::BoolModeOr, &[I2NTFNode::Chars(b"OR")]),
    (InstructionId::BoolModeAnd, &[I2NTFNode::Chars(b"AND")]),
    (
        InstructionId::Label,
        &[
            I2NTFNode::Chars(b"|"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b":"),
        ],
    ),
    (InstructionId::CcNotEmpty, &[I2NTFNode::Chars(b"=n")]),
    (InstructionId::CcEmpty, &[I2NTFNode::Chars(b"=e")]),
    (InstructionId::CcGravity, &[I2NTFNode::Chars(b"=f")]),
    (InstructionId::CcCrystall, &[I2NTFNode::Chars(b"=c")]),
    (InstructionId::CcAlive, &[I2NTFNode::Chars(b"=a")]),
    (InstructionId::CcBolder, &[I2NTFNode::Chars(b"=b")]),
    (InstructionId::CcSand, &[I2NTFNode::Chars(b"=s")]),
    (InstructionId::CcRock, &[I2NTFNode::Chars(b"=k")]),
    (InstructionId::CcDead, &[I2NTFNode::Chars(b"=d")]),
    (InstructionId::CccRedRock, &[I2NTFNode::Chars(b"=K")]),
    (InstructionId::CccBlackRock, &[I2NTFNode::Chars(b"=B")]),
    (InstructionId::CcAcid, &[I2NTFNode::Chars(b"=A")]),
    (InstructionId::CccQuadro, &[I2NTFNode::Chars(b"=q")]),
    (InstructionId::CccRoad, &[I2NTFNode::Chars(b"=R")]),
    (InstructionId::CccRedBlock, &[I2NTFNode::Chars(b"=r")]),
    (InstructionId::CccYellowBlock, &[I2NTFNode::Chars(b"=y")]),
    (InstructionId::CccBox, &[I2NTFNode::Chars(b"=x")]),
    (InstructionId::CccOpor, &[I2NTFNode::Chars(b"=o")]),
    (InstructionId::CccGreenBlock, &[I2NTFNode::Chars(b"=g")]),
    (
        InstructionId::VarMore,
        &[
            I2NTFNode::Chars(b"("),
            I2NTFNode::Literal(LiteralType::VariableIdentifierLiteral),
            I2NTFNode::Chars(b">"),
            I2NTFNode::Literal(LiteralType::VariableValueLiteral),
            I2NTFNode::Chars(b")"),
        ],
    ),
    (
        InstructionId::VarLess,
        &[
            I2NTFNode::Chars(b"("),
            I2NTFNode::Literal(LiteralType::VariableIdentifierLiteral),
            I2NTFNode::Chars(b"<"),
            I2NTFNode::Literal(LiteralType::VariableValueLiteral),
            I2NTFNode::Chars(b")"),
        ],
    ),
    (
        InstructionId::VarEqual,
        &[
            I2NTFNode::Chars(b"("),
            I2NTFNode::Literal(LiteralType::VariableIdentifierLiteral),
            I2NTFNode::Chars(b"="),
            I2NTFNode::Literal(LiteralType::VariableValueLiteral),
            I2NTFNode::Chars(b")"),
        ],
    ),
    (InstructionId::CellWw, &[I2NTFNode::Chars(b"[w]")]),
    (InstructionId::CellAa, &[I2NTFNode::Chars(b"[a]")]),
    (InstructionId::CellSs, &[I2NTFNode::Chars(b"[s]")]),
    (InstructionId::CellDd, &[I2NTFNode::Chars(b"[d]")]),
    (InstructionId::CellF, &[I2NTFNode::Chars(b"[F]")]),
    (InstructionId::CellFf, &[I2NTFNode::Chars(b"[f]")]),
    (
        InstructionId::GoSubF,
        &[
            I2NTFNode::Chars(b"=>"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b">"),
        ],
    ),
    (InstructionId::ReturnF, &[I2NTFNode::Chars(b"<=|")]),
    (
        InstructionId::IfNotGoTo,
        &[
            I2NTFNode::Chars(b"?"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b"<"),
        ],
    ),
    (
        InstructionId::IfGoTo,
        &[
            I2NTFNode::Chars(b"!?"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b"<"),
        ],
    ),
    (InstructionId::StdDigg, &[I2NTFNode::Chars(b"DIGG;")]),
    (InstructionId::StdBuild, &[I2NTFNode::Chars(b"BUILD;")]),
    (InstructionId::StdHeal, &[I2NTFNode::Chars(b"HEAL;")]),
    (InstructionId::ProgFlip, &[I2NTFNode::Chars(b"FLIP;")]),
    (InstructionId::StdMine, &[I2NTFNode::Chars(b"MINE;")]),
    (InstructionId::CcGun, &[I2NTFNode::Chars(b"=G")]),
    (InstructionId::FillGun, &[I2NTFNode::Chars(b"FILL;")]),
    (InstructionId::CbHp, &[I2NTFNode::Chars(b"=hp-")]),
    (InstructionId::CbHp50, &[I2NTFNode::Chars(b"=hp50")]),
    (InstructionId::CellRightHand, &[I2NTFNode::Chars(b"[r]")]),
    (InstructionId::CellLeftHand, &[I2NTFNode::Chars(b"[l]")]),
    (InstructionId::ModeAutodiggOn, &[I2NTFNode::Chars(b"AUT+")]),
    (InstructionId::ModeAutodiggOff, &[I2NTFNode::Chars(b"AUT-")]),
    (InstructionId::ModeAgrOn, &[I2NTFNode::Chars(b"AGR+")]),
    (InstructionId::ModeAgrOff, &[I2NTFNode::Chars(b"AGR-")]),
    (InstructionId::ActionB1, &[I2NTFNode::Chars(b"B1;")]),
    (InstructionId::ActionB3, &[I2NTFNode::Chars(b"B2;")]),
    (InstructionId::ActionB2, &[I2NTFNode::Chars(b"B3;")]),
    (InstructionId::ActionWb, &[I2NTFNode::Chars(b"VB;")]),
    (
        InstructionId::OnResp,
        &[
            I2NTFNode::Chars(b"#R"),
            I2NTFNode::Literal(LiteralType::LabelIdentifierLiteral),
            I2NTFNode::Chars(b"<"),
        ],
    ),
    (InstructionId::ActionGeopack, &[I2NTFNode::Chars(b"GEO;")]),
    (InstructionId::ActionZm, &[I2NTFNode::Chars(b"ZZ;")]),
    (InstructionId::ActionC190, &[I2NTFNode::Chars(b"C190;")]),
    (InstructionId::ActionPoly, &[I2NTFNode::Chars(b"POLY;")]),
    (InstructionId::ActionUp, &[I2NTFNode::Chars(b"UP;")]),
    (InstructionId::ActionCraft, &[I2NTFNode::Chars(b"CRAFT;")]),
    (InstructionId::ActionNano, &[I2NTFNode::Chars(b"NANO;")]),
    (InstructionId::ActionRembot, &[I2NTFNode::Chars(b"REM;")]),
    (InstructionId::InvDirW, &[I2NTFNode::Chars(b"iw")]),
    (InstructionId::InvDirA, &[I2NTFNode::Chars(b"ia")]),
    (InstructionId::InvDirS, &[I2NTFNode::Chars(b"is")]),
    (InstructionId::InvDirD, &[I2NTFNode::Chars(b"id")]),
    (InstructionId::HandModeOn, &[I2NTFNode::Chars(b"Hand+")]),
    (InstructionId::HandModeOff, &[I2NTFNode::Chars(b"Hand-")]),
    (
        InstructionId::DebugBreak,
        &[
            I2NTFNode::Chars(b"!{"),
            I2NTFNode::Literal(LiteralType::StringLiteral),
            I2NTFNode::Chars(b"}"),
        ],
    ),
    (
        InstructionId::DebugSet,
        &[
            I2NTFNode::Chars(b"{"),
            I2NTFNode::Literal(LiteralType::StringLiteral),
            I2NTFNode::Chars(b"}"),
        ],
    ),
];

/// Maps New Text Format to [`InstructionId`].
pub(super) static NTF2I: [(char, NTF2INode); 44] = [
    // DO NOT EDIT. THE DATA IS SORTED.
    // 0x0A
    ('\n', NTF2INode::Command(Command::GoToNextRow)),
    // 0x20
    (' ', NTF2INode::Command(Command::OneStepForward)),
    // 0x21
    (
        '!',
        NTF2INode::Chars(&[
            // 0x3F
            (
                '?',
                NTF2INode::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[NTF2INode::Chars(&[
                        // 0x3C
                        ('<', NTF2INode::Id(InstructionId::IfGoTo)),
                    ])],
                )),
            ),
            // 0x7B
            (
                '{',
                NTF2INode::Literal(&(
                    LiteralType::StringLiteral,
                    &[NTF2INode::Chars(&[
                        // 0x7D
                        ('}', NTF2INode::Id(InstructionId::DebugBreak)),
                    ])],
                )),
            ),
        ]),
    ),
    // 0x23
    (
        '#',
        NTF2INode::Chars(&[
            // 0x45
            ('E', NTF2INode::Id(InstructionId::End)),
            // 0x52
            (
                'R',
                NTF2INode::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[NTF2INode::Chars(&[
                        // 0x3C
                        ('<', NTF2INode::Id(InstructionId::OnResp)),
                    ])],
                )),
            ),
            // 0x53
            ('S', NTF2INode::Id(InstructionId::Start)),
        ]),
    ),
    // 0x28
    (
        '(',
        NTF2INode::Literal(&(
            LiteralType::VariableIdentifierLiteral,
            &[NTF2INode::Chars(&[
                // 0x3C
                (
                    '<',
                    NTF2INode::Literal(&(
                        LiteralType::VariableValueLiteral,
                        &[NTF2INode::Chars(&[
                            // 0x29
                            (')', NTF2INode::Id(InstructionId::VarLess)),
                        ])],
                    )),
                ),
                // 0x3D
                (
                    '=',
                    NTF2INode::Literal(&(
                        LiteralType::VariableValueLiteral,
                        &[NTF2INode::Chars(&[
                            // 0x29
                            (')', NTF2INode::Id(InstructionId::VarEqual)),
                        ])],
                    )),
                ),
                // 0x3E
                (
                    '>',
                    NTF2INode::Literal(&(
                        LiteralType::VariableValueLiteral,
                        &[NTF2INode::Chars(&[
                            // 0x29
                            (')', NTF2INode::Id(InstructionId::VarMore)),
                        ])],
                    )),
                ),
            ])],
        )),
    ),
    // 0x2C
    (',', NTF2INode::Id(InstructionId::Back)),
    // 0x2D
    (
        '-',
        NTF2INode::Chars(&[
            // 0x3E
            (
                '>',
                NTF2INode::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[NTF2INode::Chars(&[
                        // 0x3E
                        ('>', NTF2INode::Id(InstructionId::GoSub1)),
                    ])],
                )),
            ),
        ]),
    ),
    // 0x3A
    (
        ':',
        NTF2INode::Chars(&[
            // 0x3E
            (
                '>',
                NTF2INode::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[NTF2INode::Chars(&[
                        // 0x3E
                        ('>', NTF2INode::Id(InstructionId::GoSub)),
                    ])],
                )),
            ),
        ]),
    ),
    // 0x3C
    (
        '<',
        NTF2INode::Chars(&[
            // 0x2D
            (
                '-',
                NTF2INode::Chars(&[
                    // 0x7C
                    ('|', NTF2INode::Id(InstructionId::Return1)),
                ]),
            ),
            // 0x3D
            (
                '=',
                NTF2INode::Chars(&[
                    // 0x7C
                    ('|', NTF2INode::Id(InstructionId::ReturnF)),
                ]),
            ),
            // 0x7C
            ('|', NTF2INode::Id(InstructionId::Return)),
        ]),
    ),
    // 0x3D
    (
        '=',
        NTF2INode::Chars(&[
            // 0x3E
            (
                '>',
                NTF2INode::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[NTF2INode::Chars(&[
                        // 0x3E
                        ('>', NTF2INode::Id(InstructionId::GoSubF)),
                    ])],
                )),
            ),
            // 0x41
            ('A', NTF2INode::Id(InstructionId::CcAcid)),
            // 0x42
            ('B', NTF2INode::Id(InstructionId::CccBlackRock)),
            // 0x47
            ('G', NTF2INode::Id(InstructionId::CcGun)),
            // 0x4B
            ('K', NTF2INode::Id(InstructionId::CccRedRock)),
            // 0x52
            ('R', NTF2INode::Id(InstructionId::CccRoad)),
            // 0x61
            ('a', NTF2INode::Id(InstructionId::CcAlive)),
            // 0x62
            ('b', NTF2INode::Id(InstructionId::CcBolder)),
            // 0x63
            ('c', NTF2INode::Id(InstructionId::CcCrystall)),
            // 0x64
            ('d', NTF2INode::Id(InstructionId::CcDead)),
            // 0x65
            ('e', NTF2INode::Id(InstructionId::CcEmpty)),
            // 0x66
            ('f', NTF2INode::Id(InstructionId::CcGravity)),
            // 0x67
            ('g', NTF2INode::Id(InstructionId::CccGreenBlock)),
            // 0x68
            (
                'h',
                NTF2INode::Chars(&[
                    // 0x70
                    (
                        'p',
                        NTF2INode::Chars(&[
                            // 0x2D
                            ('-', NTF2INode::Id(InstructionId::CbHp)),
                            // 0x35
                            (
                                '5',
                                NTF2INode::Chars(&[
                                    // 0x30
                                    ('0', NTF2INode::Id(InstructionId::CbHp50)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x6B
            ('k', NTF2INode::Id(InstructionId::CcRock)),
            // 0x6E
            ('n', NTF2INode::Id(InstructionId::CcNotEmpty)),
            // 0x6F
            ('o', NTF2INode::Id(InstructionId::CccOpor)),
            // 0x71
            ('q', NTF2INode::Id(InstructionId::CccQuadro)),
            // 0x72
            ('r', NTF2INode::Id(InstructionId::CccRedBlock)),
            // 0x73
            ('s', NTF2INode::Id(InstructionId::CcSand)),
            // 0x78
            ('x', NTF2INode::Id(InstructionId::CccBox)),
            // 0x79
            ('y', NTF2INode::Id(InstructionId::CccYellowBlock)),
        ]),
    ),
    // 0x3E
    (
        '>',
        NTF2INode::Literal(&(
            LiteralType::LabelIdentifierLiteral,
            &[NTF2INode::Chars(&[
                // 0x7C
                ('|', NTF2INode::Id(InstructionId::GoTo)),
            ])],
        )),
    ),
    // 0x3F
    (
        '?',
        NTF2INode::Literal(&(
            LiteralType::LabelIdentifierLiteral,
            &[NTF2INode::Chars(&[
                // 0x3C
                ('<', NTF2INode::Id(InstructionId::IfNotGoTo)),
            ])],
        )),
    ),
    // 0x41
    (
        'A',
        NTF2INode::Chars(&[
            // 0x47
            (
                'G',
                NTF2INode::Chars(&[
                    // 0x52
                    (
                        'R',
                        NTF2INode::Chars(&[
                            // 0x2B
                            ('+', NTF2INode::Id(InstructionId::ModeAgrOn)),
                            // 0x2D
                            ('-', NTF2INode::Id(InstructionId::ModeAgrOff)),
                        ]),
                    ),
                ]),
            ),
            // 0x4E
            (
                'N',
                NTF2INode::Chars(&[
                    // 0x44
                    ('D', NTF2INode::Id(InstructionId::BoolModeAnd)),
                ]),
            ),
            // 0x55
            (
                'U',
                NTF2INode::Chars(&[
                    // 0x54
                    (
                        'T',
                        NTF2INode::Chars(&[
                            // 0x2B
                            ('+', NTF2INode::Id(InstructionId::ModeAutodiggOn)),
                            // 0x2D
                            ('-', NTF2INode::Id(InstructionId::ModeAutodiggOff)),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x42
    (
        'B',
        NTF2INode::Chars(&[
            // 0x31
            (
                '1',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::ActionB1)),
                ]),
            ),
            // 0x32
            (
                '2',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::ActionB3)),
                ]),
            ),
            // 0x33
            (
                '3',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::ActionB2)),
                ]),
            ),
            // 0x45
            (
                'E',
                NTF2INode::Chars(&[
                    // 0x45
                    (
                        'E',
                        NTF2INode::Chars(&[
                            // 0x50
                            (
                                'P',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::ActionBibika)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x55
            (
                'U',
                NTF2INode::Chars(&[
                    // 0x49
                    (
                        'I',
                        NTF2INode::Chars(&[
                            // 0x4C
                            (
                                'L',
                                NTF2INode::Chars(&[
                                    // 0x44
                                    (
                                        'D',
                                        NTF2INode::Chars(&[
                                            // 0x3B
                                            (';', NTF2INode::Id(InstructionId::StdBuild)),
                                        ]),
                                    ),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x43
    (
        'C',
        NTF2INode::Chars(&[
            // 0x31
            (
                '1',
                NTF2INode::Chars(&[
                    // 0x39
                    (
                        '9',
                        NTF2INode::Chars(&[
                            // 0x30
                            (
                                '0',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::ActionC190)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x43
            (
                'C',
                NTF2INode::Chars(&[
                    // 0x57
                    (
                        'W',
                        NTF2INode::Chars(&[
                            // 0x3B
                            (';', NTF2INode::Id(InstructionId::RotateCcw)),
                        ]),
                    ),
                ]),
            ),
            // 0x52
            (
                'R',
                NTF2INode::Chars(&[
                    // 0x41
                    (
                        'A',
                        NTF2INode::Chars(&[
                            // 0x46
                            (
                                'F',
                                NTF2INode::Chars(&[
                                    // 0x54
                                    (
                                        'T',
                                        NTF2INode::Chars(&[
                                            // 0x3B
                                            (';', NTF2INode::Id(InstructionId::ActionCraft)),
                                        ]),
                                    ),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x57
            (
                'W',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::RotateCw)),
                ]),
            ),
        ]),
    ),
    // 0x44
    (
        'D',
        NTF2INode::Chars(&[
            // 0x49
            (
                'I',
                NTF2INode::Chars(&[
                    // 0x47
                    (
                        'G',
                        NTF2INode::Chars(&[
                            // 0x47
                            (
                                'G',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::StdDigg)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x46
    (
        'F',
        NTF2INode::Chars(&[
            // 0x49
            (
                'I',
                NTF2INode::Chars(&[
                    // 0x4C
                    (
                        'L',
                        NTF2INode::Chars(&[
                            // 0x4C
                            (
                                'L',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::FillGun)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x4C
            (
                'L',
                NTF2INode::Chars(&[
                    // 0x49
                    (
                        'I',
                        NTF2INode::Chars(&[
                            // 0x50
                            (
                                'P',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::ProgFlip)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x47
    (
        'G',
        NTF2INode::Chars(&[
            // 0x45
            (
                'E',
                NTF2INode::Chars(&[
                    // 0x4F
                    (
                        'O',
                        NTF2INode::Chars(&[
                            // 0x3B
                            (';', NTF2INode::Id(InstructionId::ActionGeopack)),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x48
    (
        'H',
        NTF2INode::Chars(&[
            // 0x45
            (
                'E',
                NTF2INode::Chars(&[
                    // 0x41
                    (
                        'A',
                        NTF2INode::Chars(&[
                            // 0x4C
                            (
                                'L',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::StdHeal)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x61
            (
                'a',
                NTF2INode::Chars(&[
                    // 0x6E
                    (
                        'n',
                        NTF2INode::Chars(&[
                            // 0x64
                            (
                                'd',
                                NTF2INode::Chars(&[
                                    // 0x2B
                                    ('+', NTF2INode::Id(InstructionId::HandModeOn)),
                                    // 0x2D
                                    ('-', NTF2INode::Id(InstructionId::HandModeOff)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x4D
    (
        'M',
        NTF2INode::Chars(&[
            // 0x49
            (
                'I',
                NTF2INode::Chars(&[
                    // 0x4E
                    (
                        'N',
                        NTF2INode::Chars(&[
                            // 0x45
                            (
                                'E',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::StdMine)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x4E
    (
        'N',
        NTF2INode::Chars(&[
            // 0x41
            (
                'A',
                NTF2INode::Chars(&[
                    // 0x4E
                    (
                        'N',
                        NTF2INode::Chars(&[
                            // 0x4F
                            (
                                'O',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::ActionNano)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x4F
    (
        'O',
        NTF2INode::Chars(&[
            // 0x52
            ('R', NTF2INode::Id(InstructionId::BoolModeOr)),
        ]),
    ),
    // 0x50
    (
        'P',
        NTF2INode::Chars(&[
            // 0x4F
            (
                'O',
                NTF2INode::Chars(&[
                    // 0x4C
                    (
                        'L',
                        NTF2INode::Chars(&[
                            // 0x59
                            (
                                'Y',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::ActionPoly)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x52
    (
        'R',
        NTF2INode::Chars(&[
            // 0x41
            (
                'A',
                NTF2INode::Chars(&[
                    // 0x4E
                    (
                        'N',
                        NTF2INode::Chars(&[
                            // 0x44
                            (
                                'D',
                                NTF2INode::Chars(&[
                                    // 0x3B
                                    (';', NTF2INode::Id(InstructionId::ActionRandom)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x45
            (
                'E',
                NTF2INode::Chars(&[
                    // 0x4D
                    (
                        'M',
                        NTF2INode::Chars(&[
                            // 0x3B
                            (';', NTF2INode::Id(InstructionId::ActionRembot)),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x55
    (
        'U',
        NTF2INode::Chars(&[
            // 0x50
            (
                'P',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::ActionUp)),
                ]),
            ),
        ]),
    ),
    // 0x56
    (
        'V',
        NTF2INode::Chars(&[
            // 0x42
            (
                'B',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::ActionWb)),
                ]),
            ),
        ]),
    ),
    // 0x5A
    (
        'Z',
        NTF2INode::Chars(&[
            // 0x5A
            (
                'Z',
                NTF2INode::Chars(&[
                    // 0x3B
                    (';', NTF2INode::Id(InstructionId::ActionZm)),
                ]),
            ),
        ]),
    ),
    // 0x5B
    (
        '[',
        NTF2INode::Chars(&[
            // 0x41
            (
                'A',
                NTF2INode::Chars(&[
                    // 0x53
                    (
                        'S',
                        NTF2INode::Chars(&[
                            // 0x5D
                            (']', NTF2INode::Id(InstructionId::CellAs)),
                        ]),
                    ),
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellA)),
                ]),
            ),
            // 0x44
            (
                'D',
                NTF2INode::Chars(&[
                    // 0x57
                    (
                        'W',
                        NTF2INode::Chars(&[
                            // 0x5D
                            (']', NTF2INode::Id(InstructionId::CellDw)),
                        ]),
                    ),
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellD)),
                ]),
            ),
            // 0x46
            (
                'F',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellF)),
                ]),
            ),
            // 0x53
            (
                'S',
                NTF2INode::Chars(&[
                    // 0x44
                    (
                        'D',
                        NTF2INode::Chars(&[
                            // 0x5D
                            (']', NTF2INode::Id(InstructionId::CellSd)),
                        ]),
                    ),
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellS)),
                ]),
            ),
            // 0x57
            (
                'W',
                NTF2INode::Chars(&[
                    // 0x41
                    (
                        'A',
                        NTF2INode::Chars(&[
                            // 0x5D
                            (']', NTF2INode::Id(InstructionId::CellWa)),
                        ]),
                    ),
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellW)),
                ]),
            ),
            // 0x61
            (
                'a',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellAa)),
                ]),
            ),
            // 0x64
            (
                'd',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellDd)),
                ]),
            ),
            // 0x66
            (
                'f',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellFf)),
                ]),
            ),
            // 0x6C
            (
                'l',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellLeftHand)),
                ]),
            ),
            // 0x72
            (
                'r',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellRightHand)),
                ]),
            ),
            // 0x73
            (
                's',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellSs)),
                ]),
            ),
            // 0x77
            (
                'w',
                NTF2INode::Chars(&[
                    // 0x5D
                    (']', NTF2INode::Id(InstructionId::CellWw)),
                ]),
            ),
        ]),
    ),
    // 0x5E
    (
        '^',
        NTF2INode::Chars(&[
            // 0x41
            ('A', NTF2INode::Id(InstructionId::MoveA)),
            // 0x44
            ('D', NTF2INode::Id(InstructionId::MoveD)),
            // 0x46
            ('F', NTF2INode::Id(InstructionId::MoveF)),
            // 0x53
            ('S', NTF2INode::Id(InstructionId::MoveS)),
            // 0x57
            ('W', NTF2INode::Id(InstructionId::MoveW)),
        ]),
    ),
    // 0x5F
    ('_', NTF2INode::Command(Command::ThreeStepsForward)),
    // 0x61
    ('a', NTF2INode::Id(InstructionId::LookA)),
    // 0x62
    ('b', NTF2INode::Id(InstructionId::ActionBuild)),
    // 0x64
    ('d', NTF2INode::Id(InstructionId::LookD)),
    // 0x67
    ('g', NTF2INode::Id(InstructionId::ActionGeo)),
    // 0x68
    ('h', NTF2INode::Id(InstructionId::ActionHeal)),
    // 0x69
    (
        'i',
        NTF2INode::Chars(&[
            // 0x61
            ('a', NTF2INode::Id(InstructionId::InvDirA)),
            // 0x64
            ('d', NTF2INode::Id(InstructionId::InvDirD)),
            // 0x73
            ('s', NTF2INode::Id(InstructionId::InvDirS)),
            // 0x77
            ('w', NTF2INode::Id(InstructionId::InvDirW)),
        ]),
    ),
    // 0x71
    ('q', NTF2INode::Id(InstructionId::ActionQuadro)),
    // 0x72
    ('r', NTF2INode::Id(InstructionId::ActionRoad)),
    // 0x73
    ('s', NTF2INode::Id(InstructionId::LookS)),
    // 0x77
    ('w', NTF2INode::Id(InstructionId::LookW)),
    // 0x7A
    ('z', NTF2INode::Id(InstructionId::Digg)),
    // 0x7B
    (
        '{',
        NTF2INode::Literal(&(
            LiteralType::StringLiteral,
            &[NTF2INode::Chars(&[
                // 0x7D
                ('}', NTF2INode::Id(InstructionId::DebugSet)),
            ])],
        )),
    ),
    // 0x7C
    (
        '|',
        NTF2INode::Literal(&(
            LiteralType::LabelIdentifierLiteral,
            &[NTF2INode::Chars(&[
                // 0x3A
                (':', NTF2INode::Id(InstructionId::Label)),
            ])],
        )),
    ),
    // 0x7E
    ('~', NTF2INode::Command(Command::GoToNextPage)),
];

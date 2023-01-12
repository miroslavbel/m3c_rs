use crate::formats::internal::literals::LiteralType;
use crate::formats::internal::InstructionId;

use super::Command;

#[derive(Debug, Clone, Copy)]
pub(super) enum Node {
    Command(Command),
    Id(InstructionId),
    Chars(&'static [(char, Self)]),
    Literal(&'static (LiteralType, &'static [Self])),
}

pub(super) static DATA: [(char, Node); 44] = [
    // DO NOT EDIT. THE DATA IS SORTED.
    // 0x0A
    ('\n', Node::Command(Command::GoToNextRow)),
    // 0x20
    (' ', Node::Command(Command::OneStepForward)),
    // 0x21
    (
        '!',
        Node::Chars(&[
            // 0x3F
            (
                '?',
                Node::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[Node::Chars(&[
                        // 0x3C
                        ('<', Node::Id(InstructionId::IfGoTo)),
                    ])],
                )),
            ),
            // 0x7B
            (
                '{',
                Node::Literal(&(
                    LiteralType::StringLiteral,
                    &[Node::Chars(&[
                        // 0x7D
                        ('}', Node::Id(InstructionId::DebugBreak)),
                    ])],
                )),
            ),
        ]),
    ),
    // 0x23
    (
        '#',
        Node::Chars(&[
            // 0x45
            ('E', Node::Id(InstructionId::End)),
            // 0x52
            (
                'R',
                Node::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[Node::Chars(&[
                        // 0x3C
                        ('<', Node::Id(InstructionId::OnResp)),
                    ])],
                )),
            ),
            // 0x53
            ('S', Node::Id(InstructionId::Start)),
        ]),
    ),
    // 0x28
    (
        '(',
        Node::Literal(&(
            LiteralType::VariableIdentifierLiteral,
            &[Node::Chars(&[
                // 0x3C
                (
                    '<',
                    Node::Literal(&(
                        LiteralType::VariableValueLiteral,
                        &[Node::Chars(&[
                            // 0x29
                            (')', Node::Id(InstructionId::VarLess)),
                        ])],
                    )),
                ),
                // 0x3D
                (
                    '=',
                    Node::Literal(&(
                        LiteralType::VariableValueLiteral,
                        &[Node::Chars(&[
                            // 0x29
                            (')', Node::Id(InstructionId::VarEqual)),
                        ])],
                    )),
                ),
                // 0x3E
                (
                    '>',
                    Node::Literal(&(
                        LiteralType::VariableValueLiteral,
                        &[Node::Chars(&[
                            // 0x29
                            (')', Node::Id(InstructionId::VarMore)),
                        ])],
                    )),
                ),
            ])],
        )),
    ),
    // 0x2C
    (',', Node::Id(InstructionId::Back)),
    // 0x2D
    (
        '-',
        Node::Chars(&[
            // 0x3E
            (
                '>',
                Node::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[Node::Chars(&[
                        // 0x3E
                        ('>', Node::Id(InstructionId::GoSub1)),
                    ])],
                )),
            ),
        ]),
    ),
    // 0x3A
    (
        ':',
        Node::Chars(&[
            // 0x3E
            (
                '>',
                Node::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[Node::Chars(&[
                        // 0x3E
                        ('>', Node::Id(InstructionId::GoSub)),
                    ])],
                )),
            ),
        ]),
    ),
    // 0x3C
    (
        '<',
        Node::Chars(&[
            // 0x2D
            (
                '-',
                Node::Chars(&[
                    // 0x7C
                    ('|', Node::Id(InstructionId::Return1)),
                ]),
            ),
            // 0x3D
            (
                '=',
                Node::Chars(&[
                    // 0x7C
                    ('|', Node::Id(InstructionId::ReturnF)),
                ]),
            ),
            // 0x7C
            ('|', Node::Id(InstructionId::Return)),
        ]),
    ),
    // 0x3D
    (
        '=',
        Node::Chars(&[
            // 0x3E
            (
                '>',
                Node::Literal(&(
                    LiteralType::LabelIdentifierLiteral,
                    &[Node::Chars(&[
                        // 0x3E
                        ('>', Node::Id(InstructionId::GoSubF)),
                    ])],
                )),
            ),
            // 0x41
            ('A', Node::Id(InstructionId::CcAcid)),
            // 0x42
            ('B', Node::Id(InstructionId::CccBlackRock)),
            // 0x47
            ('G', Node::Id(InstructionId::CcGun)),
            // 0x4B
            ('K', Node::Id(InstructionId::CccRedRock)),
            // 0x52
            ('R', Node::Id(InstructionId::CccRoad)),
            // 0x61
            ('a', Node::Id(InstructionId::CcAlive)),
            // 0x62
            ('b', Node::Id(InstructionId::CcBolder)),
            // 0x63
            ('c', Node::Id(InstructionId::CcCrystall)),
            // 0x64
            ('d', Node::Id(InstructionId::CcDead)),
            // 0x65
            ('e', Node::Id(InstructionId::CcEmpty)),
            // 0x66
            ('f', Node::Id(InstructionId::CcGravity)),
            // 0x67
            ('g', Node::Id(InstructionId::CccGreenBlock)),
            // 0x68
            (
                'h',
                Node::Chars(&[
                    // 0x70
                    (
                        'p',
                        Node::Chars(&[
                            // 0x2D
                            ('-', Node::Id(InstructionId::CbHp)),
                            // 0x35
                            (
                                '5',
                                Node::Chars(&[
                                    // 0x30
                                    ('0', Node::Id(InstructionId::CbHp50)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x6B
            ('k', Node::Id(InstructionId::CcRock)),
            // 0x6E
            ('n', Node::Id(InstructionId::CcNotEmpty)),
            // 0x6F
            ('o', Node::Id(InstructionId::CccOpor)),
            // 0x71
            ('q', Node::Id(InstructionId::CccQuadro)),
            // 0x72
            ('r', Node::Id(InstructionId::CccRedBlock)),
            // 0x73
            ('s', Node::Id(InstructionId::CcSand)),
            // 0x78
            ('x', Node::Id(InstructionId::CccBox)),
            // 0x79
            ('y', Node::Id(InstructionId::CccYellowBlock)),
        ]),
    ),
    // 0x3E
    (
        '>',
        Node::Literal(&(
            LiteralType::LabelIdentifierLiteral,
            &[Node::Chars(&[
                // 0x7C
                ('|', Node::Id(InstructionId::GoTo)),
            ])],
        )),
    ),
    // 0x3F
    (
        '?',
        Node::Literal(&(
            LiteralType::LabelIdentifierLiteral,
            &[Node::Chars(&[
                // 0x3C
                ('<', Node::Id(InstructionId::IfNotGoTo)),
            ])],
        )),
    ),
    // 0x41
    (
        'A',
        Node::Chars(&[
            // 0x47
            (
                'G',
                Node::Chars(&[
                    // 0x52
                    (
                        'R',
                        Node::Chars(&[
                            // 0x2B
                            ('+', Node::Id(InstructionId::ModeAgrOn)),
                            // 0x2D
                            ('-', Node::Id(InstructionId::ModeAgrOff)),
                        ]),
                    ),
                ]),
            ),
            // 0x4E
            (
                'N',
                Node::Chars(&[
                    // 0x44
                    ('D', Node::Id(InstructionId::BoolModeAnd)),
                ]),
            ),
            // 0x55
            (
                'U',
                Node::Chars(&[
                    // 0x54
                    (
                        'T',
                        Node::Chars(&[
                            // 0x2B
                            ('+', Node::Id(InstructionId::ModeAutodiggOn)),
                            // 0x2D
                            ('-', Node::Id(InstructionId::ModeAutodiggOff)),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x42
    (
        'B',
        Node::Chars(&[
            // 0x31
            (
                '1',
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::ActionB1)),
                ]),
            ),
            // 0x32
            (
                '2',
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::ActionB3)),
                ]),
            ),
            // 0x33
            (
                '3',
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::ActionB2)),
                ]),
            ),
            // 0x45
            (
                'E',
                Node::Chars(&[
                    // 0x45
                    (
                        'E',
                        Node::Chars(&[
                            // 0x50
                            (
                                'P',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::ActionBibika)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x55
            (
                'U',
                Node::Chars(&[
                    // 0x49
                    (
                        'I',
                        Node::Chars(&[
                            // 0x4C
                            (
                                'L',
                                Node::Chars(&[
                                    // 0x44
                                    (
                                        'D',
                                        Node::Chars(&[
                                            // 0x3B
                                            (';', Node::Id(InstructionId::StdBuild)),
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
        Node::Chars(&[
            // 0x31
            (
                '1',
                Node::Chars(&[
                    // 0x39
                    (
                        '9',
                        Node::Chars(&[
                            // 0x30
                            (
                                '0',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::ActionC190)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x43
            (
                'C',
                Node::Chars(&[
                    // 0x57
                    (
                        'W',
                        Node::Chars(&[
                            // 0x3B
                            (';', Node::Id(InstructionId::RotateCcw)),
                        ]),
                    ),
                ]),
            ),
            // 0x52
            (
                'R',
                Node::Chars(&[
                    // 0x41
                    (
                        'A',
                        Node::Chars(&[
                            // 0x46
                            (
                                'F',
                                Node::Chars(&[
                                    // 0x54
                                    (
                                        'T',
                                        Node::Chars(&[
                                            // 0x3B
                                            (';', Node::Id(InstructionId::ActionCraft)),
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
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::RotateCw)),
                ]),
            ),
        ]),
    ),
    // 0x44
    (
        'D',
        Node::Chars(&[
            // 0x49
            (
                'I',
                Node::Chars(&[
                    // 0x47
                    (
                        'G',
                        Node::Chars(&[
                            // 0x47
                            (
                                'G',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::StdDigg)),
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
        Node::Chars(&[
            // 0x49
            (
                'I',
                Node::Chars(&[
                    // 0x4C
                    (
                        'L',
                        Node::Chars(&[
                            // 0x4C
                            (
                                'L',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::FillGun)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x4C
            (
                'L',
                Node::Chars(&[
                    // 0x49
                    (
                        'I',
                        Node::Chars(&[
                            // 0x50
                            (
                                'P',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::ProgFlip)),
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
        Node::Chars(&[
            // 0x45
            (
                'E',
                Node::Chars(&[
                    // 0x4F
                    (
                        'O',
                        Node::Chars(&[
                            // 0x3B
                            (';', Node::Id(InstructionId::ActionGeopack)),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x48
    (
        'H',
        Node::Chars(&[
            // 0x45
            (
                'E',
                Node::Chars(&[
                    // 0x41
                    (
                        'A',
                        Node::Chars(&[
                            // 0x4C
                            (
                                'L',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::StdHeal)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x61
            (
                'a',
                Node::Chars(&[
                    // 0x6E
                    (
                        'n',
                        Node::Chars(&[
                            // 0x64
                            (
                                'd',
                                Node::Chars(&[
                                    // 0x2B
                                    ('+', Node::Id(InstructionId::HandModeOn)),
                                    // 0x2D
                                    ('-', Node::Id(InstructionId::HandModeOff)),
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
        Node::Chars(&[
            // 0x49
            (
                'I',
                Node::Chars(&[
                    // 0x4E
                    (
                        'N',
                        Node::Chars(&[
                            // 0x45
                            (
                                'E',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::StdMine)),
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
        Node::Chars(&[
            // 0x41
            (
                'A',
                Node::Chars(&[
                    // 0x4E
                    (
                        'N',
                        Node::Chars(&[
                            // 0x4F
                            (
                                'O',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::ActionNano)),
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
        Node::Chars(&[
            // 0x52
            ('R', Node::Id(InstructionId::BoolModeOr)),
        ]),
    ),
    // 0x50
    (
        'P',
        Node::Chars(&[
            // 0x4F
            (
                'O',
                Node::Chars(&[
                    // 0x4C
                    (
                        'L',
                        Node::Chars(&[
                            // 0x59
                            (
                                'Y',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::ActionPoly)),
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
        Node::Chars(&[
            // 0x41
            (
                'A',
                Node::Chars(&[
                    // 0x4E
                    (
                        'N',
                        Node::Chars(&[
                            // 0x44
                            (
                                'D',
                                Node::Chars(&[
                                    // 0x3B
                                    (';', Node::Id(InstructionId::ActionRandom)),
                                ]),
                            ),
                        ]),
                    ),
                ]),
            ),
            // 0x45
            (
                'E',
                Node::Chars(&[
                    // 0x4D
                    (
                        'M',
                        Node::Chars(&[
                            // 0x3B
                            (';', Node::Id(InstructionId::ActionRembot)),
                        ]),
                    ),
                ]),
            ),
        ]),
    ),
    // 0x55
    (
        'U',
        Node::Chars(&[
            // 0x50
            (
                'P',
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::ActionUp)),
                ]),
            ),
        ]),
    ),
    // 0x56
    (
        'V',
        Node::Chars(&[
            // 0x42
            (
                'B',
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::ActionWb)),
                ]),
            ),
        ]),
    ),
    // 0x5A
    (
        'Z',
        Node::Chars(&[
            // 0x5A
            (
                'Z',
                Node::Chars(&[
                    // 0x3B
                    (';', Node::Id(InstructionId::ActionZm)),
                ]),
            ),
        ]),
    ),
    // 0x5B
    (
        '[',
        Node::Chars(&[
            // 0x41
            (
                'A',
                Node::Chars(&[
                    // 0x53
                    (
                        'S',
                        Node::Chars(&[
                            // 0x5D
                            (']', Node::Id(InstructionId::CellAs)),
                        ]),
                    ),
                    // 0x5D
                    (']', Node::Id(InstructionId::CellA)),
                ]),
            ),
            // 0x44
            (
                'D',
                Node::Chars(&[
                    // 0x57
                    (
                        'W',
                        Node::Chars(&[
                            // 0x5D
                            (']', Node::Id(InstructionId::CellDw)),
                        ]),
                    ),
                    // 0x5D
                    (']', Node::Id(InstructionId::CellD)),
                ]),
            ),
            // 0x46
            (
                'F',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellF)),
                ]),
            ),
            // 0x53
            (
                'S',
                Node::Chars(&[
                    // 0x44
                    (
                        'D',
                        Node::Chars(&[
                            // 0x5D
                            (']', Node::Id(InstructionId::CellSd)),
                        ]),
                    ),
                    // 0x5D
                    (']', Node::Id(InstructionId::CellS)),
                ]),
            ),
            // 0x57
            (
                'W',
                Node::Chars(&[
                    // 0x41
                    (
                        'A',
                        Node::Chars(&[
                            // 0x5D
                            (']', Node::Id(InstructionId::CellWa)),
                        ]),
                    ),
                    // 0x5D
                    (']', Node::Id(InstructionId::CellW)),
                ]),
            ),
            // 0x61
            (
                'a',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellAa)),
                ]),
            ),
            // 0x64
            (
                'd',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellDd)),
                ]),
            ),
            // 0x66
            (
                'f',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellFf)),
                ]),
            ),
            // 0x6C
            (
                'l',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellLeftHand)),
                ]),
            ),
            // 0x72
            (
                'r',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellRightHand)),
                ]),
            ),
            // 0x73
            (
                's',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellSs)),
                ]),
            ),
            // 0x77
            (
                'w',
                Node::Chars(&[
                    // 0x5D
                    (']', Node::Id(InstructionId::CellWw)),
                ]),
            ),
        ]),
    ),
    // 0x5E
    (
        '^',
        Node::Chars(&[
            // 0x41
            ('A', Node::Id(InstructionId::MoveA)),
            // 0x44
            ('D', Node::Id(InstructionId::MoveD)),
            // 0x46
            ('F', Node::Id(InstructionId::MoveF)),
            // 0x53
            ('S', Node::Id(InstructionId::MoveS)),
            // 0x57
            ('W', Node::Id(InstructionId::MoveW)),
        ]),
    ),
    // 0x5F
    ('_', Node::Command(Command::ThreeStepsForward)),
    // 0x61
    ('a', Node::Id(InstructionId::LookA)),
    // 0x62
    ('b', Node::Id(InstructionId::ActionBuild)),
    // 0x64
    ('d', Node::Id(InstructionId::LookD)),
    // 0x67
    ('g', Node::Id(InstructionId::ActionGeo)),
    // 0x68
    ('h', Node::Id(InstructionId::ActionHeal)),
    // 0x69
    (
        'i',
        Node::Chars(&[
            // 0x61
            ('a', Node::Id(InstructionId::InvDirA)),
            // 0x64
            ('d', Node::Id(InstructionId::InvDirD)),
            // 0x73
            ('s', Node::Id(InstructionId::InvDirS)),
            // 0x77
            ('w', Node::Id(InstructionId::InvDirW)),
        ]),
    ),
    // 0x71
    ('q', Node::Id(InstructionId::ActionQuadro)),
    // 0x72
    ('r', Node::Id(InstructionId::ActionRoad)),
    // 0x73
    ('s', Node::Id(InstructionId::LookS)),
    // 0x77
    ('w', Node::Id(InstructionId::LookW)),
    // 0x7A
    ('z', Node::Id(InstructionId::Digg)),
    // 0x7B
    (
        '{',
        Node::Literal(&(
            LiteralType::StringLiteral,
            &[Node::Chars(&[
                // 0x7D
                ('}', Node::Id(InstructionId::DebugSet)),
            ])],
        )),
    ),
    // 0x7C
    (
        '|',
        Node::Literal(&(
            LiteralType::LabelIdentifierLiteral,
            &[Node::Chars(&[
                // 0x3A
                (':', Node::Id(InstructionId::Label)),
            ])],
        )),
    ),
    // 0x7E
    ('~', Node::Command(Command::GoToNextPage)),
];

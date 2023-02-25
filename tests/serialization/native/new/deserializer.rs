use m3c::formats::native::new::diagnostics::{Diagnostics, NoMagicFound, UnknownToken};
use m3c::serialization::native::new::{TextFormatDeserializer, TextFormatDeserializerV2};
use m3c::utils::CharPosition;

use crate::common;

/// Tests all `Simple` kind instuctions.
#[test]
fn deserialize_simple_instructions() {
    let given_string = common::native::new::ALL_SIMPLE;
    let expected_program = common::internal::all_simple();

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, given_string);
    de.deserialize().unwrap();

    assert_eq!(expected_program, actual_program);
}

#[test]
fn deserialize_not_all_commands() {
    let given_string = common::native::new::NOT_ALL_COMMANDS;
    let expected_program = common::internal::not_all_commands();

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, given_string);
    de.deserialize().unwrap();

    assert_eq!(expected_program, actual_program);
}

#[test]
fn deserialize_literals() {
    let given_string = common::native::new::LITERALS;
    let expected_program = common::internal::literals();

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, given_string);
    de.deserialize().unwrap();

    assert_eq!(expected_program, actual_program);
}

/// Totally empty string without magic.
#[test]
fn deserialize_v2_empty_string() {
    let given_string = common::native::new::EMPTY;

    let expected_program = common::internal::empty();
    let expected_diagnostics: Vec<Diagnostics> = vec![NoMagicFound::new().into()];

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializerV2::new(given_string);
    let actual_diagnostics = de.deserialize(&mut actual_program);

    assert_eq!(expected_program, actual_program);
    assert_eq!(expected_diagnostics, actual_diagnostics);
}

/// String containing only magic.
#[test]
fn deserialize_v2_string_with_only_magic() {
    let given_string = common::native::new::ONLY_MAGIC;

    let expected_program = common::internal::empty();
    let expected_diagnostics: Vec<Diagnostics> = vec![];

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializerV2::new(given_string);
    let actual_diagnostics = de.deserialize(&mut actual_program);

    assert_eq!(expected_program, actual_program);
    assert_eq!(expected_diagnostics, actual_diagnostics);
}

/// Tests for no magic.
#[test]
fn deserialize_v2_no_magic_but_move_w() {
    let given_string = common::native::new::NO_MAGIC_BUT_MOVE_W;

    let expected_program = common::internal::only_move_w();
    let expected_diagnostics: Vec<Diagnostics> = vec![NoMagicFound::new().into()];

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializerV2::new(given_string);
    let actual_diagnostics = de.deserialize(&mut actual_program);

    assert_eq!(expected_program, actual_program);
    assert_eq!(expected_diagnostics, actual_diagnostics);
}

/// Tests all `Simple` kind instuctions.
#[test]
fn deserialize_v2_simple_instructions() {
    let given_string = common::native::new::ALL_SIMPLE;

    let expected_program = common::internal::all_simple();
    let expected_diagnostics: Vec<Diagnostics> = vec![];

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializerV2::new(given_string);
    let actual_diagnostics = de.deserialize(&mut actual_program);

    assert_eq!(expected_program, actual_program);
    assert_eq!(expected_diagnostics, actual_diagnostics);
}

/// Tests that Deserializer can go the next char if this char is illegal to start the token with.
///
/// ```text
/// 012345678901234567
/// $^W]]]]^Sфівіаві^F
///    ^^^^  ^^^^^^^
/// ```
#[test]
fn deserialize_v2_simple_instructions_with_illegal_start_chars() {
    let given_string = common::native::new::WITH_ILLEGAL_START_CHARS;

    let expected_program = common::internal::moves_wsf();
    let expected_diagnostics: Vec<Diagnostics> = vec![
        UnknownToken::new(
            CharPosition {
                index: 3,
                line: 0,
                column: 3,
            },
            CharPosition {
                index: 6,
                line: 0,
                column: 6,
            },
        )
        .into(),
        UnknownToken::new(
            CharPosition {
                index: 9,
                line: 0,
                column: 9,
            },
            CharPosition {
                index: 15,
                line: 0,
                column: 15,
            },
        )
        .into(),
    ];

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializerV2::new(given_string);
    let actual_diagnostics = de.deserialize(&mut actual_program);

    assert_eq!(expected_program, actual_program);
    assert_eq!(expected_diagnostics, actual_diagnostics);
}

/// A test for illegal token continuation ([`UnknownToken`]).
///
/// ```text
/// 01234567890123
/// $^W^a^SGEa^FGE
///    ^   ^^   ^^
/// ```
#[test]
fn deserialize_v2_unknown_token_continuation() {
    let given_string = common::native::new::WITH_UNKNOWN_CONTINUATION_CHARS;

    let expected_program = common::internal::moves_and_looks();
    let expected_diagnostics: Vec<Diagnostics> = vec![
        UnknownToken::new(
            CharPosition {
                index: 3,
                line: 0,
                column: 3,
            },
            CharPosition {
                index: 3,
                line: 0,
                column: 3,
            },
        )
        .into(),
        UnknownToken::new(
            CharPosition {
                index: 7,
                line: 0,
                column: 7,
            },
            CharPosition {
                index: 8,
                line: 0,
                column: 8,
            },
        )
        .into(),
        UnknownToken::new(
            CharPosition {
                index: 12,
                line: 0,
                column: 12,
            },
            CharPosition {
                index: 13,
                line: 0,
                column: 13,
            },
        )
        .into(),
    ];

    let mut actual_program = common::internal::empty();

    let mut de = TextFormatDeserializerV2::new(given_string);
    let actual_diagnostics = de.deserialize(&mut actual_program);

    assert_eq!(expected_program, actual_program);
    assert_eq!(expected_diagnostics, actual_diagnostics);
}

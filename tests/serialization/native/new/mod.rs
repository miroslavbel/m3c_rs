use m3c::formats::native::new::diagnostics::{Diagnostics, NoMagicFound};
use m3c::serialization::native::new::{TextFormatDeserializer, TextFormatDeserializerV2};

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
fn deserialize_commands() {
    let given_string = common::native::new::COMMANDS;
    let expected_program = common::internal::commands();

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
    let expected_diagnostics: Vec<Diagnostics> =
        vec![Diagnostics::NoMagicFound(NoMagicFound::new())];

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

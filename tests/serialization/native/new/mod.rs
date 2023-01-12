use m3c::formats::internal::Program;
use m3c::serialization::native::new::TextFormatDeserializer;

use crate::common;

/// Tests all `Simple` kind instuctions.
#[test]
fn deserialize_v2_simple_instructions() {
    let given_string = common::native::new::ALL_SIMPLE;
    let expected_program = common::internal::all_simple();

    let mut actual_program = Program::default();

    let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, given_string);
    de.deserialize_v2().unwrap();

    assert_eq!(expected_program, actual_program);
}

#[test]
fn deserialize_v2_commands() {
    let given_string = common::native::new::COMMANDS;
    let expected_program = common::internal::commands();

    let mut actual_program = Program::default();

    let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, given_string);
    de.deserialize_v2().unwrap();

    assert_eq!(expected_program, actual_program);
}

#[test]
fn deserialize_v2_literals() {
    let given_string = common::native::new::LITERALS;
    let expected_program = common::internal::literals();

    let mut actual_program = Program::default();

    let mut de = TextFormatDeserializer::new_from_str(&mut actual_program, given_string);
    de.deserialize_v2().unwrap();

    assert_eq!(expected_program, actual_program);
}

use m3c::serialization::native::new::TextFormatSerializer;

use crate::common;

/// Tests all `Simple` kind instuctions.
#[test]
fn serialize_simple_instructions() {
    let given_program = common::internal::all_simple();

    let expected_string = common::native::new::ALL_SIMPLE;

    let mut buf = vec![];
    let mut se = TextFormatSerializer::new(&given_program);
    se.serialize(&mut buf).unwrap();

    assert_eq!(expected_string, String::from_utf8(buf).unwrap());
}

/// Tests all commands.
#[test]
fn serialize_commands() {
    let given_program = common::internal::commands();

    let expected_string = common::native::new::COMMANDS;

    let mut buf = vec![];
    let mut se = TextFormatSerializer::new(&given_program);
    se.serialize(&mut buf).unwrap();

    assert_eq!(expected_string, String::from_utf8(buf).unwrap());
}

/// Tests all not `Simple` kind instuctions.
#[test]
fn serialize_literals() {
    let given_program = common::internal::literals();

    let expected_string = common::native::new::LITERALS;

    let mut buf = vec![];
    let mut se = TextFormatSerializer::new(&given_program);
    se.serialize(&mut buf).unwrap();

    assert_eq!(expected_string, String::from_utf8(buf).unwrap());
}

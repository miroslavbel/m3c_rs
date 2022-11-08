//! Serializer and deserializer for New Text format.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).

use std::{iter::Enumerate, str::Chars};

use crate::formats::internal::{InstructionPosition, Program};

/// A structure that deserializes New Text format into [Internal format](crate::formats::internal).
#[derive(Debug)]
pub struct NewTextFormatDeserializer<'e> {
    enumeration: Enumerate<Chars<'e>>,
    position: InstructionPosition,
    program: Program,
}

impl<'e> NewTextFormatDeserializer<'e> {
    /// Creates a new [`NewTextFormatDeserializer`] from a `&str`.
    pub fn new_from_str(s: &'e str) -> Self {
        Self {
            enumeration: s.chars().enumerate(),
            position: InstructionPosition::default(),
            program: Program::default(),
        }
    }
}

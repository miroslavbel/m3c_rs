//! Module for New Text format's diagnostics.

use crate::utils::CharPosition;

// region: general

/// A trait for diagnostic objects.
pub trait Diagnostic {
    /// Returns this diagnostic's [id](DiagnosticId).
    fn id(&self) -> DiagnosticId;
    /// Returns this diagnostic's [id](DiagnosticId) prefixed with letter `N` (stands for New Text Format).
    fn prefixed_id(&self) -> String {
        let id: u8 = self.id().into();
        format_args!("N{:0>2}", id).to_string()
    }
    /// Returns this diagnostic's message.
    fn what(&self) -> String;
    /// Returns this diagnostic's position of source code.
    fn position(&self) -> CharPosition;
}

/// Represents all id of [Diagnostic].
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DiagnosticId {
    NoMagicFound = 1,
}

impl From<DiagnosticId> for u8 {
    fn from(id: DiagnosticId) -> Self {
        id as u8
    }
}

/// An enumeration of all [Diagnostic]s.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Diagnostics {
    NoMagicFound(NoMagicFound),
}

macro_rules! impl_trait_for_diagnostics {
    ($method:ident, $rtype:ident) => {
        fn $method(&self) -> $rtype {
            match self {
                Self::NoMagicFound(x) => x.$method(),
            }
        }
    };
}

impl Diagnostic for Diagnostics {
    impl_trait_for_diagnostics!(id, DiagnosticId);
    impl_trait_for_diagnostics!(prefixed_id, String);
    impl_trait_for_diagnostics!(what, String);
    impl_trait_for_diagnostics!(position, CharPosition);
}

// endregion: general

/// No magic (`$`) found at the start of the line.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoMagicFound;

impl NoMagicFound {
    pub const ID: DiagnosticId = DiagnosticId::NoMagicFound;
    pub fn new() -> Self {
        Self {}
    }
}

impl Diagnostic for NoMagicFound {
    fn id(&self) -> DiagnosticId {
        Self::ID
    }
    fn what(&self) -> String {
        format_args!(
            "{}: [{}] no magic ('$') found at the start of the line\n",
            self.position().custom_format(),
            self.prefixed_id()
        )
        .to_string()
    }
    fn position(&self) -> CharPosition {
        // NOTE: this diagnostic is always at the beginning of string
        CharPosition::default()
    }
}

impl From<NoMagicFound> for Diagnostics {
    fn from(x: NoMagicFound) -> Self {
        Diagnostics::NoMagicFound(x)
    }
}

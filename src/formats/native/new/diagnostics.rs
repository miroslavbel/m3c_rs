//! Module for New Text format's diagnostics.

use crate::utils::CharPosition;

// region: general

/// A trait for diagnostic objects.
pub trait Diagnostic {
    /// Returns this diagnostic's [id](DiagnosticId).
    fn id(&self) -> DiagnosticId;
    /// Returns this diagnostic's [id](DiagnosticId) prefixed with letter `N` (stands for New Text
    /// Format).
    fn prefixed_id(&self) -> String {
        let id: u8 = self.id().into();
        format_args!("N{:0>2}", id).to_string()
    }
    /// Returns this diagnostic's message.
    fn what(&self) -> String;
    /// Returns this diagnostic's position in source code.
    ///
    /// Returns the start position for [`UnknownToken`].
    fn position(&self) -> CharPosition;
}

/// Represents all id of [Diagnostic].
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum DiagnosticId {
    NoMagicFound = 1,
    UnknownToken = 2,
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
    UnknownToken(UnknownToken),
}

macro_rules! impl_trait_for_diagnostics {
    ($method:ident, $rtype:ident) => {
        fn $method(&self) -> $rtype {
            match self {
                Self::NoMagicFound(x) => x.$method(),
                Self::UnknownToken(x) => x.$method(),
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

/// Illegal char(s) at the beginning of the token.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UnknownToken {
    start: CharPosition,
    end: CharPosition,
}

impl UnknownToken {
    pub const ID: DiagnosticId = DiagnosticId::UnknownToken;
    pub fn new(start: CharPosition, end: CharPosition) -> Self {
        Self { start, end }
    }
    /// Returns position of the first illegal char.
    pub fn start(&self) -> CharPosition {
        self.start
    }
    /// Returns position of the last illegal char.
    pub fn end(&self) -> CharPosition {
        self.end
    }
}

impl Diagnostic for UnknownToken {
    fn id(&self) -> DiagnosticId {
        Self::ID
    }
    fn what(&self) -> String {
        let pos_str = if self.start == self.end {
            self.position().custom_format()
        } else {
            format_args!(
                "{}-{}",
                self.start.custom_format(),
                self.end.custom_format()
            )
            .to_string()
        };
        format_args!(
            "{}: [{}] unknown token found\n",
            pos_str,
            self.prefixed_id()
        )
        .to_string()
    }
    /// Returns position of the first illegal char.
    fn position(&self) -> CharPosition {
        self.start()
    }
}

impl From<UnknownToken> for Diagnostics {
    fn from(x: UnknownToken) -> Self {
        Diagnostics::UnknownToken(x)
    }
}

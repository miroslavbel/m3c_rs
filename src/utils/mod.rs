//! A module for various unsorted staff.

use std::str::Chars;

/// A simple struct to handle char's `index`, `line`, `column`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CharPosition {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

/// A wrapper to track char position (in `line:column` way) around the [`str::chars()`].
///
/// To return only char index without line and column see [`Chars::enumerate()`].
///
/// # Examples
///
/// ```
/// use m3c::utils::{CharPosition, EnumerateWithPosition};
///
/// let s = "01\nA\r\n+";
/// let mut ewp = EnumerateWithPosition::new(s);
///
/// assert_eq!(ewp.next(), Some((CharPosition{index: 0, line: 0, column: 0}, '0')));
/// assert_eq!(ewp.next(), Some((CharPosition{index: 1, line: 0, column: 1}, '1')));
/// assert_eq!(ewp.next(), Some((CharPosition{index: 2, line: 0, column: 2}, '\n')));
/// assert_eq!(ewp.next(), Some((CharPosition{index: 3, line: 1, column: 0}, 'A')));
/// assert_eq!(ewp.next(), Some((CharPosition{index: 4, line: 1, column: 1}, '\r')));
/// assert_eq!(ewp.next(), Some((CharPosition{index: 5, line: 1, column: 2}, '\n')));
/// assert_eq!(ewp.next(), Some((CharPosition{index: 6, line: 2, column: 0}, '+')));
/// assert_eq!(ewp.next(), None);
/// ```
///
/// [`std::str::Chars::enumerate()`]: [`Chars::enumerate()`]
#[derive(Debug)]
pub struct EnumerateWithPosition<'s> {
    iter: Chars<'s>,
    pos: CharPosition,
}

impl<'s> EnumerateWithPosition<'s> {
    /// Creates a new [`EnumerateWithPosition`].
    pub fn new(s: &'s str) -> Self {
        EnumerateWithPosition {
            iter: s.chars(),
            pos: CharPosition::default(),
        }
    }
}

impl<'s> Iterator for EnumerateWithPosition<'s> {
    type Item = (CharPosition, char);
    /// Advances the iterator and returns the next value.
    ///
    /// Returns a pair `(pos, ch)`, where `pos` is the [`CharPosition`] of `ch` and `ch` is the char
    /// returned by the [`str::chars()`].
    ///
    /// For more information see [`EnumerateWithPosition`] examples sections.
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.iter.next()?;
        let old_position = self.pos;
        self.pos.index += 1;
        match ch {
            '\n' => {
                self.pos.column = 0;
                self.pos.line += 1;
                Some((old_position, ch))
            }
            _ => {
                self.pos.column += 1;
                Some((old_position, ch))
            }
        }
    }
}

impl<'s> From<&'s str> for EnumerateWithPosition<'s> {
    fn from(s: &'s str) -> Self {
        EnumerateWithPosition::new(s)
    }
}

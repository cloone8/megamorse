use crate::{MorseCode, MorseSequence};

/// A struct representing a single morse code sequence that maps
/// to a single character. For example, 'a', '0' or 'G' all map
/// to a single [MorseWord].
///
/// The [MorseWord] can contain up to 5 [MorseCode] values, and can
/// thus represent any character in the Morse code alphabet (and no more)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseWord {
    code: u8,
}

macro_rules! from_word {
    ($num:literal) => {
        impl From<[MorseCode; $num]> for MorseWord {
            /// Creates a [MorseWord] from an array of [MorseCode] values.
            fn from(codes: [MorseCode; $num]) -> Self {
                MorseWord::new(codes)
            }
        }
    };
}

from_word!(1);
from_word!(2);
from_word!(3);
from_word!(4);
from_word!(5);

impl MorseWord {
    const fn new<const N: usize>(codes: [MorseCode; N]) -> Self {
        debug_assert!(N <= 5);

        // The first three bits are used to store the word length
        let mut code: u8 = (N as u8) & 0b0000_0111;

        if N >= 1 {
            code |= codes[0].to_bit() << 3;
        }

        if N >= 2 {
            code |= codes[1].to_bit() << 4;
        }

        if N >= 3 {
            code |= codes[2].to_bit() << 5;
        }

        if N >= 4 {
            code |= codes[3].to_bit() << 6;
        }

        if N == 5 {
            code |= codes[4].to_bit() << 7;
        }

        MorseWord { code }
    }

    /// Returns the [MorseWord] as an array of [MorseCode] values.
    ///
    /// The array will contain up to 5 [MorseCode] values, depending
    /// on the length of the [MorseWord].
    ///
    /// # Returns
    ///
    /// A tuple containing the length of the MorseWord and an array
    /// of [MorseCode] values.
    ///
    /// The returned array will be padded with [MorseCode::Dot] values if the
    /// [MorseWord] is shorter than 5 characters.
    ///
    /// # Examples
    ///
    /// ```
    /// let word = morse!(..-);
    ///
    /// let (len, codes) = word.to_array();
    ///
    /// assert_eq!(len, 3);
    ///
    /// assert_eq!(codes[0], MorseCode::Dot);
    /// assert_eq!(codes[1], MorseCode::Dot);
    /// assert_eq!(codes[2], MorseCode::Dash);
    /// ````
    pub const fn to_array(self) -> (usize, [MorseCode; 5]) {
        let mut codes = [MorseCode::Dot; 5];
        let n = self.len();

        if n >= 1 {
            codes[0] = if self.code & 0b0000_0001 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if n >= 2 {
            codes[1] = if self.code & 0b0000_0010 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if n >= 3 {
            codes[2] = if self.code & 0b0000_0100 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if n >= 4 {
            codes[3] = if self.code & 0b0000_1000 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if n == 5 {
            codes[4] = if self.code & 0b0001_0000 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        (n, codes)
    }

    /// Returns the amount of [MorseCode] values in the [MorseWord].
    pub const fn len(&self) -> usize {
        debug_assert!(self.code & 0b0000_0111 <= 5);
        (self.code & 0b0000_0111) as usize
    }

    /// Returns true if the [MorseWord] is empty, i.e. contains no [MorseCode] values.
    /// Should basically always return false, as a [MorseWord] with no [MorseCode] values
    /// cannot be constructed without shenanigans.
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the [MorseWord] as a [MorseSequence] array.
    /// Used internally by the megamorse library to convert a [MorseWord] to a
    /// sequence of playable [MorseSequence] values, with each
    /// [MorseSequence] representing a single Morse code time unit.
    ///
    /// # Returns
    ///
    /// A tuple containing the length of the [MorseSequence] array and the
    /// array itself, padded with [MorseSequence::Pause] values if the
    /// [MorseWord] is shorter than 5 characters.
    ///
    /// Reading beyond the length given by the first element of the tuple is
    /// not useful, as the array will be padded with [MorseSequence::Pause] values.
    pub const fn to_sequence(self) -> (usize, [MorseSequence; 9]) {
        let mut sequence = [MorseSequence::Pause; 9];

        let (n, codes) = self.to_array();

        debug_assert!(n <= 5);

        if n >= 1 {
            sequence[0] = MorseSequence::Code(codes[0]);
        }

        if n >= 2 {
            sequence[2] = MorseSequence::Code(codes[1]);
        }

        if n >= 3 {
            sequence[4] = MorseSequence::Code(codes[2]);
        }

        if n >= 4 {
            sequence[6] = MorseSequence::Code(codes[3]);
        }

        if n == 5 {
            sequence[8] = MorseSequence::Code(codes[4]);
        }

        let seq_len = 2 * n - 1;

        debug_assert!(seq_len <= 9);

        (seq_len, sequence)
    }
}

/// Converts a single [char] to a [MorseWord].
/// Is used both internally by the megamorse library and can be used
/// by the user to convert a single character to a [MorseWord], which
/// can then be played by a player.
///
/// Will return an error if the character has no Morse code representation.
impl TryFrom<char> for MorseWord {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let as_lower = value.to_ascii_lowercase();

        let word = match as_lower {
            'a' => MorseWord::from([MorseCode::Dot, MorseCode::Dash]),
            'b' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            'c' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dot,
            ]),
            'd' => MorseWord::from([MorseCode::Dash, MorseCode::Dot, MorseCode::Dot]),
            'e' => MorseWord::from([MorseCode::Dot]),
            'f' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dot,
            ]),
            'g' => MorseWord::from([MorseCode::Dash, MorseCode::Dash, MorseCode::Dot]),
            'h' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            'i' => MorseWord::from([MorseCode::Dot, MorseCode::Dot]),
            'j' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ]),
            'k' => MorseWord::from([MorseCode::Dash, MorseCode::Dot, MorseCode::Dash]),
            'l' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            'm' => MorseWord::from([MorseCode::Dash, MorseCode::Dash]),
            'n' => MorseWord::from([MorseCode::Dash, MorseCode::Dot]),
            'o' => MorseWord::from([MorseCode::Dash, MorseCode::Dash, MorseCode::Dash]),
            'p' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
            ]),
            'q' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dash,
            ]),
            'r' => MorseWord::from([MorseCode::Dot, MorseCode::Dash, MorseCode::Dot]),
            's' => MorseWord::from([MorseCode::Dot, MorseCode::Dot, MorseCode::Dot]),
            't' => MorseWord::from([MorseCode::Dash]),
            'u' => MorseWord::from([MorseCode::Dot, MorseCode::Dot, MorseCode::Dash]),
            'v' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
            ]),
            'w' => MorseWord::from([MorseCode::Dot, MorseCode::Dash, MorseCode::Dash]),
            'x' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
            ]),
            'y' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
            ]),
            'z' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            '0' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ]),
            '1' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ]),
            '2' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ]),
            '3' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
            ]),
            '4' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
            ]),
            '5' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            '6' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            '7' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            '8' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
            ]),
            '9' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
            ]),
            _ => return Err(()),
        };

        Ok(word)
    }
}

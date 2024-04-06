use crate::{MorseCode, MorseSequence};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseWord {
    code: u8,
}

macro_rules! from_word {
    ($num:literal) => {
        impl From<[MorseCode; $num]> for MorseWord {
            /// Create a MorseWord from an array of MorseCode values.
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

    pub const fn len(&self) -> usize {
        debug_assert!(self.code & 0b0000_0111 <= 5);
        (self.code & 0b0000_0111) as usize
    }

    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

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

impl TryFrom<char> for MorseWord {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let as_lower = value.to_ascii_lowercase();

        let word = match as_lower {
            'a' => MorseWord::from([MorseCode::Dot, MorseCode::Dash]).into(),
            'b' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            'c' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dot,
            ])
            .into(),
            'd' => MorseWord::from([MorseCode::Dash, MorseCode::Dot, MorseCode::Dot]).into(),
            'e' => MorseWord::from([MorseCode::Dot]).into(),
            'f' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dot,
            ])
            .into(),
            'g' => MorseWord::from([MorseCode::Dash, MorseCode::Dash, MorseCode::Dot]).into(),
            'h' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            'i' => MorseWord::from([MorseCode::Dot, MorseCode::Dot]).into(),
            'j' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ])
            .into(),
            'k' => MorseWord::from([MorseCode::Dash, MorseCode::Dot, MorseCode::Dash]),
            'l' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            'm' => MorseWord::from([MorseCode::Dash, MorseCode::Dash]).into(),
            'n' => MorseWord::from([MorseCode::Dash, MorseCode::Dot]).into(),
            'o' => MorseWord::from([MorseCode::Dash, MorseCode::Dash, MorseCode::Dash]).into(),
            'p' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
            ])
            .into(),
            'q' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dash,
            ])
            .into(),
            'r' => MorseWord::from([MorseCode::Dot, MorseCode::Dash, MorseCode::Dot]).into(),
            's' => MorseWord::from([MorseCode::Dot, MorseCode::Dot, MorseCode::Dot]).into(),
            't' => MorseWord::from([MorseCode::Dash]).into(),
            'u' => MorseWord::from([MorseCode::Dot, MorseCode::Dot, MorseCode::Dash]).into(),
            'v' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
            ])
            .into(),
            'w' => MorseWord::from([MorseCode::Dot, MorseCode::Dash, MorseCode::Dash]).into(),
            'x' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
            ])
            .into(),
            'y' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
            ])
            .into(),
            'z' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            '0' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ])
            .into(),
            '1' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ])
            .into(),
            '2' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
            ])
            .into(),
            '3' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
                MorseCode::Dash,
            ])
            .into(),
            '4' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dash,
            ])
            .into(),
            '5' => MorseWord::from([
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            '6' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            '7' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            '8' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
                MorseCode::Dot,
            ])
            .into(),
            '9' => MorseWord::from([
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dash,
                MorseCode::Dot,
            ])
            .into(),
            _ => return Err(()),
        };

        Ok(word)
    }
}

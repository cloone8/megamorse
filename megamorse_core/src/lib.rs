#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseCode {
    Dot,
    Dash,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseSequence {
    Code(MorseCode),
    Pause,
}

impl MorseCode {
    const fn to_bit(self) -> u8 {
        match self {
            MorseCode::Dot => 0,
            MorseCode::Dash => 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MorseWord<const N: usize> {
    code: u8,
}

macro_rules! from_word {
    ($num:literal) => {
        impl From<[MorseCode; $num]> for MorseWord<$num> {
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

impl<const N: usize> From<MorseWord<N>> for [MorseCode; N] {
    /// Create an array of MorseCode values from a MorseWord.
    fn from(word: MorseWord<N>) -> Self {
        word.to_array()
    }
}

impl<const N: usize> MorseWord<N> {
    const fn new(codes: [MorseCode; N]) -> Self {
        debug_assert!(N <= 5);

        let mut code: u8 = 0;

        if N >= 1 {
            code |= codes[0].to_bit();
        }

        if N >= 2 {
            code |= codes[1].to_bit() << 1;
        }

        if N >= 3 {
            code |= codes[2].to_bit() << 2;
        }

        if N >= 4 {
            code |= codes[3].to_bit() << 3;
        }

        if N == 5 {
            code |= codes[4].to_bit() << 4;
        }

        MorseWord { code }
    }

    pub const fn to_array(self) -> [MorseCode; N] {
        let mut codes = [MorseCode::Dot; N];

        if N >= 1 {
            codes[0] = if self.code & 0b0000_0001 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if N >= 2 {
            codes[1] = if self.code & 0b0000_0010 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if N >= 3 {
            codes[2] = if self.code & 0b0000_0100 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if N >= 4 {
            codes[3] = if self.code & 0b0000_1000 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        if N == 5 {
            codes[4] = if self.code & 0b0001_0000 == 0 {
                MorseCode::Dot
            } else {
                MorseCode::Dash
            };
        }

        codes
    }

    pub const fn len(&self) -> usize {
        N
    }

    pub const fn is_empty(&self) -> bool {
        N == 0
    }
}

impl MorseWord<1> {
    pub const fn to_sequence(self) -> [MorseSequence; 1] {
        let codes = self.to_array();

        [MorseSequence::Code(codes[0])]
    }
}

impl MorseWord<2> {
    pub const fn to_sequence(self) -> [MorseSequence; 3] {
        let codes = self.to_array();

        [
            MorseSequence::Code(codes[0]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[1]),
        ]
    }
}

impl MorseWord<3> {
    pub const fn to_sequence(self) -> [MorseSequence; 5] {
        let codes = self.to_array();

        [
            MorseSequence::Code(codes[0]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[1]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[2]),
        ]
    }
}

impl MorseWord<4> {
    pub const fn to_sequence(self) -> [MorseSequence; 7] {
        let codes = self.to_array();

        [
            MorseSequence::Code(codes[0]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[1]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[2]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[3]),
        ]
    }
}

impl MorseWord<5> {
    pub const fn to_sequence(self) -> [MorseSequence; 9] {
        let codes = self.to_array();

        [
            MorseSequence::Code(codes[0]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[1]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[2]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[3]),
            MorseSequence::Pause,
            MorseSequence::Code(codes[4]),
        ]
    }
}

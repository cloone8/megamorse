#![no_std]

#[doc(inline)]
pub use megamorse_core::*;

#[doc(inline)]
pub use megamorse_proc_macro::*;

enum MorseUnsized {
    One(MorseWord<1>),
    Two(MorseWord<2>),
    Three(MorseWord<3>),
    Four(MorseWord<4>),
    Five(MorseWord<5>),
}

impl From<MorseWord<1>> for MorseUnsized {
    fn from(word: MorseWord<1>) -> Self {
        MorseUnsized::One(word)
    }
}

impl From<MorseWord<2>> for MorseUnsized {
    fn from(word: MorseWord<2>) -> Self {
        MorseUnsized::Two(word)
    }
}

impl From<MorseWord<3>> for MorseUnsized {
    fn from(word: MorseWord<3>) -> Self {
        MorseUnsized::Three(word)
    }
}

impl From<MorseWord<4>> for MorseUnsized {
    fn from(word: MorseWord<4>) -> Self {
        MorseUnsized::Four(word)
    }
}

impl From<MorseWord<5>> for MorseUnsized {
    fn from(word: MorseWord<5>) -> Self {
        MorseUnsized::Five(word)
    }
}

pub struct MorseChar {
    morse: MorseUnsized,
}

impl From<MorseUnsized> for MorseChar {
    fn from(morse: MorseUnsized) -> Self {
        MorseChar { morse }
    }
}

impl TryFrom<char> for MorseChar {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let as_lower = value.to_ascii_lowercase();

        let as_unsized: MorseUnsized = match as_lower {
            'a' => MorseWord::from(morse!(.-)).into(),
            'b' => MorseWord::from(morse!(-...)).into(),
            'c' => MorseWord::from(morse!(-.-.)).into(),
            'd' => MorseWord::from(morse!(-..)).into(),
            'e' => MorseWord::from(morse!(.)).into(),
            'f' => MorseWord::from(morse!(..-.)).into(),
            'g' => MorseWord::from(morse!(--.)).into(),
            'h' => MorseWord::from(morse!(....)).into(),
            'i' => MorseWord::from(morse!(..)).into(),
            'j' => MorseWord::from(morse!(.---)).into(),
            'k' => MorseWord::from(morse!(-.-)).into(),
            'l' => MorseWord::from(morse!(.-..)).into(),
            'm' => MorseWord::from(morse!(--)).into(),
            'n' => MorseWord::from(morse!(-.)).into(),
            'o' => MorseWord::from(morse!(---)).into(),
            'p' => MorseWord::from(morse!(.--.)).into(),
            'q' => MorseWord::from(morse!(--.-)).into(),
            'r' => MorseWord::from(morse!(.-.)).into(),
            's' => MorseWord::from(morse!(...)).into(),
            't' => MorseWord::from(morse!(-)).into(),
            'u' => MorseWord::from(morse!(..-)).into(),
            'v' => MorseWord::from(morse!(...-)).into(),
            'w' => MorseWord::from(morse!(.--)).into(),
            'x' => MorseWord::from(morse!(-..-)).into(),
            'y' => MorseWord::from(morse!(-.--)).into(),
            'z' => MorseWord::from(morse!(--..)).into(),
            '0' => MorseWord::from(morse!(-----)).into(),
            '1' => MorseWord::from(morse!(.----)).into(),
            '2' => MorseWord::from(morse!(..---)).into(),
            '3' => MorseWord::from(morse!(...--)).into(),
            '4' => MorseWord::from(morse!(....-)).into(),
            '5' => MorseWord::from(morse!(.....)).into(),
            '6' => MorseWord::from(morse!(-....)).into(),
            '7' => MorseWord::from(morse!(--...)).into(),
            '8' => MorseWord::from(morse!(---..)).into(),
            '9' => MorseWord::from(morse!(----.)).into(),
            _ => return Err(()),
        };

        Ok(as_unsized.into())
    }
}

pub struct MorsePlayer<Play, Pause>
where
    Play: Fn(usize),
    Pause: Fn(usize),
{
    time_unit: usize,
    play: Play,
    pause: Pause,
}

pub enum MorsePlayerError {
    InvalidCharacter,
}

impl<Play, Pause> MorsePlayer<Play, Pause>
where
    Play: Fn(usize),
    Pause: Fn(usize),
{
    pub fn new(time_unit: usize, play: Play, pause: Pause) -> Self {
        MorsePlayer {
            time_unit,
            play,
            pause,
        }
    }

    fn play_word(&self, word: &str) -> Result<(), MorsePlayerError> {
        for (index, c) in word.chars().enumerate() {
            let morse_char = match MorseChar::try_from(c) {
                Ok(morse_char) => morse_char,
                Err(_) => return Err(MorsePlayerError::InvalidCharacter),
            };

            if index != 0 {
                (self.pause)(self.time_unit * 3);
            }

            let seq_len: usize;
            let mut seq_padded = [MorseSequence::Pause; 9];

            match morse_char.morse {
                MorseUnsized::One(morse) => {
                    seq_len = 1;
                    seq_padded[..seq_len].copy_from_slice(&morse.to_sequence());
                }
                MorseUnsized::Two(morse) => {
                    seq_len = 3;
                    seq_padded[..seq_len].copy_from_slice(&morse.to_sequence());
                }
                MorseUnsized::Three(morse) => {
                    seq_len = 5;
                    seq_padded[..seq_len].copy_from_slice(&morse.to_sequence());
                }
                MorseUnsized::Four(morse) => {
                    seq_len = 7;
                    seq_padded[..seq_len].copy_from_slice(&morse.to_sequence());
                }
                MorseUnsized::Five(morse) => {
                    seq_len = 9;
                    seq_padded[..seq_len].copy_from_slice(&morse.to_sequence());
                }
            };

            (0..seq_len).for_each(|i| match seq_padded[i] {
                MorseSequence::Code(code) => match code {
                    MorseCode::Dot => (self.play)(self.time_unit),
                    MorseCode::Dash => (self.play)(self.time_unit * 3),
                },
                MorseSequence::Pause => {
                    (self.pause)(self.time_unit);
                }
            });
        }

        Ok(())
    }

    pub fn play(&self, source: &str) -> Result<(), MorsePlayerError> {
        let words = source.split_whitespace();

        for (index, word) in words.enumerate() {
            if index != 0 {
                (self.pause)(self.time_unit * 7);
            }

            self.play_word(word)?;
        }

        Ok(())
    }
}

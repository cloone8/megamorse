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

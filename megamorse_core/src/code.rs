#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseCode {
    Dot,
    Dash,
}

impl MorseCode {
    pub(crate) const fn to_bit(self) -> u8 {
        match self {
            MorseCode::Dot => 0,
            MorseCode::Dash => 1,
        }
    }
}

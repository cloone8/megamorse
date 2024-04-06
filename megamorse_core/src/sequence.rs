use crate::MorseCode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseSequence {
    Code(MorseCode),
    Pause,
}

use crate::MorseCode;

/// Enum representing a single playable Morse code time unit.
/// 
/// A time unit can either be a Morse code symbol (a dot or a dash),
/// or a pause between Morse code symbols.
/// 
/// There is no need to handle this enum directly, as it is used
/// internally by the megamorse library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorseSequence {
    Code(MorseCode),
    Pause,
}

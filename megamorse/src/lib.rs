#![no_std]

#[doc(inline)]
pub use megamorse_core::*;

/// This macro is used to generate a static array of [MorseWord] structs from a literal whitespace-delimited sequence of dots and dashes.
/// It's main use should be generating compile-time Morse code sequences.
/// It accepts the characters `.` for a dot, and `-` or `_` for a dash.
/// 
/// Every sequence of up to five characters will be converted to a [MorseWord] struct, as each alphanumeric character is
/// mapped to a sequence of up to five Morse code characters.
/// 
/// # Examples
/// 
/// ```
/// use megamorse::morse;
/// 
/// // Maps to the Morse words representing "S" "O" "S"
/// const SOS: [MorseWord; 3] = morse!(... ___ ...);
/// ```
pub use megamorse_proc_macro::morse;

pub trait MorseDecoder {
    type Error;
    fn on(&mut self, timeunits: usize) -> Result<(), Self::Error>;
    fn off(&mut self, timeunits: usize) -> Result<(), Self::Error>;
}

pub struct MorsePlayer<T: MorseDecoder> {
    decoder: T,
}

#[derive(Debug)]
pub enum MorsePlayerError<DecoderError> {
    InvalidCharacter,
    DecoderError(DecoderError),
}

impl<DecoderError> From<DecoderError> for MorsePlayerError<DecoderError> {
    fn from(error: DecoderError) -> Self {
        MorsePlayerError::DecoderError(error)
    }
}

impl<T: MorseDecoder> MorsePlayer<T> {
    pub fn new(decoder: T) -> Self {
        MorsePlayer { decoder }
    }

    fn play_str_word(&mut self, word: &str) -> Result<(), MorsePlayerError<T::Error>> {
        for (index, c) in word.chars().enumerate() {
            if index != 0 {
                self.decoder.off(3)?;
            }

            let mword = MorseWord::try_from(c).map_err(|_| MorsePlayerError::InvalidCharacter)?;

            self.play_word(mword)?;
        }

        Ok(())
    }

    pub fn play_str(&mut self, source: &str) -> Result<(), MorsePlayerError<T::Error>> {
        let words = source.split_whitespace();

        for (index, word) in words.enumerate() {
            if index != 0 {
                self.decoder.off(7)?;
            }

            self.play_str_word(word)?;
        }

        Ok(())
    }

    pub fn play_word(&mut self, word: MorseWord) -> Result<(), MorsePlayerError<T::Error>> {
        let (seq_len, seq_padded) = word.to_sequence();

        seq_padded
            .into_iter()
            .take(seq_len)
            .try_for_each(|seq| match seq {
                MorseSequence::Code(code) => match code {
                    MorseCode::Dot => self.decoder.on(1),
                    MorseCode::Dash => self.decoder.on(3),
                },
                MorseSequence::Pause => self.decoder.off(1),
            })?;

        Ok(())
    }
}

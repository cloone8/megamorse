#![no_std]

#[doc(inline)]
pub use megamorse_core::*;

#[doc(inline)]
pub use megamorse_proc_macro::*;

pub trait MorseDecoder {
    type Error;
    fn on(&mut self, timeunits: usize);
    fn off(&mut self, timeunits: usize);
}

pub struct MorsePlayer<T: MorseDecoder> {
    decoder: T,
}

#[derive(Debug)]
pub enum MorsePlayerError<DecoderError> {
    InvalidCharacter,
    DecoderError(DecoderError),
}

impl<T: MorseDecoder> MorsePlayer<T>
{
    pub fn new(decoder: T) -> Self {
        MorsePlayer {
            decoder
        }
    }

    fn play_str_word(&mut self, word: &str) -> Result<(), MorsePlayerError<T::Error>> {
        for (index, c) in word.chars().enumerate() {
            if index != 0 {
                self.decoder.off(3);
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
                self.decoder.off(7);
            }

            self.play_str_word(word)?;
        }

        Ok(())
    }

    pub fn play_word(&mut self, word: MorseWord) -> Result<(), MorsePlayerError<T::Error>> {
        let (seq_len, seq_padded) = word.to_sequence();

        (0..seq_len).for_each(|i| match seq_padded[i] {
            MorseSequence::Code(code) => match code {
                MorseCode::Dot => self.decoder.on(1),
                MorseCode::Dash => self.decoder.on(3),
            },
            MorseSequence::Pause => self.decoder.off(1)
        });

        Ok(())
    }
}

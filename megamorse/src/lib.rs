#![no_std]

#[doc(inline)]
pub use megamorse_core::*;

/// This macro is used to generate a static array of [MorseWord] structs from a
/// literal whitespace-delimited sequence of dots and dashes.
/// It's main use should be generating compile-time Morse code sequences.
/// It accepts the characters `.` for a dot, and `-` or `_` for a dash.
///
/// Every sequence of up to five characters will be converted to a [MorseWord] struct,
/// as each alphanumeric character is mapped to a sequence of up to five Morse
/// code characters.
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

/// Trait representing a Morse code decoder.
/// Can be used to construct a [MorsePlayer], which
/// then uses the decoder to play Morse code sequences.
///
/// The decoder is responsible for interpreting the
/// Morse code sequences and translating them into
/// actions, such as turning on and off a light or
/// a buzzer, or sending signals to any other type
/// of application or program.
pub trait MorseDecoder {
    /// The error type that the decoder can return.
    type Error;

    /// Set the decoder output to "on" for a given number of time units.
    /// After the time units have passed, the output should be turned off.
    ///
    /// # Arguments
    ///
    /// * `timeunits` - The number of time units to keep the output on for.
    ///
    /// # Examples
    ///
    /// ```
    /// fn on(&mut self, timeunits: usize) -> Result<(), Self::Error> {
    ///     let led = get_led();
    ///     // Turn the output on for the specified number of time units.
    ///     led.on();
    ///     // Now wait for the specified number of time units.
    ///     let timeunit_ms = 200;
    ///     sleep(timeunits * timeunit_ms);
    ///     // Turn the output off, and return control to the player.
    ///     led.off();
    ///     Ok(())
    /// }
    fn on(&mut self, timeunits: usize) -> Result<(), Self::Error>;

    /// Pause the decoder output for a given number of time units.
    ///
    /// The function should ensure that it does not return until the
    /// specified number of time units have passed.
    ///
    /// This is mainly used to keep the [MorsePlayer] flexible, as
    /// sleep functionality can be implemented in different ways
    /// depending on the target platform.
    ///
    /// # Arguments
    ///
    /// * `timeunits` - The number of time units to pause for.
    ///
    /// # Examples
    ///
    /// ```
    /// fn off(&mut self, timeunits: usize) -> Result<(), Self::Error> {
    ///     // Sleep for the specified number of time units, and
    ///     // return control to the player once the sleep has completed.
    ///     let timeunit_ms = 200;
    ///     sleep(timeunits * timeunit_ms);
    ///     Ok(())
    /// }
    /// ````
    ///
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

# megamorse

A simple and flexible no_std-by-default morse code library for Rust.

[![crate](https://img.shields.io/crates/v/megamorse.svg)](https://crates.io/crates/megamorse)
[![documentation](https://docs.rs/megamorse/badge.svg)](https://docs.rs/megamorse)
[![license](https://img.shields.io/crates/l/megamorse.svg)](https://crates.io/crates/megamorse)

## Usage

See the [documentation](https://docs.rs/megamorse) for usage information.

An example for the SparkFun Pro Micro board:

```rust
#![no_std]
#![no_main]

use arduino_hal::port::{mode, Pin, PinOps};
use megamorse::{morse, MorseDecoder, MorsePlayer};
use panic_halt as _;

struct MorseLedDecoder<'a, P: PinOps> {
    timeunit: u16,
    led: &'a mut Pin<mode::Output, P>,
}

// The magic: We create a decoder
// that listens for commands from the megamorse library,
// and controls the LED on the SparkFun Pro Micro board.
impl<P: PinOps> MorseDecoder for MorseLedDecoder<'_, P> {
    type Error = ();

    fn on(&mut self, timeunits: usize) -> Result<(), Self::Error> {
        self.led.set_low(); // NOTE: This board uses low as on, and high as off
        arduino_hal::delay_ms(self.timeunit * (timeunits as u16));
        self.led.set_high();

        Ok(())
    }

    fn off(&mut self, timeunits: usize) -> Result<(), Self::Error>{
        self.led.set_high();
        arduino_hal::delay_ms(self.timeunit * (timeunits as u16));

        Ok(())
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut led = pins.led_tx.into_output();

    // High is off, low is on
    led.set_high();

    let decoder = MorseLedDecoder {
        timeunit: 100, // A timeunit is "100". In the decoder, we will choose to interpret this as milliseconds.
        led: &mut led,
    };

    let mut player = MorsePlayer::new(decoder);

    // Will blink "Hello world" in morse code
    player.play_str("Hello world!").unwrap();

    // Or you can construct a literal morse code
    let sos = morse!(... ___ ...);
    
    for word in sos.into_iter() {
       player.play_word(word).unwrap();
   }

    loop {}
}
```

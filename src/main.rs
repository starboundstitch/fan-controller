#![no_std]
#![no_main]

use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
};

#[arduino_hal::entry]
fn main() -> ! {
    // Object which holds everything on the arduino (eeprom, CPU, etc)
    let dp = arduino_hal::Peripherals::take().unwrap();
    // Object which holds all the pins on the Arduino
    let pins = arduino_hal::pins!(dp);

    // Assign a pin as an output
    let mut led = pins.d13.into_output();

    // Configure I2C
    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    // I2C interface
    let interface = I2CDisplayInterface::new(i2c);

    // Configure the display
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Font and text color from the embedded_graphics library
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    // Write Text to the display
    Text::with_baseline("Hello world!", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    // Write Text to the display
    Text::with_baseline("Hello Rust!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    // Flush to the display
    display.flush().unwrap();

    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

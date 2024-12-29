#![no_std]
#![no_main]

use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{PrimitiveStyle, Rectangle},
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

    // Analog Input
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    // Configure Voltage Input
    let voltage_pin = pins.a3.into_analog_input(&mut adc);

    // PWM Pin
    let tc1 = dp.TC1;
    tc1.tccr1a
        .write(|w| w.wgm1().bits(0b01).com1a().match_clear());
    tc1.tccr1b
        .write(|w| w.wgm1().bits(0b01).cs1().prescale_64());
    pins.d9.into_output();

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

    // Fill
    let fill = PrimitiveStyle::with_fill(BinaryColor::Off);

    // Write Text to the display
    Text::with_baseline("Fan Speed:", Point::zero(), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    loop {
        led.toggle();
        // Voltage to Display
        let voltage = voltage_pin.analog_read(&mut adc);
        let mut buffer = [0u8; 4];
        base_10_bytes(voltage.into(), &mut buffer);
        //* TEXT DISPLAY *//

        // Reset Updating Display Area
        Rectangle::new(Point::new(0, 16), Size::new(50, 20))
            .into_styled(fill)
            .draw(&mut display)
            .unwrap();

        // Draw Duty Cycle
        Text::with_baseline(
            unsafe { core::str::from_utf8_unchecked(&buffer) },
            Point::new(0, 16),
            text_style,
            Baseline::Top,
        )
        .draw(&mut display)
        .unwrap();

        // Flush Display
        display.flush().unwrap();

        // PWM STUFFS
        tc1.ocr1a.write(|w| w.bits((voltage / 3) as u16));

        arduino_hal::delay_ms(1000);
    }
}

fn base_10_bytes(mut n: u64, buf: &mut [u8]) -> &[u8] {
    if n == 0 {
        return b"0";
    }
    let mut i = 0;
    while n > 0 {
        buf[i] = (n % 10) as u8 + b'0';
        n /= 10;
        i += 1;
    }
    let slice = &mut buf[..i];
    slice.reverse();
    &*slice
}

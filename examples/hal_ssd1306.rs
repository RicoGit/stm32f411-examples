//! SSD1306 - monochrome OLED display
//!
//! ```bash
//! cargo run --example hal_ssd1306

#![allow(clippy::empty_loop)]
#![no_std]
#![no_main]

use stm32f4xx_hal as hal;

use panic_semihosting as _;

use cortex_m_rt::ExceptionFrame;
use cortex_m_rt::{entry, exception};
use embedded_graphics::{
    mono_font::{ascii::FONT_5X8, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

use core::fmt;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Arc, Polyline, PrimitiveStyle};
use hal::{i2c::I2c, pac, prelude::*};
use heapless::String;
use stm32f4xx_hal::delay::Delay;

// dimensions of SSD1306 OLED display known to work
pub const SCREEN_WIDTH: i32 = 128;
pub const SCREEN_HEIGHT: i32 = 64;
pub const FONT_HEIGHT: i32 = 8;
pub const FONT_WIDTH: i32 = 5;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        let mut delay_source = Delay::new(cp.SYST, &clocks);

        // Set up I2C1: SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8;
        let sda = gpiob.pb9;
        let i2c = I2c::new(dp.I2C1, (scl, sda), 400.khz(), &clocks);

        // Set up the display
        let interface = I2CDisplayInterface::new(i2c);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();
        display.init().unwrap();

        let mut format_buf = String::<30>::new();
        loop {
            //display clear
            display.clear();

            format_buf.clear();
            if fmt::write(
                &mut format_buf,
                format_args!("{}", "Hello world from Rust!!!!"),
            )
            .is_ok()
            {
                // draw test

                let text_style = MonoTextStyleBuilder::new()
                    .font(&FONT_5X8)
                    .text_color(BinaryColor::On)
                    .build();

                Text::new(&format_buf, Point::new(3, FONT_HEIGHT), text_style)
                    .draw(&mut display)
                    .unwrap();

                // draw screen frame (rectangle)

                let points: [Point; 5] = [
                    Point::new(0, 0),
                    Point::new(SCREEN_WIDTH - 1, 0),
                    Point::new(SCREEN_WIDTH - 1, SCREEN_HEIGHT - 1),
                    Point::new(0, SCREEN_HEIGHT - 1),
                    Point::new(0, 0),
                ];

                let line_style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

                Polyline::new(&points)
                    .into_styled(line_style)
                    .draw(&mut display)
                    .unwrap();
            }
            display.flush().unwrap();
            //delay a little while between refreshes so the display is readable
            delay_source.delay_ms(10u8);
        }
    }

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

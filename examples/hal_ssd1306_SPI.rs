//! SSD1306 - monochrome OLED display
//!
//! ```bash
//! cargo run --example hal_ssd1306
//! ```
//!
//! SDA (MOSI) - PC3
//! SCK (SCK)  - PB10
//! RES        - PB4
//! DC         - PB5

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
use cortex_m_semihosting::hprintln;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::primitives::{Arc, Polyline, PrimitiveStyle};
use hal::{i2c::I2c, pac, prelude::*};
use heapless::String;
use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::spi::{Mode, NoMiso, NoMosi, Phase, Polarity, Spi};

// dimensions of SSD1306 OLED display known to work
pub const SCREEN_WIDTH: i32 = 128;
pub const SCREEN_HEIGHT: i32 = 64;
pub const FONT_HEIGHT: i32 = 8;
pub const FONT_WIDTH: i32 = 5;

#[entry]
fn main() -> ! {
    hprintln!("start").unwrap();

    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        let mut delay = Delay::new(cp.SYST, &clocks);

        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();

        // SPI2
        let sck = gpiob.pb10.into_alternate();
        let mosi = gpioc.pc3.into_alternate();
        let mut res = gpiob.pb4.into_push_pull_output();
        let dc = gpiob.pb5.into_push_pull_output();

        let spi2 = Spi::new(
            dp.SPI2,
            (sck, NoMiso {}, mosi), // SCK, MISO, MOSI
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            18.mhz(),
            &clocks,
        );
        // Set up the display
        let interface = SPIInterfaceNoCS::new(spi2, dc);
        let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
            .into_buffered_graphics_mode();

        display.reset(&mut res, &mut delay).unwrap();
        display.init().unwrap();


        let mut format_buf = String::<30>::new();
        let mut position = 0;

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

                Text::new(&format_buf, Point::new(3, position), text_style)
                    .draw(&mut display)
                    .unwrap();

                if position > SCREEN_HEIGHT {
                    position = 0;
                } else {
                    position += 1;
                }

            }

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
            
            display.flush().unwrap();
            //delay a little while between refreshes so the display is readable
            delay.delay_ms(50u16);
        }
    }

    loop {}
}

#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

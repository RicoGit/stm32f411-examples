//! DS18B20 is 1-wire temperature sensor
//!
//! `cargo run --example hal_onewire`
//!
//! todo not working - raise error: CrcMismatch(201, 255)

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use onewire::{ds18b20, DeviceSearch, OneWire, DS18B20};

use crate::hal::{pac, prelude::*};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    hprintln!("start configure").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(86.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, &clocks);

    let gpioc = dp.GPIOC.split();
    let mut pin = gpioc.pc10.into_open_drain_output().internal_pull_up(true);

    let mut wire = OneWire::new(&mut pin, false);
    wire.reset(&mut delay).unwrap();

    // search for devices
    let mut search = DeviceSearch::new();

    let sensor = loop {

        match wire.search_next(&mut search, &mut delay).unwrap() {
            None => {
                hprintln!("No one device was found.").unwrap();
                asm::delay(100_000)
            }
            Some(device) =>
                break DS18B20::new::<()>(device).unwrap(),
        };
    };


    hprintln!("start loop").unwrap();

    loop {
        // request sensor to measure temperature
        let resolution = sensor.measure_temperature(&mut wire, &mut delay).unwrap();
        // wait for compeltion, depends on resolution
        delay.delay_ms(resolution.time_ms());

        let temp = sensor.read_temperature(&mut wire, &mut delay).unwrap();

        hprintln!("Temperature = {} deg C", temp);

        asm::delay(100_000_000);
    }
}

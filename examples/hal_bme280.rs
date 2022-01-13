//! BME280 is temp, hum, press i2c sensor
//!
//! `cargo run --example hal_bme280`


#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use bme280::BME280;
use cortex_m::asm;
use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;

use stm32f4xx_hal as hal;
use stm32f4xx_hal::delay::Delay;
use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    hprintln!("start configure").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(86.mhz()).freeze();
    let delay = Delay::new(cp.SYST, &clocks);


    let i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (
            gpiob.pb8, //scl
            gpiob.pb9, //sda
        ),
        900.khz(), // bme should support 1 Mhz, but it doesn't work for me
        &clocks
    );

    // initialize the BME280 using the primary I2C address 0x76
    let mut bme280 = BME280::new_primary(i2c, delay);

    // initialize the sensor
    bme280.init().unwrap();

    hprintln!("start loop").unwrap();

    loop {
        // // measure temperature, pressure, and humidity
        let measurements = bme280.measure().unwrap();

        hprintln!("Relative Humidity = {}%", measurements.humidity);
        hprintln!("Temperature = {} deg C", measurements.temperature);
        hprintln!("Pressure = {} mmhg", measurements.pressure / 133_f32);

       asm::delay(100_000_000);

    }
}

//! BME280 is temp, hum, press i2c sensor
//!
//! This example of using raw i2c protocol (no bme280 driver).
//! see datasheet: https://www.bosch-sensortec.com/media/boschsensortec/downloads/datasheets/bst-bme280-ds002.pdf
//!
//! `cargo run --example hal_bme280`

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use bme280::BME280;
use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use crate::hal::{pac, prelude::*};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::delay::Delay;

#[entry]
fn main() -> ! {
    hprintln!("start configure").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let gpiob = dp.GPIOB.split();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(86.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, &clocks);

    let mut i2c = hal::i2c::I2c::new(
        dp.I2C1,
        (
            gpiob.pb8, //scl
            gpiob.pb9, //sda
        ),
        400.khz(), // bme should support 1 Mhz, but it doesn't work for me
        &clocks,
    );

    const BME280_I2C_ADDR_PRIMARY: u8 = 0x76;
    const BME280_I2C_CHIP_ID_ADDR: u8 = 0xD0;

    let mut chip_id_buffer: [u8; 2] = [0, 0];

    // read chip id

    i2c.write_read(
        BME280_I2C_ADDR_PRIMARY,
        &[BME280_I2C_CHIP_ID_ADDR],
        &mut chip_id_buffer,
    )
    .unwrap();

    hprintln!("chip id: {}", chip_id_buffer[0]).unwrap();

    // read temperature

    const BME280_I2C_TEMP_MSB_ADDR: u8 = 0xFA; // most significant bits (MSB) raw temperature measurement (signed short)
    const BME280_I2C_TEMP_LSB_ADDR: u8 = 0xFB; // less significant bits (LSB) raw temperature measurement (signed short)

    hprintln!("start loop").unwrap();

    let mut temp_buffer: [u8; 3] = [0, 0, 0];

    loop {
        // temperature: read 2 bytes started from BME280_I2C_TEMP_MSB_ADDR
        i2c.write_read(
            BME280_I2C_ADDR_PRIMARY,
            &[BME280_I2C_TEMP_MSB_ADDR],
            &mut temp_buffer,
        )
        .unwrap();

        let data_msb: u32 = (temp_buffer[0] as u32) << 12;
        let data_lsb: u32 = (temp_buffer[1] as u32) << 4;
        let data_xlsb: u32 = (temp_buffer[2] as u32) >> 4;
        let temp = data_msb | data_lsb | data_xlsb;

        // returned temperature doesn't look correct, looks like calibration needed)
        hprintln!("Temperature = {} C", temp);

        delay.delay_ms(2000u16);
    }
}

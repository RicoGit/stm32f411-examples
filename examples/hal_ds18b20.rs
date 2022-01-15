//! DS18B20 is 1-wire temperature sensor
//!
//! example from [here](https://github.com/fuchsnj/ds18b20)
//!
//! `cargo run --example hal_ds18b20`

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::fmt::{Debug, Write};
use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use ds18b20::{Ds18b20, Resolution};
use one_wire_bus::{OneWire, OneWireResult};

use crate::hal::{pac, prelude::*};
use stm32f4xx_hal as hal;
use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::hal::blocking::delay::{DelayMs, DelayUs};
use stm32f4xx_hal::hal::digital::v2::{InputPin, IoPin, OutputPin, PinState};

#[entry]
fn main() -> ! {
    hprintln!("start configure").unwrap();

    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(86.mhz()).freeze();
    let mut delay = Delay::new(cp.SYST, &clocks);

    let gpioc = dp.GPIOC.split();
    let pin = gpioc.pc10.into_open_drain_output().internal_pull_up(true);

    let mut one_wire = OneWire::new(pin).unwrap();

    hprintln!("start loop").unwrap();

    loop {
        get_temperature(&mut delay, &mut one_wire);
        asm::delay(100_000_000);
    }
}

fn get_temperature<P, E>(
    delay: &mut (impl DelayUs<u16> + DelayMs<u16>),
    one_wire_bus: &mut OneWire<P>,
) -> OneWireResult<(), E>
where
    P: OutputPin<Error = E> + InputPin<Error = E>,
    E: Debug,
{
    // initiate a temperature measurement for all connected devices
    ds18b20::start_simultaneous_temp_measurement(one_wire_bus, delay)?;

    // wait until the measurement is done. This depends on the resolution you specified
    // If you don't know the resolution, you can obtain it from reading the sensor data,
    // or just wait the longest time, which is the 12-bit resolution (750ms)
    Resolution::Bits12.delay_for_measurement_time(delay);

    // iterate over all the devices, and report their temperature
    let mut search_state = None;
    loop {
        if let Some((device_address, state)) =
            one_wire_bus.device_search(search_state.as_ref(), false, delay)?
        {
            search_state = Some(state);
            if device_address.family_code() != ds18b20::FAMILY_CODE {
                // skip other devices
                continue;
            }
            // You will generally create the sensor once, and save it for later
            let sensor = Ds18b20::new(device_address)?;

            // contains the read temperature, as well as config info such as the resolution used
            let sensor_data = sensor.read_data(one_wire_bus, delay)?;

            hprintln!(
                "Device at {:?} is {}°C",
                device_address,
                sensor_data.temperature
            );
        } else {
            hprint!(".");
            break;
        }
    }
    Ok(())
}

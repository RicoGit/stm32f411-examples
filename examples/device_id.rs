//! `cargo run --example device_id`

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use core::any::Any;
use panic_semihosting as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::Peripherals;
use cortex_m_rt::{entry, exception};
use cortex_m_semihosting::hprintln;
use stm32f4::stm32f411;

#[entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();
    let dp = stm32f411::Peripherals::take().unwrap();

    let id = dp.DBGMCU.idcode.read().bits();
    hprintln!("Device id: {:?}", id);
    hprintln!("Device id: {:b}", id);

    loop {}
}

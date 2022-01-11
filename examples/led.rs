//! No hal example
//!
//! `cargo run --example led`

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
use panic_semihosting as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use stm32f4::stm32f411;
use stm32f4::stm32f411::{GPIOB, interrupt, Interrupt, NVIC};

use cortex_m::asm::delay;

#[entry]
fn main() -> ! {
    let dp = stm32f411::Peripherals::take().unwrap();

    //  Enable the clock to GPIOA
    dp.RCC.ahb1enr.modify(|_,w| w.gpioaen().set_bit());

    // Configure led (PA5)
    dp.GPIOA.moder.modify(|_, w| w.moder5().output());
    dp.GPIOA.pupdr.modify(|_, w| w.pupdr5().pull_down());

    loop {
        dp.GPIOA.odr.write(|w| w.odr5().high());
        hprint!("+").unwrap();
        delay(10_000_000);

        dp.GPIOA.odr.write(|w| w.odr5().low());
        hprint!("-").unwrap();
        delay(10_000_000);
    }
}



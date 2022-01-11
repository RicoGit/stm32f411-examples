//! No hal example
//!
//! `cargo run --example led_and_btn`

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
use panic_semihosting as _;

use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use stm32f4::stm32f411;
use stm32f4::stm32f411::{GPIOB, interrupt, Interrupt, NVIC};
use stm32f4::stm32f411::gpiob::moder::MODER13_A;

use cortex_m::asm::delay;

#[entry]
fn main() -> ! {
    let dp = stm32f411::Peripherals::take().unwrap();

    //  Enable the clock to GPIOA and GPIOC
    dp.RCC.ahb1enr.modify(|_,w| w.gpioaen().set_bit());
    dp.RCC.ahb1enr.modify(|_,w| w.gpiocen().set_bit());

    // configure led (PA5)
    dp.GPIOA.moder.modify(|_, w| w.moder5().output());
    dp.GPIOA.pupdr.modify(|_, w| w.pupdr5().pull_down());
    
    // configure blue btn (PC13)
    dp.GPIOC.moder.modify(|_, w| w.moder13().input());
    dp.GPIOC.pupdr.modify(|_, w| w.pupdr13().pull_up());

    let mut btn_pushed = false;

    loop {

        if dp.GPIOC.idr.read().idr13().is_high() {
            btn_pushed = false;
        }
        
        if !btn_pushed && dp.GPIOA.idr.read().idr5().is_low() && dp.GPIOC.idr.read().idr13().is_low() {
            dp.GPIOA.odr.write(|w| w.odr5().high());
            btn_pushed = true;
            hprint!("-").unwrap();
        }


        if !btn_pushed && dp.GPIOA.idr.read().idr5().is_high() && dp.GPIOC.idr.read().idr13().is_low() {
            dp.GPIOA.odr.write(|w| w.odr5().low());
            btn_pushed = true;
            hprint!("+").unwrap();
        }

    }
}



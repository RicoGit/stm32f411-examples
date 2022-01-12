//! Button triggers interrupts
//!
//! `cargo run --example led_interrupt`

#![no_main]
#![no_std]

use cortex_m::asm::delay;
use cortex_m::peripheral::NVIC;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4::stm32f411;

#[allow(unused_extern_crates)]
use panic_semihosting as _;
use stm32f4::stm32f411::gpioh::moder::MODER5_R;
use stm32f4::stm32f411::{Interrupt, interrupt, exti};


static mut EXTI: Option<stm32f411::EXTI> = None;

#[entry]
fn main() -> ! {
    let p = cortex_m::peripheral::Peripherals::take().unwrap();
    let dp = stm32f411::Peripherals::take().unwrap();

    // clock
    dp.RCC.ahb1enr.modify(|_, w| w.gpioaen().enabled());
    dp.RCC.ahb1enr.modify(|_, w| w.gpiocen().enabled());

    // led
    dp.GPIOA.moder.modify(|_, w| w.moder5().output());
    // interrupt when led changes state
    unsafe { NVIC::unmask(Interrupt::EXTI9_5); }
    dp.EXTI.imr.modify(|_, w| w.mr5().set_bit());
    dp.EXTI.rtsr.modify(|_, w| w.tr5().set_bit());

    unsafe { EXTI = Option::Some(dp.EXTI) };

    loop {
        dp.GPIOA.odr.write(|w| w.odr5().high());
        delay(10_000_000);
        dp.GPIOA.odr.write(|w| w.odr5().low());
        delay(10_000_000);
    }
}


#[interrupt]
fn EXTI9_5() {
    // bad code with unsafe, use mutex instead
    unsafe {
        EXTI.as_ref().map(|e| e.pr.modify(|_, w| w.pr5().set_bit()));
    }
    hprintln!(".").unwrap();
}

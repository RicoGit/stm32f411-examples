//! No hal example
//!
//! `cargo run --example btn_interrupt`

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
use panic_semihosting as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use stm32f4::stm32f411;
use stm32f4::stm32f411::gpiob::moder::MODER13_A;
use stm32f4::stm32f411::{interrupt, Interrupt, EXTI, GPIOB, NVIC, GPIOC};

use cortex_m::asm::delay;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref MUTEX_GPIOC:  Mutex<RefCell<Option<GPIOC>>>  = Mutex::new(RefCell::new(None));
    static ref MUTEX_EXTI:  Mutex<RefCell<Option<EXTI>>>  = Mutex::new(RefCell::new(None));
}

#[entry]
unsafe fn main() -> ! {
    hprintln!("init start").unwrap();

    let dp = stm32f411::Peripherals::take().unwrap();

    // enable the clock for GPIOC
    dp.RCC.ahb1enr.write(|w| w.gpiocen().set_bit());
    // enable clock for SYSCFG
    dp.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    // configure blue btn (PC13)
    dp.GPIOC.moder.write(|w| w.moder13().input());
    dp.GPIOC.pupdr.write(|w| w.pupdr13().pull_up()); // pulled up by hardware as well

    dp.SYSCFG.exticr4.modify(|r, w| unsafe {
        // the same as make_interrupt_source in hal
        let offset = 4;
        w.bits((r.bits() & !(0xf << offset)) | (2 << offset))
    });
    // configure interrupts (see 10.2.4 Hardware interrupt selection)

    // 1. Configure the mask bits of the 13th interrupt line (EXTI_IMR)
    dp.EXTI.imr.modify(|_, w| w.mr13().set_bit());
    // 2. Configure the Trigger selection bits of the interrupt lines (EXTI_RTSR or EXTI_FTSR)
    dp.EXTI.rtsr.modify(|_, w| w.tr13().set_bit());
    // 3. Configure the enable and mask bits that control the NVIC IRQ channel mapped to the external interrupt controller (EXTI)
    unsafe { NVIC::unmask(Interrupt::EXTI15_10) } // pin 13 between 10 and 15

    cortex_m::interrupt::free(|cs| {
        MUTEX_EXTI.borrow(cs).replace(Some(dp.EXTI));
        MUTEX_GPIOC.borrow(cs).replace(Some(dp.GPIOC));
    });

    loop {
        delay(10_000_000);
        hprint!("|");
    }
}

#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let exti = MUTEX_EXTI.borrow(cs).borrow_mut();
        exti.as_ref().unwrap().pr.write(|w| w.pr13().set_bit());
        let gpioc = MUTEX_GPIOC.borrow(cs).borrow();
        hprint!("\n btn: {}", gpioc.as_ref().unwrap().idr.read().idr13().is_high());
    });
}

//! Hal example is much easier
//!
//! `cargo run --example hal_btn_interrupt`

#![no_main]
#![no_std]

#[allow(unused_extern_crates)]
use panic_semihosting as _;

use core::cell::RefCell;
use cortex_m::interrupt::Mutex;
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m_rt::entry;
use cortex_m_semihosting::{hprint, hprintln};
use stm32f4xx_hal::{prelude::*, hal};

use cortex_m::asm::delay;
use cortex_m::peripheral::NVIC;
use stm32f4::stm32f411;
use stm32f4::stm32f411::{interrupt, Interrupt};
use stm32f4xx_hal::gpio::{Edge, Input, Pin, PullDown, PullUp};
use stm32f4xx_hal::gpio::gpioc::PC13;

#[macro_use]
extern crate lazy_static;


type UserBtn = PC13<Input<PullUp>>;

lazy_static! {
    static ref MUTEX_BTN:  Mutex<RefCell<Option<UserBtn>>>  = Mutex::new(RefCell::new(None));
}

#[entry]
unsafe fn main() -> ! {
    hprintln!("init start").unwrap();

    let mut dp = stm32f411::Peripherals::take().unwrap();

    // enable the clock for GPIOC
    dp.RCC.ahb1enr.write(|w| w.gpiocen().enabled());
    // enable clock for SYSCFG
    dp.RCC.apb2enr.write(|w| w.syscfgen().enabled());

    // configure user btn (PC13)
    let mut gpioc = dp.GPIOC.split();
    let mut btn = gpioc.pc13.into_pull_up_input();

    btn.enable_interrupt(&mut dp.EXTI); // the same as dp.EXTI.imr.modify(|_, w| w.mr13().set_bit());
    btn.make_interrupt_source(&mut dp.SYSCFG.constrain());
    btn.trigger_on_edge(&mut dp.EXTI, Edge::Rising);

    // 3. Configure the enable and mask bits that control the NVIC IRQ channel mapped to the external interrupt controller (EXTI)
    unsafe { NVIC::unmask(Interrupt::EXTI15_10) } // pin 13 between 10 and 15

    cortex_m::interrupt::free(|cs| {
        MUTEX_BTN.borrow(cs).replace(Some(btn));
    });

    loop {
        delay(10_000_000);
        hprint!("|");
    }
}

#[interrupt]
fn EXTI15_10() {
    cortex_m::interrupt::free(|cs| {
        let mut btn = MUTEX_BTN.borrow(cs).borrow_mut();
        btn.as_mut().unwrap().clear_interrupt_pending_bit();
        hprint!("\n btn: {}", btn.as_ref().unwrap().is_high());
    });
}

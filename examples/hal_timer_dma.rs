//! Led blink with timer in DMA mode.
//!
//!  `cargo run --example hal_timer_dma`


#![no_main]

use cortex_m::peripheral::scb::VectActive::Interrupt;
use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use hal::timer;
use hal::timer::Timer;
use stm32f4xx_hal as hal;
use stm32f4xx_hal::dma::{StreamsTuple, Transfer};

use crate::hal::{pac, dma, prelude::*};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();

    // TIMER

    let clocks = rcc.cfgr.sysclk(24.mhz()).freeze();
    // Create a timer based on SysTick
    let mut timer = Timer::new(dp.TIM2, &clocks).counter_ms();


    // DMA

    // todo have no idea how to configure )
}


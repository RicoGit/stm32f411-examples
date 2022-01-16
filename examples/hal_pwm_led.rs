//! Smoothly blinks board led (PA5) via PWM signal.
//!
//! `cargo run --example hal_pwm_led`

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_semihosting as _;
use stm32f4xx_hal::delay::Delay;
use stm32f4xx_hal::{pac, prelude::*, timer::Timer};

#[entry]
fn main() -> ! {
        let c = cortex_m::peripheral::Peripherals::take().unwrap();
        let dp = pac::Peripherals::take().unwrap();
        // Set up the system clock.

        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        let mut delay = Delay::new(c.SYST, &clocks);

        let gpioa = dp.GPIOA.split();
        let channels = gpioa.pa5.into_alternate();

        // note that for nucleo-F411RE PA5 pin connected only with TIM2 timer
        let pwm = Timer::new(dp.TIM2, &clocks).pwm(channels, 20u32.khz());
        let mut channel = pwm;
        let max_duty = channel.get_max_duty();
        channel.set_duty(max_duty / 2);
        channel.enable();

        let mut counter: i32 = 0;
        let mut step: i32 = 1;
        loop {
            if counter as u16 >= max_duty {
                step = -1;
            }

            if counter <= 0 {
                step = 1;
            }

            counter += step;

            channel.set_duty(counter as u16);

            delay.delay_ms(1u8);
    }
}

//! Simple example of using RTic
//!
//! `cargo run --example hal_rtic`

#![deny(warnings)]
#![no_std]
#![no_main]

use panic_semihosting as _;

#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    use cortex_m::peripheral::NVIC;
    use cortex_m_semihosting::hprint;
    use stm32f4::stm32f411::Interrupt;
    use stm32f4xx_hal::{
        gpio::{gpioa::PA5, gpioc::PC13, Edge, Input, Output, PullUp, PushPull},
        prelude::*,
    };

    #[shared]
    struct Shared {}

    #[local]
    struct Local {
        button: PC13<Input<PullUp>>,
        led: PA5<Output<PushPull>>,
    }

    #[init]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut syscfg = ctx.device.SYSCFG.constrain();

        let gpioa = ctx.device.GPIOA.split();
        let led = gpioa.pa5.into_push_pull_output();

        let gpioc = ctx.device.GPIOC.split();
        let mut button = gpioc.pc13.into_pull_up_input();
        button.make_interrupt_source(&mut syscfg);
        button.enable_interrupt(&mut ctx.device.EXTI);
        button.trigger_on_edge(&mut ctx.device.EXTI, Edge::Falling);

        unsafe { NVIC::unmask(Interrupt::EXTI15_10) }

        (Shared {}, Local { button, led }, init::Monotonics())
    }

    #[task(binds = EXTI15_10, local = [button, led])]
    fn button_click(ctx: button_click::Context) {
        hprint!(".").unwrap();
        ctx.local.button.clear_interrupt_pending_bit();
        ctx.local.led.toggle();
    }
}

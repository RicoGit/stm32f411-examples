//! Prints "Hello, world!" on the host console using semihosting
//! cargo test --example hello

#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use stm32f4 as _;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!").unwrap();

    loop {
    }
}

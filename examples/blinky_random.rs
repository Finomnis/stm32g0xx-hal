#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32g0xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::prelude::*;
use hal::rng::Config;
use hal::stm32;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");
    let cp = cortex_m::Peripherals::take().expect("cannot take core peripherals");
    let mut rcc = dp.RCC.constrain();
    let mut delay = cp.SYST.delay(&rcc.clocks);

    let gpioa = dp.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();

    let mut rng = dp.RNG.enable(Config::default(), &mut rcc);
    let mut random_bytes = [0u16; 3];
    match rng.fill(&mut random_bytes) {
        Ok(()) => hprintln!("random bytes: {:?}", random_bytes).unwrap(),
        Err(err) => hprintln!("RNG error: {:?}", err).unwrap(),
    }
    loop {
        match rng.gen_range(20, 200) {
            Ok(period) => {
                led.toggle();
                delay.delay(period.ms());
            }
            Err(err) => hprintln!("RNG error: {:?}", err).unwrap(),
        }
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("Hard fault {:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

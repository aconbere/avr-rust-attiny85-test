#![feature(llvm_asm)]
#![no_main]
#![no_std]

extern crate panic_halt;

extern crate avr_device;

extern crate attiny85_hal as hal;

use hal::pac;

use pac::Peripherals;
use hal::prelude::*;

#[cfg(feature = "rt")]
use hal::entry;

type Delay = hal::delay::Delay<hal::clock::MHz8>;

#[entry]
fn main() -> ! {
    let mut delay = Delay::new();

    let dp = Peripherals::take().unwrap();

    let mut led = pac::PORTB::pb1::PB1.into_output(&mut dp.PORTB);

    loop {
        led.toggle.void_unwrap();
        delay.delay_us(500u16);
    }
}

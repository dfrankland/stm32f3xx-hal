//! Uses the StatefulOutputPin embedded_hal trait to toggle the pin
//! On the stm32 discovery board this is the red led
//! Target board: STM32F3DISCOVERY

#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_semihosting as _;

use cortex_m_rt::entry;
use stm32f3xx_hal::hal::digital::v2::OutputPin;
use stm32f3xx_hal::prelude::*;
use stm32f3xx_hal::stm32;

#[entry]
fn main() -> ! {
    let p = stm32::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let mut gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut led_red = gpioe
        .pe13
        .into_push_pull_output(&mut gpioe.moder, &mut gpioe.otyper);

    led_red.set_low().unwrap();

    loop {
        led_red.toggle().unwrap();
        cortex_m::asm::delay(8_000_000);
        // Toggle by hand.
        // Uses `StatefulOutputPin` instead of `ToggleableOutputPin`
        // which is logically the same.
        if led_red.is_set_low().unwrap() {
            led_red.set_high().unwrap();
        } else {
            led_red.set_low().unwrap();
        }
        cortex_m::asm::delay(8_000_000);
    }
}

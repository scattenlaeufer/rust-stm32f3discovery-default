#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 500_u16;

    // infinite loop; just so we don't leave this stack frame
    loop {
        for current in 0..8 {
            let next = (current + 1) % 8;
            leds[next].on();
            delay.delay_ms(half_period);
            leds[current].off();
            delay.delay_ms(half_period);
        }
    }
}

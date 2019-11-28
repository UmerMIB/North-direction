#![deny(unsafe_code)]
#![no_main]
#![no_std]

// You'll find this useful ;-)
use core::f32::consts::PI;

#[allow(unused_imports)]
use aux15::{entry, iprint, iprintln, prelude::*, Direction, I16x3};
use m::Float;

#[entry]
fn main() -> ! {
    let (mut leds, mut lsm303dlhc, mut delay, _itm) = aux15::init();

    loop {
        let I16x3 { x, y, .. } = lsm303dlhc.mag().unwrap();

        let theta = (y as f32).atan2(x as f32); // in radians

        if theta < -7. * PI / 8. {
            leds.iter_mut().for_each(|led| led.off());
            
            leds[Direction::North].on();
            
        } else{
            leds.iter_mut().for_each(|led| led.on());
            delay.delay_ms(200_u16);

            leds.iter_mut().for_each(|led| led.off());

        
        }
        delay.delay_ms(500_u16)


    }
}

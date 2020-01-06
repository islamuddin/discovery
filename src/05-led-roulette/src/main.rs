#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};


#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period:u16 = 50; 
    loop {
        // MY SOLUTION for 05-led-roulette : https://docs.rust-embedded.org/discovery/05-led-roulette/my-solution.html
        for curr in 0..8 {
            led_roulette(curr,8,half_period,&mut delay,&mut leds);            
        }

        // Practice LOGIC : round + tail follows off one by one
        // led_control(0,true,half_period,&mut delay,&mut leds);
        // led_control(0,false,half_period,&mut delay,&mut leds);

    }
}


fn led_roulette(c:i32, mut limit:i32,half_period:u16,delay:&mut Delay,leds:&mut Leds){
    if limit!=0{
        let mut n = c+1;
        if n<0 {
            n *=-1;
        }
        if limit<0 {
            limit *= -1;
        }
        while n>=limit {
            n -= limit;
        }
        leds[n as usize].on();
        delay.delay_ms(half_period);
        leds[c as usize].off();
        delay.delay_ms(half_period);            
    }
}

fn led_control(led_index:usize,led_status:bool,half_period:u16,delay:&mut Delay,leds:&mut Leds) {
    if led_index < 8 {
        match led_status {
            true => {
                leds[led_index].on();
                delay.delay_ms(half_period);
            },
            false => {
                leds[led_index].off();
                delay.delay_ms(half_period);
            },
        }

        led_control(led_index+1,led_status,half_period,delay,leds);
    }
}
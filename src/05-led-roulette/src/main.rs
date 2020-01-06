#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};


#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period:u16 = 50; 
    loop {
        // LOGIC : round + tail follows off
        led_control(0,true,half_period,&mut delay,&mut leds);
        led_control(0,false,half_period,&mut delay,&mut leds);

        // LOGIC : round + tail follow off
        for current in 0..8 {
            led_control((current + 1) % 8,true,half_period,&mut delay,&mut leds);
            led_control(current,false,half_period,&mut delay,&mut leds);
        }

        // MY SOLUTION for this : https://docs.rust-embedded.org/discovery/05-led-roulette/my-solution.html
        for curr in 0..8 {
            leds[(next(curr + 1,8)) as usize].on();
            delay.delay_ms(half_period);
            leds[curr as usize].off();
            delay.delay_ms(half_period);            
        }
    }
}


fn next(mut n:i32, mut divisor:i32)->i32{
    if divisor!=0{
    if n<0 {
        n *=-1;
    }
    if divisor<0 {
        divisor *= -1;
    }
    while n>=divisor {
        n -= divisor;
    }
    return n
    }
    0
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


// #![deny(unsafe_code)]
// #![no_main]
// #![no_std]

// use aux5::{entry, prelude::*, Delay, Leds};

// #[entry]
// fn main() -> ! {
//     let (mut delay, mut leds): (Delay, Leds) = aux5::init();

//     let ms = 50_u8;
//     loop {
//         for curr in 0..8 {
//             let next = (curr + 1) % 8;

//             leds[next].on();
//             delay.delay_ms(ms);
//             leds[curr].off();
//             delay.delay_ms(ms);
//         }
//     }
// }
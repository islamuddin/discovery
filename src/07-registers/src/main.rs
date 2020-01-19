#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux7::{entry, iprint, iprintln};

static PE8_ON: u32 = 8;
static PE8_OFF: u32 = PE8_ON+16;

static PE9_ON: u32 = 9;
static PE9_OFF: u32 = PE9_ON+16;


#[entry]
fn main() -> ! {
    aux7::init();    
    const GPIOE_BSRR: u32 = 0x48001018; // A magic address!
    unsafe {
        let led_switch=|a:u32| *(GPIOE_BSRR as *mut u32)= 1 << a;

        led_switch(PE9_ON);
        led_switch(PE8_ON);

        led_switch(PE9_OFF);
        led_switch(PE8_OFF);
    }

    loop {}
}
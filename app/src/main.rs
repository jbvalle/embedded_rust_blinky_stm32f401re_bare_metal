#![no_std]
#![no_main]
use stm32::peripherals::*;
use core::arch::asm;

mod isr_handlers;


const RCCX: *mut RCC = 0x40023800 as *mut RCC;
const GPIOA: *mut GPIOx = 0x40020000 as *mut GPIOx;

#[inline(always)]
pub fn wait_ms(time: u32) {
    for _ in 0..time {
        for _ in 0..1600 {
            unsafe { asm!(""); } // Prevent the loop from being optimized out
        }
    }
}


#[no_mangle]
pub extern "C" fn main() -> ! {

    unsafe{

        let rcc = &mut *RCCX;
        let gpioa = &mut *GPIOA;

        rcc.ahb1enr |= 1;
        
        gpioa.moder &= !(3 << (5 * 2));
        gpioa.moder |=  1 << (5 * 2);
    }

    loop{
        unsafe{
            let gpioa = &mut *GPIOA;
            // Toggle PA5
            gpioa.odr ^= 1 << 5;
            // Wait for 100ms
            wait_ms(10);
        }
    }
}



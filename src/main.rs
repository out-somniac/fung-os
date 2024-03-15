#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;

unsafe fn kernel_init() -> ! {
    panic!()
}

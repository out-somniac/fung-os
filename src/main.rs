#![no_main]
#![no_std]

mod bsp;
mod cpu;
mod panic_wait;

// Kernel entry point
unsafe fn kernel_init() -> ! {
    panic!()
}

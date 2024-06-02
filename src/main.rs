#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

// Kernel entry point
unsafe fn kernel_init() -> ! {
    print!("Hello fung-os!\n");
    panic!()
}

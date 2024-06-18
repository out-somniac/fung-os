#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod driver;
mod panic_wait;
mod print;
mod synchronization;

// Kernel entry point
unsafe fn kernel_init() -> ! {
    if let Err(x) = bsp::driver::init() {
        print!("Error initializing BSP driver subsystem: {}\n", x);
        panic!();
    }

    // Initialize all device drivers.
    driver::driver_manager().init_drivers();
    // println! is usable from here on.

    // Transition from unsafe to safe.
    kernel_main()
}

fn kernel_main() -> ! {
    use console::console;

    print!(
        "[0] {} version {}\n",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );
    print!("[1] Booting on: {}\n", bsp::board_name());

    print!("[2] Drivers loaded:\n");
    driver::driver_manager().enumerate();

    print!("[3] Chars written: {}\n", console().chars_written());
    print!("[4] Echoing input now\n");

    // Discard any spurious received characters before going into echo mode.
    console().clear_rx();
    loop {
        let c = console().read_char();
        console().write_char(c);
    }
}

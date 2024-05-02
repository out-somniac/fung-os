use crate::cpu;
use core::panic::PanicInfo;

// Default panic handler that must be defined for code to compile
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    cpu::wait_forever();
}

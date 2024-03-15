use core::arch::global_asm;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init()
}

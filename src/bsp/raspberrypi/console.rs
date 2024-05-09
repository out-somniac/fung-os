use core::fmt;

/// A mystical, magical device for generating QEMU output out of the void.
struct QEMUOutput;

impl core::fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }

        Ok(())
    }
}

pub fn console() -> impl core::fmt::Write {
    QEMUOutput {}
}

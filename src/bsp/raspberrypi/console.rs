use crate::console;
use crate::synchronization::{DummyLock, Mutex};
use core::fmt;

/// Struct containing the console's state. For now empty
struct State {}

impl fmt::Write for State {
    // Implementing `write_str` also implements `write_fmt`
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
            }
        }
        Ok(())
    }
}

/// A mystical, magical device for generating QEMU output out of the void.
struct QEMUOutput {
    state: DummyLock<State>,
}

impl QEMUOutput {
    const fn new() -> QEMUOutput {
        return QEMUOutput {
            state: DummyLock::new(State {}),
        };
    }
}

/// Singelton instance of the QEMUOutput
static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

impl console::Write for QEMUOutput {
    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        self.state.lock(|state| fmt::Write::write_fmt(state, args))
    }
}

pub fn console() -> &'static dyn console::Write {
    &QEMU_OUTPUT
}

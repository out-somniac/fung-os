//! BSP console facilities.
use crate::console;

/// Return a reference to the console.
pub fn console() -> &'static dyn console::interface::All {
    &super::driver::PL011_UART
}

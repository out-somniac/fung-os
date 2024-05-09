use crate::bsp;

pub fn console() -> impl core::fmt::Write {
    bsp::console::console()
}

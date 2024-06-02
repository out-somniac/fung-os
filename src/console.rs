use crate::bsp;
use core::fmt;

pub trait Write {
    fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;
}

pub fn console() -> &'static dyn Write {
    bsp::console::console()
}

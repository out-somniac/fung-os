#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod raspberrypi;

#[allow(unused_imports)]
#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use raspberrypi::*;

pub mod cpu;
pub mod driver;
pub mod memory;

/// Board identification.
pub fn board_name() -> &'static str {
    #[cfg(feature = "bsp_rpi3")]
    {
        return "Raspberry Pi 3";
    }

    #[cfg(feature = "bsp_rpi4")]
    {
        return "Raspberry Pi 4";
    }
}

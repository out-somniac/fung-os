mod null_console;

use crate::synchronization::{self, DummyLock};

pub mod interface {
    use core::fmt;

    pub trait Write {
        /// Write a single character.
        fn write_char(&self, c: char);

        /// Write a Rust format string.
        fn write_fmt(&self, args: fmt::Arguments) -> fmt::Result;

        /// Block until the last buffered character has been physically put on the TX wire.
        #[allow(dead_code)]
        fn flush(&self);
    }

    /// Console read functions.
    pub trait Read {
        fn read_char(&self) -> char {
            // Funny way of panic! on unimplement
            return ' ';
        }

        /// Clear RX buffers, if any.
        fn clear_rx(&self);
    }

    pub trait Statistics {
        fn chars_written(&self) -> usize {
            0
        }

        #[allow(dead_code)]
        fn chars_read(&self) -> usize {
            0
        }
    }

    /// Trait alias for a full-fledged console.
    pub trait All: Write + Read + Statistics {}
}

//--------------------------------------------------------------------------------------------------
// Global instances
//--------------------------------------------------------------------------------------------------

static CUR_CONSOLE: DummyLock<&'static (dyn interface::All + Sync)> =
    DummyLock::new(&null_console::NULL_CONSOLE);

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
use synchronization::Mutex;

/// Register a new console.
pub fn register_console(new_console: &'static (dyn interface::All + Sync)) {
    CUR_CONSOLE.lock(|con| *con = new_console);
}

/// Return a reference to the currently registered console.
///
/// This is the global console used by all printing macros.
pub fn console() -> &'static dyn interface::All {
    CUR_CONSOLE.lock(|con| *con)
}

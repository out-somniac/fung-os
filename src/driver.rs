use crate::{
    print,
    synchronization::{DummyLock, Mutex},
};

const NUM_DRIVERS: usize = 5;

struct DriverManagerInner {
    next_index: usize,
    descriptors: [Option<DeviceDriverDescriptor>; NUM_DRIVERS],
}

pub mod interface {
    pub trait DeviceDriver {
        fn compatible(&self) -> &'static str;

        /// Called by the kernel to bring up the device.
        /// # Safety
        /// - During init, drivers might do stuff with system-wide impact.
        unsafe fn init(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }
}

pub type DeviceDriverPostInitCallback = unsafe fn() -> Result<(), &'static str>;

/// A descriptor for device drivers.
#[derive(Copy, Clone)]
pub struct DeviceDriverDescriptor {
    device_driver: &'static (dyn interface::DeviceDriver + Sync),
    post_init_callback: Option<DeviceDriverPostInitCallback>,
}

/// Provides device driver management functions.
pub struct DriverManager {
    inner: DummyLock<DriverManagerInner>,
}

static DRIVER_MANAGER: DriverManager = DriverManager::new();

impl DriverManagerInner {
    pub const fn new() -> Self {
        Self {
            next_index: 0,
            descriptors: [None; NUM_DRIVERS],
        }
    }
}

impl DeviceDriverDescriptor {
    pub fn new(
        device_driver: &'static (dyn interface::DeviceDriver + Sync),
        post_init_callback: Option<DeviceDriverPostInitCallback>,
    ) -> Self {
        Self {
            device_driver,
            post_init_callback,
        }
    }
}

/// Return a reference to the global DriverManager.
pub fn driver_manager() -> &'static DriverManager {
    &DRIVER_MANAGER
}

impl DriverManager {
    /// Create an instance.
    pub const fn new() -> Self {
        Self {
            inner: DummyLock::new(DriverManagerInner::new()),
        }
    }

    /// Register a device driver with the kernel.
    pub fn register_driver(&self, descriptor: DeviceDriverDescriptor) {
        self.inner.lock(|inner| {
            inner.descriptors[inner.next_index] = Some(descriptor);
            inner.next_index += 1;
        })
    }

    /// Helper for iterating over registered drivers.
    fn for_each_descriptor<'a>(&'a self, f: impl FnMut(&'a DeviceDriverDescriptor)) {
        self.inner.lock(|inner| {
            inner
                .descriptors
                .iter()
                .filter_map(|x| x.as_ref())
                .for_each(f)
        })
    }

    /// Fully initialize all drivers.
    ///
    /// # Safety
    ///
    /// - During init, drivers might do stuff with system-wide impact.
    pub unsafe fn init_drivers(&self) {
        self.for_each_descriptor(|descriptor| {
            // 1. Initialize driver.
            if let Err(x) = descriptor.device_driver.init() {
                panic!(
                    "Error initializing driver: {}: {}",
                    descriptor.device_driver.compatible(),
                    x
                );
            }

            // 2. Call corresponding post init callback.
            if let Some(callback) = &descriptor.post_init_callback {
                if let Err(x) = callback() {
                    panic!(
                        "Error during driver post-init callback: {}: {}",
                        descriptor.device_driver.compatible(),
                        x
                    );
                }
            }
        });
    }

    /// Enumerate all registered device drivers.
    pub fn enumerate(&self) {
        let mut i: usize = 1;
        self.for_each_descriptor(|descriptor| {
            print!("      {}. {}\n", i, descriptor.device_driver.compatible());

            i += 1;
        });
    }
}

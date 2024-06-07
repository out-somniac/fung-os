use core::cell::UnsafeCell;

pub trait Mutex {
    type Data;

    /// The provided closure is wrapped in a critical section by the mutex
    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R;
}

/// Warning: This doesn't actually protect against data races!
pub struct DummyLock<T>
where
    T: ?Sized,
{
    data: UnsafeCell<T>,
}

// `DummyLock` needs to implement these marker traits
unsafe impl<T> Send for DummyLock<T> where T: ?Sized + Send {}
unsafe impl<T> Sync for DummyLock<T> where T: ?Sized + Send {}

impl<T> DummyLock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> Mutex for DummyLock<T> {
    type Data = T;

    fn lock<'a, R>(&'a self, f: impl FnOnce(&'a mut Self::Data) -> R) -> R {
        // Like described in the `DummyLock` warning: This does not prevent actual data races. It
        // only prevents creation of multiple &mut Data at the same time.
        let data = unsafe { &mut *self.data.get() };
        f(data)
    }
}

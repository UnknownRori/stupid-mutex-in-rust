use inner::Inner;
use locked_mutex::LockedMutex;

mod inner;
mod locked_mutex;
mod unlocked_mutex;

pub struct Mutex;

impl Mutex {
    pub fn new<T>(data: T) -> LockedMutex<T> {
        LockedMutex {
            inner: Inner::new(data),
        }
    }
}

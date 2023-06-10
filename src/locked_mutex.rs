use std::hint::spin_loop;

use crate::{inner::Inner, unlocked_mutex::UnlockedMutex};

#[derive(Debug)]
pub struct LockedMutex<T> {
    pub inner: Inner<T>,
}

unsafe impl<T> Send for LockedMutex<T> {}
unsafe impl<T> Sync for LockedMutex<T> {}

impl<T> LockedMutex<T> {
    pub fn acquire(&self) -> UnlockedMutex<T> {
        while !self.inner.try_unlock() {
            spin_loop();
        }

        UnlockedMutex::new(&self)
    }

    pub fn release(&self) {
        self.inner.lock();
    }
}

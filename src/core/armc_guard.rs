use std::ops::{DerefMut, Deref};

use super::Core;

pub struct ArmcGuard<'a, T: ?Sized> {
    mutex: &'a mut Core<T>,
}

impl<'a, T: ?Sized> ArmcGuard<'a, T> {
    pub(crate) fn new(mutex: &'a mut Core<T>) -> Self { Self { mutex } }
}

impl<T: ?Sized> Deref for ArmcGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.mutex.deref()
    }
}

impl<T : ?Sized> DerefMut for ArmcGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mutex.deref_mut()
    }
}

impl<T: ?Sized> Drop for ArmcGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.drop()
    }
}

unsafe impl<T: ?Sized> Send for ArmcGuard<'_, T> where T: Send {}
unsafe impl<T: ?Sized> Sync for ArmcGuard<'_, T> where T: Send + Sync {}
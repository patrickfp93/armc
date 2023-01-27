use std::ops::{Deref};

use super::Core;

pub struct ArmcRefGuard<'a, T: ?Sized> {
    refex: &'a Core<T>,
}

impl<'a, T: ?Sized> ArmcRefGuard<'a, T> {
    pub(crate) fn new(refex: &'a Core<T>) -> Self { Self { refex } }
}

impl<T: ?Sized> Deref for ArmcRefGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.refex.deref()
    }
}

impl<T: ?Sized> Drop for ArmcRefGuard<'_, T> {
    fn drop(&mut self) {
        self.refex.drop_ref()
    }
}

unsafe impl<T: ?Sized> Send for ArmcRefGuard<'_, T> where T: Send {}
unsafe impl<T: ?Sized> Sync for ArmcRefGuard<'_, T> where T: Send + Sync {}
use std::{
    hint::spin_loop,
    sync::{atomic::{AtomicBool, AtomicUsize, Ordering}}
};

pub(crate) struct Core<T> {
    count_ref: AtomicUsize,
    looked: AtomicBool,
    data: OptionCell<T>,
}

impl<T> Core<T> {
    pub fn new(data: T) -> Core<T> {
        Core {
            count_ref: AtomicUsize::default(),
            looked: AtomicBool::default(),
            data: OptionCell::new(data),
        }
    }
    pub fn lock(&self) -> ArmcGuard<'_, T> {
        while !self.looked.swap(true, Ordering::AcqRel)
            && self.count_ref.load(Ordering::Relaxed) > 0
        {
            spin_loop();
        }
        ArmcGuard { mutex: &self }
    }

    pub fn lock_ref(&self) -> ArmcRefGuard<'_, T> {
        while self.looked.load(Ordering::Relaxed) {
            spin_loop();
        }
        self.count_ref.fetch_add(1, Ordering::Relaxed);
        ArmcRefGuard { refex: &self }
    }

    fn drop(&self) {
        self.looked.store(false, Ordering::Release);
    }

    fn drop_ref(&self) {
        self.count_ref.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn unwrap(a: Self) -> T {
        OptionCell::unwrap(a.data)
    }
}

use std::ops::{Deref, DerefMut};

use crate::option_cell::OptionCell;

pub struct ArmcGuard<'a, T> {
    mutex: &'a Core<T>,
}

pub struct ArmcRefGuard<'a, T> {
    refex: &'a Core<T>,
}

impl<T> Deref for Core<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        while self.looked.load(Ordering::SeqCst) {
            spin_loop();
        }
        self.data.get_ref()
    }
}

impl<T> Deref for ArmcGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.mutex.data.get_ref()
    }
}

impl<T> DerefMut for ArmcGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.mutex.data.get_mut()
    }
}

impl<T> Drop for ArmcGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.drop()
    }
}

impl<T> Deref for ArmcRefGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.refex.data.get_ref()
    }
}

impl<T> Drop for ArmcRefGuard<'_, T> {
    fn drop(&mut self) {
        self.refex.drop_ref()
    }
}

unsafe impl<T> Send for ArmcGuard<'_, T> where T: Send {}
unsafe impl<T> Sync for ArmcGuard<'_, T> where T: Send + Sync {}

unsafe impl<T> Send for Core<T> where T: Send {}
unsafe impl<T> Sync for Core<T> where T: Send {}


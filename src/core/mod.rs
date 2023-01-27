use std::{
    hint::spin_loop,
    sync::{atomic::{AtomicBool, AtomicUsize, Ordering}}, cell::UnsafeCell
};

pub mod armc_ref_guard;
pub mod armc_guard;


use std::ops::{Deref, DerefMut};

use self::{armc_ref_guard::ArmcRefGuard, armc_guard::ArmcGuard};

pub(crate) struct Core<T: ?Sized> {
    count_ref: AtomicUsize,
    looked: AtomicBool,
    data: UnsafeCell<T>,
}

impl<T: ?Sized> Core<T> {
    
    pub fn lock(&self) -> ArmcGuard<'_, T> {
        while !self.looked.swap(true, Ordering::AcqRel)
            && self.count_ref.load(Ordering::Relaxed) > 0
        {
            spin_loop();
        }
        let ref_mut = unsafe {
            &mut *(( self as *const Self) as *mut Self)
        };
        ArmcGuard::new(ref_mut)
    }

    pub fn lock_ref(&self) -> ArmcRefGuard<'_, T> {
        while self.looked.load(Ordering::Relaxed) {
            spin_loop();
        }
        self.count_ref.fetch_add(1, Ordering::Relaxed);
        ArmcRefGuard::new(&self)
    }

    fn drop(&self) {
        self.looked.store(false, Ordering::Release);
    }

    fn drop_ref(&self) {
        self.count_ref.fetch_sub(1, Ordering::Relaxed);
    }

}

impl<T : Sized> Core<T>{
    pub fn new(data: T) -> Core<T> {
        Core {
            count_ref: AtomicUsize::default(),
            looked: AtomicBool::default(),
            data: UnsafeCell::new(data),
        }
    }
    pub fn unwrap(a: Self) -> T {
        a.data.into_inner()
    }
}


impl<T: ?Sized> Deref for Core<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        while self.looked.load(Ordering::SeqCst) {
            spin_loop();
        }
        unsafe{&*self.data.get()}
    }
}

impl<T: ?Sized> DerefMut for Core<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{&mut *self.data.get()}
    }
}


unsafe impl<T: ?Sized> Send for Core<T> where T: Send {}
unsafe impl<T: ?Sized> Sync for Core<T> where T: Send {}

/*use std::fmt::Debug;
impl <T : Debug> Debug for Core<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Core").field("data", &self.data).finish()
    }
}*/

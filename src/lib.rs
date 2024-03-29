mod tests;
pub mod core;
pub mod object;

use crate::core::Core;
use crate::core::armc_ref_guard::{ArmcRefGuard};
use crate::core::armc_guard::ArmcGuard;
use std::ops::Deref;
use std::sync::Arc;

use std::fmt::Debug;


pub struct Armc<T:?Sized> {
    ptr : usize,
    core: Arc<Core<T>>,
}

impl<T:?Sized> Armc<T> {
    
    /// Returns a token([ArmcGuard<'_,T>]) for data to be securely modified.
    ///
    /// # Examples
    ///
    /// ```
    /// use armc::Armc;
    ///
    /// let armc = Armc::new(5);
    /// assert_eq!(*armc.lock(),5);
    /// ```
    pub fn lock(&self) -> ArmcGuard<'_,T>{
        self.core.lock()
    }

    /// Returns a token([ArmcRefGuard<'_,T>]) to block any thread that tries to modify the data,
    /// guaranteeing its integrity for asynchronous reading.
    pub fn lock_ref(&self) -> ArmcRefGuard<'_,T>{
        self.core.lock_ref()
    }

}

impl<T> Armc<T>{
    
    pub fn new(data: T) -> Self {        
        let mut result = Self {
            ptr : 0,
            core: Arc::new(Core::new(data))
        };
        result.ptr = get_address(result.as_ref());
        result
    }

    /// Attempts to return data that is under the domain of [Armc<T>].
    ///
    /// # Errors
    /// This function returns to itself as error, it is because there is another instance of [Armc<T>] pointing to the same data.
    pub fn try_unwrap(a : Self) -> Result<T,Self>{
        let address = a.ptr;
        let result = Arc::try_unwrap(a.core);
        match result {
            Ok(core) => Ok(Core::unwrap(core)),
            Err(core) => Err(Armc{core,ptr: address}),
        }
    }
}

impl<T: ?Sized> AsRef<T> for Armc<T>{
    fn as_ref(&self) -> &T {
        self.core.as_ref()
    }
}

impl<T: ?Sized> Deref for Armc<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let reply = self.core.as_ref().deref();
        reply
    }
}

impl<T: ?Sized> Clone for Armc<T>{
    fn clone(&self) -> Self {
        Self { ptr: self.ptr, core: self.core.clone() }
    }
}

impl<T: ?Sized> PartialEq for Armc<T>{
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}

fn get_address<T>(data: &T) -> usize{
    (data as *const T) as usize
}

fn get_value_by_addrs<T>(addrs : usize) -> T{
    unsafe{
        let pointer = addrs as *const T;
        std::ptr::read(pointer) as T
    }
}

impl<T> From<Armc<T>> for Arc<T> {
    fn from(data: Armc<T>) -> Self {
        let addrs = get_address(data.as_ref());
        let data = get_value_by_addrs(addrs);        
        Arc::new(data)
    }
}

impl<T> From<Armc<T>> for Arc<std::sync::Mutex<T>>{
    fn from(data: Armc<T>) -> Self {        
        let data = get_value_by_addrs(get_address(data.as_ref()));
        Arc::new(std::sync::Mutex::new(data))
    }
}

impl <T : Debug> Debug for Armc<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = &*self.core.as_ref().lock_ref();
        f.debug_struct("Armc").field("value", value).finish()
    }
}



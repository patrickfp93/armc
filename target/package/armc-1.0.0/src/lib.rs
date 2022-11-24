mod tests;
mod core;
mod option_cell;

use crate::core::Core;
use crate::core::ArmcGuard;
use crate::core::ArmcRefGuard;
use std::ops::Deref;
use std::sync::Arc;
/*use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;


static COUNTER : AtomicUsize = AtomicUsize::new(1);
fn get_id() -> usize { COUNTER.fetch_add(1, Ordering::Relaxed) }
*/


pub struct Armc<T> {
    address : usize,
    core: Arc<Core<T>>,
}

impl<T> Armc<T> {
    pub fn new(data: T) -> Self {        
        let mut result = Self {
            address : 0,
            core: Arc::new(Core::new(data)),
        };
        result.address = get_address(result.as_ref());
        result
    }

    pub fn lock(&self) -> ArmcGuard<'_,T>{
        self.core.lock()
    }

    pub fn lock_ref(&self) -> ArmcRefGuard<'_,T>{
        self.core.lock_ref()
    }

    pub fn try_unwrap(a : Self) -> Result<T,Self>{
        let address = a.address;
        let result = Arc::try_unwrap(a.core);
        match result {
            Ok(core) => Ok(Core::unwrap(core)),
            Err(core) => Err(Armc{core,address}),
        }
    }

}

impl<T> AsRef<T> for Armc<T>{
    fn as_ref(&self) -> &T {
        self.core.as_ref()
    }
}

impl<T> Deref for Armc<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let reply = self.core.as_ref().deref();
        reply
    }
}

impl<T> Clone for Armc<T>{
    fn clone(&self) -> Self {
        Self { address: self.address, core: self.core.clone() }
    }
}

impl<T> PartialEq for Armc<T>{
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
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

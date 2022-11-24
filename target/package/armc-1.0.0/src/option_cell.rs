use std::{cell::UnsafeCell};

const ERROR01: &str = "Armc não aponta mais para nenhum objeto.";

pub struct OptionCell<T> {
    value: UnsafeCell<Option<T>>,
}

impl<'a,T> OptionCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(Some(value)),
        }
    }

    pub fn get_ref(&'a self) -> &'a T {
        unsafe {            
            let result = self.value.get();
            let result = result.as_ref().expect(ERROR01).as_ref().expect(ERROR01);
            result
         }
    }

    pub fn get_mut(&self) -> &mut T {
        unsafe {            
            let result = self.value.get();
            let result = result.as_mut().expect(ERROR01).as_mut().expect(ERROR01);
            result
         }
    }

    pub fn unwrap(cell: Self) -> T {
        let mut cell = cell;
        let o_t = std::mem::replace(cell.value.get_mut(), None);
        o_t.expect(ERROR01)
    }
}

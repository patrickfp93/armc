#[cfg(test)]

#[test]
fn it_works() {
    use crate::Armc;
    let result = Armc::new(5);
    *result.lock() -= 1;
    assert_eq!(*result.as_ref(), 4);
}

#[test]
fn it_works_async() {
    use std::thread;
    use crate::Armc;
    let numbers : Vec<usize> = (0..10).collect();
    let numbers = Armc::new(numbers);
    let numbers_clone = numbers.clone(); 

    let handle = thread::spawn(move ||{
        for (i,n) in numbers_clone.iter().enumerate() {
            numbers_clone.clone().lock()[i] += n * 2;
        }
    });
    for (i,n) in numbers.iter().enumerate() {
        numbers.clone().lock()[i] += n * 2;
    }
    handle.join().unwrap();
    println!("{:?}",numbers.as_ref());
}

#[test]
fn test_cast_to_arc() {
    use std::sync::{Arc, Mutex};
    use crate::Armc;
    let result = Armc::new(5);
    *result.lock() -= 1;
    let arc_r : Arc<i32>  = result.clone().into();
    assert_eq!(*result.as_ref(), 4);
    assert_eq!(*arc_r.as_ref(), 4);
    let arc_mut_r : Arc<Mutex<i32>> = result.into();
    assert_eq!(*arc_mut_r.lock().unwrap(), 4);
}

#[test]
fn test_cast_pointer_address_in_usize() {
    use std::ptr;
    let n = -50;
    let pointer = &n as *const i32;
    let addr = pointer as usize;
    let readed_n = unsafe {
        let p = addr as *const i32;
         ptr::read(p) as i32   
    };
    println!("oringinal number {} address:{:x?} and readed_n:{}",n,addr,readed_n);
}

#[test]
fn test_unwrap() {
    use std::sync::{Arc, Mutex};
    use crate::Armc;
    let result = Armc::new(5);
    *result.lock() -= 1;
    match Armc::try_unwrap(result) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

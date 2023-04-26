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
    use crate::Armc;
    use std::thread;
    let numbers: Vec<usize> = (0..10).collect();
    let numbers = Armc::new(numbers);
    let numbers_clone = numbers.clone();

    let handle = thread::spawn(move || {
        for (i, n) in numbers_clone.iter().enumerate() {
            numbers_clone.clone().lock()[i] += n * 2;
        }
    });
    for (i, n) in numbers.iter().enumerate() {
        numbers.clone().lock()[i] += n * 2;
    }
    handle.join().unwrap();
    println!("{:?}", numbers.as_ref());
}

#[test]
fn test_cast_to_arc() {
    use crate::Armc;
    use std::sync::{Arc, Mutex};
    let result = Armc::new(5);
    *result.lock() -= 1;
    let arc_r: Arc<i32> = result.clone().into();
    assert_eq!(*result.as_ref(), 4);
    assert_eq!(*arc_r.as_ref(), 4);
    let arc_mut_r: Arc<Mutex<i32>> = result.into();
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
    println!(
        "oringinal number {} address:{:x?} and readed_n:{}",
        n, addr, readed_n
    );
}

#[test]
fn test_unwrap() {
    use crate::Armc;
    let result = Armc::new(5);
    *result.lock() -= 1;
    println!("{:?}", result);
    match Armc::try_unwrap(result) {
        Ok(_) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_object() {
    mod ta {
        use crate::Armc;
        crate::object_mut_access!( pub(super) A{ value : usize});
    }

    let mut a = ta::A::new(4);
    
    a.value_mut(10);

    assert_eq!(*a.value(), 10);
}

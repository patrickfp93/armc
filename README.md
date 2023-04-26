# Armc
Armc is a Rust library that provides a wrapper for shared data, ensuring data integrity and thread-blocking during modifications and reads.

## Installation
You can add the Armc dependency to your Cargo.toml file:

### toml
 ```yaml
   [dependencies]
    armc = "1.4.4"
```
## Usage
To use the library, simply import it with the following code:
```rust
use armc::Armc;
```
### Features
Below are some of the library's features:

### Creating an Armc object
To create an Armc object, simply use the new method and pass the data you want to store:
```rust
let armc = Armc::new(5);
```

### lock_ref:
Accessing the data of an Armc object
You can access the stored data by blocking possible mutations. Multiple accesses can be done in parallel.
```rust
let data = armc.lock_ref();
println!("Data: {:?}", data);
```
### lock:
Modifying data of an Armc object
To modify the data of an Armc object, you need to use the lock method, which will block all mutation accesses:
```rust
let mut data = armc.lock();
*data = 10;
println!("Data: {:?}", data);
```
### Cloning an Armc object
You can clone an Armc object using the clone method:
```rust
let armc_clone = armc.clone();
println!("Data: {:?}", *armc_clone.lock_ref());
```
### The macro `object!` and its derivatives
One set of macros that might be useful for Rust programmers are the `object!`, `object_with_new!`, `object_ref_access!`, and `object_mut_access!` macros. These macros are designed to simplify the creation of structs with thread-safe access to their fields.

The `object!` macro creates a struct with fields wrapped in an ARMC (Atomic Reference-Counting Mutex) to allow thread-safe mutation. The macro retains the name of the macro for the name of the struct.

The `object_with_new!` macro is similar to the `object!` macro, but it also generates a constructor method named `new` that takes in the initial values for each field.

The `object_ref_access!` macro is similar to the `object_with_new!` macro, but it also generates getter methods for each field.

The `object_mut_access!` macro is similar to the `object_ref_access!` macro, but it also generates mutable setter methods for each field with the suffix `_mut`.
Attention! you need a crate `paste` dependency in your project for object_mut_access to work.
```
[dependencies]
paste = "1.0.12"
```

These macros are designed to save time and effort when creating structs with thread-safe access to their fields, and can be used in the following way:

```rust
#[macro_use]
extern crate my_crate;

object_mut_access!(MyStruct {
    foo: u32,
    bar: String,
});

let mut my_struct = MyStruct::new(42, "hello".to_string());

assert_eq!(*my_struct.foo(), 42);

my_struct.foo_mut(13);
assert_eq!(*my_struct.foo(), 13);
```

For more information on the implementation of these macros, see the documentation for each individual macro.
    
## Contribution
Contributions are welcome! Feel free to open an issue or submit a pull request.
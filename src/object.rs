/// This macro will create an ARMC wrapper in a struct defined in the macro's scope while retaining the macro's name.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate my_crate;
///
/// object!(pub MyStruct {
///     foo: u32,
///     bar: String,
/// });
///
/// let my_struct = MyStruct::new(42, "hello".to_string());
/// ```
#[macro_export]
macro_rules! object {
    ($vis:vis $name:ident { $($field:ident : $type:ty),* }) => {
        struct Base {
            $(pub $field: $type),*
        }

        #[derive(Clone,PartialEq)]
        $vis struct $name {
            base : Armc<Base>,
        }

    };
}

/// This macro will create an ARMC wrapper in a struct defined in the macro's scope while retaining the macro's name with a constructor.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate my_crate;
/// object_with_new!(MyStruct { foo: u32, bar: String, });
/// let my_struct = MyStruct::new(42, "hello".to_string());
///```

#[macro_export]
macro_rules! object_with_new {
    ($vis:vis $name:ident { $($field:ident : $type:ty),* }) => {
        struct Base {
            $(pub $field: $type),*
        }

        #[derive(Clone,PartialEq)]
        $vis struct $name {
            base : Armc<Base>,
        }
        /// A constructor that creates a new instance of the struct with the given field values.
        ///
        /// # Arguments
        ///
        /// * `$($field: $type),*` - A list of field values to initialize the struct with.
        ///
        /// # Example
        ///
        /// ```
        /// # #[macro_use] extern crate my_crate;
        /// let my_struct = MyStruct::new(42, "hello".to_string());
        /// ```
            impl $name {
                pub fn new($($field: $type),*) -> Self {
                    $name {
                        base: Armc::new(Base {
                            $($field: $field),*
                        })
                    }
                }
            }


    };
}

/// This macro expands to a struct with fields wrapped in an ARMC to allow thread-safe mutation and access to those fields via generated getters.
///
/// The generated struct will also have a constructor `new` that takes in the initial values for each field.
///
/// # Example
///
/// ```
/// #[macro_use]
/// extern crate my_crate;
///
/// object_ref_access!(MyStruct {
///     foo: u32,
///     bar: String,
/// });
///
/// let my_struct = MyStruct::new(42, "hello".to_string());
/// let foo_ref = my_struct.foo();
/// ```
#[macro_export]
macro_rules! object_ref_access {
    ($vis:vis $name:ident { $($field:ident : $type:ty),* }) => {
        struct Base {
            $(pub $field: $type),*
        }

        #[derive(Clone,PartialEq)]
        $vis struct $name {
            base : Armc<Base>,
        }
            impl $name {
                pub fn new($($field: $type),*) -> Self {
                    $name {
                        base: Armc::new(Base {
                            $($field: $field),*
                        })
                    }
                }
            $(
                // Generate getter
                pub fn $field(&self) -> &$type {
                    &self.base.$field
                }
            )*
        }


    };
}

/// This macro creates an ARMC wrapper in a struct defined in the macro's scope, along with a new constructor and getter methods for each field, and mutable setter methods for each field with the suffix `_mut`.
///
/// # Example
///
/// ```
///
/// object_mut_access!(MyStruct {
///     foo: u32,
///     bar: String,
/// });
///
/// let mut my_struct = MyStruct::new(42, "hello".to_string());
///
/// assert_eq!(*my_struct.foo(), 42);
///
/// my_struct.foo_mut(13);
/// assert_eq!(*my_struct.foo(), 13);
/// ```
#[macro_export]
macro_rules! object_mut_access {
    ($vis:vis $name:ident { $($field:ident : $type:ty),* }) => {
        struct Base {
            $(pub $field: $type),*
        }

        #[derive(Clone,PartialEq)]
        $vis struct $name {
            base : Armc<Base>,
        }
            impl $name {
                pub fn new($($field: $type),*) -> Self {
                    $name {
                        base: Armc::new(Base {
                            $($field: $field),*
                        })
                    }
                }
            $(
                // Generate getter
                pub fn $field(&self) -> &$type {
                    &self.base.$field
                }
            )*
        }
        use paste::paste;
        paste! {
        impl $name {
            $(
                // Generate mutable setter
                pub fn [<$field _mut>] (&mut self, value: $type) -> &mut Self {
                    self.base.lock().$field = value;
                    self
                }
            )*
        }
        }


    };
}

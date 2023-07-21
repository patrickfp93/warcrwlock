use std::sync::RwLockReadGuard;

#[test]
pub fn test_macro_struture_and_impl() {
    mod my_module {
        use warcrwlock::warc_rwlock;

        #[warc_rwlock]
        pub struct MyStruct {
            value: usize,
        }
        #[warc_rwlock]
        impl MyStruct {
            pub fn reset(&mut self) {
                self.value = 0;
            }
            pub fn new() -> Self {
                Self { value: 0 }
            }
            pub fn set_value(&mut self, value: usize) {
                self.value = value;
            }

            pub fn get_value(&self) -> usize {
                self.value
            }

            pub fn value_mut(&mut self) -> &mut usize {
                &mut self.value
            }

            pub fn value_ref(&self) -> &usize {
                &self.value
            }
        }
    }
    use my_module::MyStruct;
    let mut a = MyStruct::new();
    *a.write().unwrap().value_mut() = 10;
    assert_eq!(*a.read().unwrap().value_ref(), 10);
    let mut b = a.clone();
    b.set_value(11);    
    assert_eq!(a.get_value(), 11);
    a.reset();
    assert_eq!(b.get_value(), 0);

}

#[test]
pub fn test_macro_in_mod() {
    use warcrwlock::warc_rwlock;
    #[warc_rwlock]
    mod my_module {
        pub const name : &str= "my_module";

        pub struct MyStruct {
            value: usize,
        }
        impl MyStruct {
            pub fn reset(&mut self) {
                self.value = 0;
            }
            pub fn new() -> Self {
                Self { value: 0 }
            }
            pub fn set_value(&mut self, value: usize) {
                self.value = value;
            }

            pub fn get_value(&self) -> usize {
                self.value
            }

            pub fn value_mut(&mut self) -> &mut usize {
                &mut self.value
            }
            pub fn value_ref(&self) -> &usize {
                &self.value
            }
        }
    }
    use my_module::MyStruct;
    let mut a = MyStruct::new();
    *a.write().unwrap().value_mut() = 10;
    assert_eq!(*a.read().unwrap().value_ref(), 10);
    let mut b = a.clone();
    b.set_value(11);    
    assert_eq!(a.get_value(), 11);
    a.reset();
    assert_eq!(b.get_value(), 0);
}
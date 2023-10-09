//mod model_tests;



use crate::test_impl::private::MyStruct;
mod private {
    use std::sync::{Arc,RwLock};

    #[repr(C)] // Certifique-se de que a struct use o layout de memória C.
    pub struct MyStruct {
        field: Arc<RwLock<i32>>,
    }
    
    impl MyStruct {
        pub fn new(field: i32) -> Self { Self { field : Arc::new(RwLock::new(field)) } }
    }        
}
#[test]
pub fn test_get_value_field() {
    
    use private::MyStruct;
    let instance = MyStruct::new(42);

    let value = force_get_private_field_in_wrapper(instance);

    println!("Valor do campo: {}",value.read().unwrap());
}

use std::{mem::size_of, sync::{Arc,RwLock}};

fn force_get_private_field_in_wrapper(instance: MyStruct) -> Arc<RwLock<i32>>{
    type W = Arc<RwLock<i32>>;
    let ptr = &instance as *const _ as *const u8;
    let size = size_of::<MyStruct>();
    let bytes_struct: &[u8] = unsafe { std::slice::from_raw_parts(ptr, size) };
    let bytes_field: &[u8] = &bytes_struct[0..size_of::<W>()];
    let reply: W = unsafe { &*(bytes_field.as_ptr() as *const W) }.clone();
    reply
}






use std::sync::{Arc, RwLock};

mod fragment_test;
mod multiple_impls;



#[test]
pub fn test_asref_arc(){
    let number=  Arc::new(0);
    let prt_number = (number.as_ref() as *const i32) as usize;
    
    let prt_number_clone = (number.clone().as_ref() as *const i32) as usize;

    assert_eq!(prt_number,prt_number_clone)

}

#[test]
pub fn test_asref_arc_rwlock(){
    let number=  Arc::new(RwLock::new(0));
    let prt_number = (number.as_ref() as *const RwLock<i32>) as usize;
    *number.write().unwrap() = 65498;
    let prt_number_clone = (number.clone().as_ref() as *const RwLock<i32>) as usize;
    assert_eq!(prt_number,prt_number_clone)
}
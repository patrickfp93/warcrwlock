use std::ops::{Add, Mul, AddAssign, MulAssign};

use warcrwlock::warcrwlock;

pub trait GenericContract : Sized + PartialEq + PartialOrd + AddAssign + MulAssign {}
#[warcrwlock]
pub struct Generic<T : GenericContract>{
    value : T
}

#[warcrwlock]
impl<T: GenericContract> Generic<T>{
    pub fn new(value : T) -> Self{
        Self { value }
    }

    pub fn plus(&mut self, other : Self){
        self.value += other.value 
    }

}



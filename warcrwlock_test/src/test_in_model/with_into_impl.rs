use std::ops::Range;

use warcrwlock::warcrwlock;

#[warcrwlock]
pub struct Number{
    pub number : usize
}

#[warcrwlock]
impl Number {
    fn new(number: usize) -> Self { Number{number} }

    pub fn insert(&mut self,value : usize){
        self.number += value;
    }
    
    pub fn multiply(&mut self, other : &mut Self) -> &mut Self{
        self.number *= other.number;
        other.number *= self.number;
        self
    }

    fn full_method(&mut self, other : &mut Self) -> Self{
        let reply = Number::new(self.number + other.number);
        reply        
    }

    pub fn full_method_mut_ref(&mut self, other : &mut usize) -> Self{
        let reply = Number::new(self.number + *other);
        *other *= self.number;
        reply        
    }

    pub fn ref_value(&self) -> &usize{
        &self.number
    }

    pub fn mut_value(&mut self) -> &mut usize{
        &mut self.number
    }

    pub fn set_value(&mut self, value : usize) -> &mut Self{
        self.number = value;
        self
    }

    /*pub(super) fn generic_method<T: Into<String>>(&self,info : T){
        println!("{}: {}",info.into(),self.number);
    }*/
}

impl Number{
    pub fn push_one(&mut self){
        *self.number_mut() += 1;
    }
    pub fn sequencial_numbers(range: Range<usize>) -> Vec<Self>{
        let mut reply = vec![];
        for index in range{
            reply.push(Self::new(index))
        }
        reply
    }
}




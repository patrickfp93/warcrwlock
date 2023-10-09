use warcrwlock::warcrwlock;

#[warcrwlock]
pub struct MyStruct{
    value : usize,
    value_2 : usize
}

impl MyStruct {
    pub fn new(value: usize) -> Self { MyStruct::builder(value,value*2) }

    fn sum(&mut self,value : usize){
        *self.value_mut() += value;
    }
}
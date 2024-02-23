use warcrwlock::warcrwlock;

pub trait A{}

#[warcrwlock]
pub struct S<T : A>{
    a : T,
    b : Vec<f64>
}
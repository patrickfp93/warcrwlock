use warcrwlock::warcrwlock;


#[test]
pub fn impl_i_mut_self_o_default() {
    #[warcrwlock]
    pub struct A {
        value: usize,
    }
    #[warcrwlock]
    impl A {
        pub fn reset(&mut self) {
            self.value = 0;
        }
    }

}

#[test]
pub fn impl_mut_i_self_imput_o_default() {
    #[warcrwlock]
    pub struct A {
        value: usize,
    }
    #[warcrwlock]
    impl A {
        pub fn set(&mut self, value: usize) {
            self.value = value;
        }
    }
}

#[test]
pub fn impl_i_ref_self_output() {
    #[warcrwlock]
    pub struct A {
        value: usize,
    }    
    #[warcrwlock]
    impl A {
        pub fn get(&self) -> usize {
            self.value
        }
    }
}

#[test]
pub fn impl_static_o_self(){
    #[warcrwlock]
    pub struct A {
        value: usize,
    }
    #[warcrwlock]
    impl A {
        pub fn new() -> Self {
            Self{
                value: 0
            }
        }
    }
}

#[test]
pub fn impl_static_input_o_self(){
    #[warcrwlock]
    pub struct A {
        value: usize,
    }
    #[warcrwlock]
    impl A {
        pub fn new(value : usize) -> Self {
            Self{
                value
            }
        }

        pub fn value(&self) -> usize{
            self.value
        }

        pub fn static_method(current : A ,other : A) -> A{
            A::new(current.value() + other.value())
        }

        pub fn static_method_with_return() -> A{
            return A::new(0);
        }

    }
    
}

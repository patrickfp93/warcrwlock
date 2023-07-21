use warcrwlock::warc_rwlock;

#[test]
pub fn impl_i_mut_self_o_default() {
    #[warc_rwlock]
    pub struct A {
        value: usize,
    }
    #[warc_rwlock]
    impl A {
        pub fn reset(&mut self) {
            self.value = 0;
        }
    }

}

#[test]
pub fn impl_mut_i_self_imput_o_default() {
    #[warc_rwlock]
    pub struct A {
        value: usize,
    }
    #[warc_rwlock]
    impl A {
        pub fn set(&mut self, value: usize) {
            self.value = value;
        }
    }
}

#[test]
pub fn impl_i_ref_self_output() {
    #[warc_rwlock]
    pub struct A {
        value: usize,
    }    
    #[warc_rwlock]
    impl A {
        pub fn get(&self) -> usize {
            self.value
        }
    }
}

#[test]
pub fn impl_static_o_self(){
    #[warc_rwlock]
    pub struct A {
        value: usize,
    }
    #[warc_rwlock]
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
    #[warc_rwlock]
    pub struct A {
        value: usize,
    }
    #[warc_rwlock]
    impl A {
        pub fn new(value : usize) -> Self {
            Self{
                value
            }
        }
    }
    
}
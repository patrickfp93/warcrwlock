use std::{sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard}, ops::{DerefMut, Deref}};

//Essa implementação Ocorre com a macro na struct e na implementação
struct Base {
    value: usize,
}

impl Base {
    fn new(value: usize) -> Self {
        Self { value }
    }

    fn plus_one(&mut self) {
        self.value += 1;
    }

    fn plus(&mut self, other: Self) -> &mut Self {
        self.value += other.value;
        self
    }
    pub fn plus_static(a : Self, b : Self) -> Self {
        Self::new(a.value + b.value)
    }

    pub fn mut_value(&mut self) -> &mut usize{
        &mut self.value
    }

    pub fn ref_value(&self) -> &usize{
        &self.value
    }

    pub fn copy_value(&self) -> usize{
        self.value
    }

    //pub fn plus_static_manipulate(a : &Self, b : &Self) -> Self
    pub fn plus_static_manipulate(a : RwLockReadGuard<Self>, b : RwLockReadGuard<Self>) -> Self {
        Self::new(a.value + b.value)
    }

    //pub fn plus_manipulate(&smut self,other_value : &usize, b : &mut Base) -> Self
    pub fn plus_manipulate(&mut self,other_value : &usize, b : &mut RwLockWriteGuard<Self>){
        b.value += other_value;
        self.value += other_value;
    }

    pub fn get_vec(max : usize) -> Vec<Base>{
        let mut reply = vec![];
        for i in 0..max{
            reply.push(Self { value: i })
        }
        reply
    }

    pub fn multiple_plus(multiples : Vec<Self>) -> Self{
        let mut reply = Self::new(0);
        for item in multiples{
            reply.value += item.value;
        }
        reply
    }

    pub fn multiple_plus_slice(multiples : &[Self]) -> Self{
        let mut reply = Self::new(0);
        for item in multiples{
            reply.value += item.value;
        }
        reply
    }


}

#[derive(Clone)]
pub struct Wrapper {
    base: Arc<RwLock<Base>>
}

impl Wrapper {
    pub fn new(value: usize) -> Self {
        let reply = Base::new(value).into();
        return reply; 
    }

    pub fn plus_one(&mut self) {
        let mut guard = self.base.write().unwrap();
        let reply = guard.plus_one();
        return reply;
    }

    pub fn plus(&mut self, other: Self) -> &mut Self {
        let mut guard = self.base.write().unwrap();
        let other = other.into();
        guard.plus(other);
        drop(guard);
        return self;
    }

    pub fn plus_static(a : Self, b : Self) -> Self {
        let a = a.into();
        let b = b.into();
        let reply = Base::plus_static(a, b).into();
        return reply;
    }

    pub fn mut_value(&mut self) -> RefMutLockWrapper<usize>{
        let mut guard = self.base.write().unwrap();
        let value = guard.mut_value();
        let value = (value as *const usize) as *mut usize;
        return RefMutLockWrapper::new(value, guard);
    }

    pub fn ref_value(&mut self) -> RefLockWrapper<usize>{
        let guard = self.base.read().unwrap();
        let value = guard.ref_value();
        let value = value as *const usize;
        return RefLockWrapper::new(value, guard);
    }

    pub fn copy_value(&self) -> usize{
        let guard = self.base.read().unwrap();
        let reply = guard.copy_value();
        reply
    }

    pub fn plus_static_manipulate(a : Self, b : Self) -> Self {
        let a = a.base.read().unwrap();
        let b = b.base.read().unwrap();
        let reply = Base::plus_static_manipulate(a, b).into();
        return reply;
    }

    pub fn plus_manipulate(&mut self,other_value : &usize, b : Self){
        let mut guard = self.base.write().unwrap();
        let b = &mut b.base.write().unwrap();
        let reply = guard.plus_manipulate(other_value,b);
        return reply;
    }

    pub fn get_vec(max : usize) -> Vec<Wrapper>{        
        let reply = Self::into_vec_wrapper(Base::get_vec(max));
        return reply;
    }
    
    pub fn multiple_plus(multiples : Vec<Self>) -> Self{
        Base::multiple_plus(Self::into_vec_base(multiples)).into()
    }

    pub fn multiple_plus_slice(multiples : &[Self]) -> Self{
        let multiples = &Self::into_vec_base(multiples.to_vec());
        let reply = Base::multiple_plus_slice(multiples).into();
        return reply;
    }

    fn into_vec_wrapper(bases : Vec<Base>) -> Vec<Wrapper>{
        bases.into_iter().map(|b| b.into()).collect()
    }

    fn into_vec_base(multiples : Vec<Wrapper>) -> Vec<Base>{
        multiples.iter().map(|w| w.clone().into()).collect()
    }

}

impl From<Wrapper> for Base {
    fn from(wrapper: Wrapper) -> Self {
        let guard = wrapper.base.write().unwrap();
        let ptr = &*guard as *const Base;
        let reply = unsafe { std::ptr::read(ptr) };
        drop(guard);
        reply
    }
}


impl<'a> From<Base> for Wrapper{
    fn from(base: Base) -> Self {
        Self {
            base: Arc::new(RwLock::new(base))
        }
    }
}

impl PartialEq for Wrapper{
    fn eq(&self, other: &Self) -> bool {
        let ptr_number = self.base.as_ref() as *const RwLock<Base> as usize;
        let other_ptr_number = other.base.as_ref() as *const RwLock<Base> as usize;
        ptr_number == other_ptr_number
    }
}

unsafe impl Send for Wrapper{}

unsafe impl Sync for Wrapper{}

pub struct RefLockWrapper<'a,T>{
    _guard : RwLockReadGuard<'a,Base>,
    reference : &'a T,
}

impl<'a,T> RefLockWrapper<'a,T>{
    fn new( ptr : *const T, guard : RwLockReadGuard<'a,Base>)->Self{
        let reference = unsafe {&*ptr};
        Self {_guard : guard , reference }
    }
}

impl<'a, T> Deref for RefLockWrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.reference
    }
}

pub struct RefMutLockWrapper<'a,T,>{
    _guard : RwLockWriteGuard<'a,Base>,
    reference : &'a T,
    reference_mutable : &'a mut T,
}

impl<'a,T> RefMutLockWrapper<'a,T>{
    fn new( ptr : *mut T, guard : RwLockWriteGuard<'a,Base>)->Self{
        let reference = unsafe {&*ptr};
        let reference_mutable = unsafe {&mut *ptr};
        Self {_guard : guard , reference,reference_mutable }
    }
}

impl<'a, T> Deref for RefMutLockWrapper<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.reference
    }
}

impl<'a, T> DerefMut for RefMutLockWrapper<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.reference_mutable
    }
}

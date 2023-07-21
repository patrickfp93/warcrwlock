use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult};

struct Base{
    value: usize
}

impl Base{

    fn none_some_none(value : usize){
        println!("{}",value);
    }

    fn refself_none_none(&self){
        println!("{}",self.value);
    }
    fn refmutself_none_none(&mut self){
        self.value = 0;
    }
    fn refself_some_none(&self,value : usize){
        println!("{}",value);
    }
    fn refmutself_some_none(&mut self){
        self.value = 0;
    }

    fn refself_none_type(&self) -> usize{
        self.value        
    }
    
    fn none_some_type(value : usize) -> Self{
        Self { value }
    }

    pub fn refself_none_reftype(&self) -> &usize{
        &self.value
    }

    pub fn refmutself_some_refmuttype(&mut self) -> &mut usize{
        &mut self.value
    }

    fn none_none_type() -> Self{
        Self { value : 0 }
    }

    fn refmutself_none_remuttype(&mut self) -> &mut Self{
        self
    }

    fn refmutself_some_remuttype(&mut self, value : usize) -> &mut Self{
        self.value = value;
        self
    }

}

struct A{
    base : Arc<RwLock<Base>>
}

impl<'a> A{
    
    pub fn none_some_none(value : usize){
        Base::none_some_none(value);
    }
    //2
    pub fn refself_none_none(&self){
        self.base.read().unwrap().refself_none_none();
    }    
    //2
    pub fn refmutself_none_none(&mut self){
        self.base.write().unwrap().refself_none_none();
    }
    pub fn refself_some_none(&self,value : usize){
        self.base.read().unwrap().refself_some_none(value);
    }
    pub fn refmutself_some_none(&mut self){
        self.base.write().unwrap().refmutself_some_none();
    }

    pub fn refself_none_type(&self) -> usize{
        self.base.read().unwrap().refself_none_type()        
    }
    
    pub fn none_some_type(value : usize) -> Self{
        Self{
            base: Arc::new(RwLock::new(Base::none_some_type(value)))
        }
    }
    pub fn none_none_type() -> Self{        
        Self{
            base: Arc::new(RwLock::new(Base::none_none_type())),
        }
    }

    pub fn refmutself_none_remuttype(&mut self) -> &mut Self{
        self
    }

    pub fn refmutself_some_remuttype(&self, value : usize) -> &Self{
        self.base.write().unwrap().refmutself_some_remuttype(value);
        self
    }   

    ///função escrita obrigatória
    pub fn write(&mut self) -> LockResult<RwLockWriteGuard<'_, Base>>{
        self.base.write()
    }
    // função de leitura obrigatória
    pub fn read(&mut self) -> LockResult<RwLockReadGuard<'_, Base>>{
        self.base.read()
    }

}

impl Clone for A{
    fn clone(&self) -> Self {
        Self { base: self.base.clone() }
    }
}

unsafe impl Send for A{}
unsafe impl Sync for A{}

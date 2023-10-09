/*
Essa implementação é feita á partir da macro na struct.
*/

pub type Wrapper = wrapper::Wrapper;
pub type RefLockWrapper<'a, T> = wrapper::RefLockWrapper<'a, T>;
pub type RefMutLockWrapper<'a, T> = wrapper::RefMutLockWrapper<'a, T>;

mod wrapper {
    use std::{
        fmt::Debug,
        ops::{Deref, DerefMut},
        sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
    };

    #[doc(hidden)]
    pub(super) struct _Base {
        pub(super)value: usize,
        /*pub*/ pub(super)value_2: usize,
    }

    impl Debug for _Base {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Wrapper")
                .field("value", &self.value)
                .field("value_2", &self.value_2)
                .finish()
        }
    }

    pub struct Wrapper {
        base: Arc<RwLock<_Base>>,
    }

    //basic impls
    impl Wrapper {
        pub(super) fn builder(value: usize, value_2: usize) -> Self {
            _Base { value, value_2 }.into()
        }

        pub(super) fn value_mut(&mut self) -> RefMutLockWrapper<usize> {
            let mut guard = self.base.write().unwrap();
            let value = &mut guard.value;
            let value = (value as *const usize) as *mut usize;
            return RefMutLockWrapper::new(value, guard);
        }

        pub(super) fn value(&self) -> RefLockWrapper<usize> {
            let guard = self.base.read().unwrap();
            let value = &guard.value;
            let value = value as *const usize;
            return RefLockWrapper::new(value, guard);
        }

        pub fn value_2_mut(&mut self) -> RefMutLockWrapper<usize> {
            let mut guard = self.base.write().unwrap();
            let value = &mut guard.value_2;
            let value = (value as *const usize) as *mut usize;
            return RefMutLockWrapper::new(value, guard);
        }

        pub fn value_2(&self) -> RefLockWrapper<usize> {
            let guard = self.base.read().unwrap();
            let value = &guard.value_2;
            let value = value as *const usize;
            return RefLockWrapper::new(value, guard);
        }
    }

    impl From<Wrapper> for _Base {
        fn from(wrapper: Wrapper) -> Self {
            let guard = wrapper.base.write().unwrap();
            let ptr = &*guard as *const _Base;
            let reply = unsafe { std::ptr::read(ptr) };
            drop(guard);
            reply
        }
    }

    impl<'a> From<_Base> for Wrapper {
        fn from(base: _Base) -> Self {
            Self {
                base: Arc::new(RwLock::new(base)),
            }
        }
    }

    impl PartialEq for Wrapper {
        fn eq(&self, other: &Self) -> bool {
            let ptr_number = self.base.as_ref() as *const RwLock<_Base> as usize;
            let other_ptr_number = other.base.as_ref() as *const RwLock<_Base> as usize;
            ptr_number == other_ptr_number
        }
    }

    impl Debug for Wrapper {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            (*self.base.read().unwrap()).fmt(f)
        }
    }

    unsafe impl Send for Wrapper {}

    unsafe impl Sync for Wrapper {}

    pub struct RefLockWrapper<'a, T> {
        _guard: RwLockReadGuard<'a, _Base>,
        reference: &'a T,
    }

    impl<'a, T> RefLockWrapper<'a, T> {
        fn new(ptr: *const T, guard: RwLockReadGuard<'a, _Base>) -> Self {
            let reference = unsafe { &*ptr };
            Self {
                _guard: guard,
                reference,
            }
        }
    }

    impl<'a, T> Deref for RefLockWrapper<'a, T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            self.reference
        }
    }

    pub struct RefMutLockWrapper<'a, T> {
        _guard: RwLockWriteGuard<'a, _Base>,
        reference: &'a T,
        reference_mutable: &'a mut T,
    }

    impl<'a, T> RefMutLockWrapper<'a, T> {
        fn new(ptr: *mut T, guard: RwLockWriteGuard<'a, _Base>) -> Self {
            let reference = unsafe { &*ptr };
            let reference_mutable = unsafe { &mut *ptr };
            Self {
                _guard: guard,
                reference,
                reference_mutable,
            }
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

    impl Clone for Wrapper {
        fn clone(&self) -> Self {
            Self {
                base: self.base.clone(),
            }
        }
    }
}

pub mod into_wrapper {
    use super::wrapper::_Base;

    impl _Base{
        /*pub*/ fn plus(&mut self){
            self.value +=1;
        }
    }

}

impl Wrapper {
    pub fn new() -> Self {
        Wrapper::builder(0, 0)
    }
}


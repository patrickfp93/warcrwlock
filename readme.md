# WarcRwLock

[![Crates.io](https://img.shields.io/crates/v/warcrwlock.svg)](https://crates.io/crates/warcrwlock)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

The WarcRwLock crate is a Rust library that provides a macro attribute for mods, structs, and impls. The purpose of this library is to generate a wrapper that allows the struct to be used with the asynchronous reference control called Arc and the RwLock for asynchronous mutation control.

## Installation

To use the WarcRwLock crate, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
warcrwlock = "1.5.1"
```

## Example Usage

Here's a simple example of using WarcRwLock:

```rust

#[warcrwlock]
#[derive(Debug)]
pub struct MyStruct {
    value: usize,
}

#[warcrwlock]
impl MyStruct {
    pub fn new(value : usize) -> Self {
        Self {
            value,
        }
    }

    pub fn reset(&mut self) {
        self.value = 0;
    }

    pub fn value_mut(&mut self) -> &mut usize {
        &mut self.value
    }

    pub fn get_value(&self) -> usize {
        self.value
    }
    
    //wrapper Method
    pub fn plus(a : MyStruct, b : MyStruct) -> MyStruct{
        *a.write().unwrap().value_mut() += b.get_value();
        a
    }
    #[wrapper_method]
    pub fn get_wrapper_value(&self)-> usize{
        self.get_value()
    }

    pub fn child(&self) -> MyStruct{
        MyStruct::new(self.value + 10)
    }
    
}
```

After applying the `#[warcrwlock]` attribute, the code is transformed into:

```rust
pub struct MyStructBase {
    value: usize,
}

impl MyStructBase {
    pub fn new(value : usize) -> Self {
        Self {
            value,
        }
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    pub fn value_mut(&mut self) -> &mut usize {****
        &mut self.value
    }

    fn get_value(&self) -> usize {
        self.value
    }
    
    pub fn child(&self) -> MyStructBase{
        MyStructBase::new(self.value + 10)
    }

}

impl Debug for MyStructBase{
    //**** default impl
}


pub struct MyStruct {
    base: Arc<RwLock<MyStructBase>>,
}

impl MyStruct {
    pub fn new(value : usize) -> Self {
        Self {
            base: Arc::new(RwLock::new(MyStructBase::new(value))),
        }
    }

    pub fn reset(&mut self) {
        self.base.write().unwrap().reset();
    }

    pub fn get_value(&self) -> usize {
        self.base.read().unwrap().get_value()
    }
    
    pub fn plus(a : MyStruct, b : MyStruct) -> MyStruct{
        *a.write().unwrap().value_mut() = b.get_value();
        a
    }

    pub fn get_wrapper_value(&self)-> usize{
        self.get_value()
    }

    pub fn child(&self) -> MyStruct{
        MyStruct {
            base: Arc::new(RwLock::new(self.child())),
        }
    }    

}

impl MyStruct {
    pub fn read(&self) -> RwLockReadGuard<'_, MyStructBase> {
        self.base.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, MyStructBase> {
        self.base.write().unwrap()
    }
}

impl Clone for MyStruct {
    fn clone(&self) -> Self {
        Self {
            base: self.base.clone(),
        }
    }
}

impl PartialEq for MyStruct{
    fn eq(&self, other: &Self) -> bool {
        let ptr_usize_a = (self.base.as_ref() as *const RwLock<MyStructBase>) as usize;        
        let ptr_usize_b = (other.base.as_ref() as *const RwLock<MyStructBase>) as usize;
        ptr_usize_a == ptr_usize_b
    }
}
impl Debug for MyStruct{
    //**** default impl
}
unsafe impl Send for MyStruct {}
unsafe impl Sync for MyStruct {}
```

After using the `#[warcrwlock]` attribute, the `MyStruct` will be automatically rewritten with the addition of a `base` field containing an `Arc<RwLock<MyStructBase>>`. The functions of `MyStruct` will then be implemented to safely access the `base` field.

### Wrapper Methods
This type of method happens for two reasons: when the method has parameters of type ``Self`` or when the ``wrapper_attribute`` attribute is added.

### Result
The result is a wrapper that is laborious to implement with replicated functions of the original type so that it can safely access data without access concurrency.
```rust
fn main() {
    use my_module::MyStruct;
    let mut a = MyStruct::new();
    *a.write().unwrap().value_mut() = 10;
    assert_eq!(*a.read().unwrap().value_ref(), 10);
    let mut b = a.clone();
    b.set_value(11);    
    assert_eq!(a.get_value(), 11);
    a.reset();
    assert_eq!(b.get_value(), 0);
    assert!(a == b);   
}
```

### Modules
You can simplify the use of `#[warcrwlock]` by placing it as an attribute for the module, which will have the same effect as in the previous example:
```rust
use warcrwlock::warcrwlock;

#[warcrwlock]
mod my_module {
    /// other mods, structs, and/or impls...
}
```

> When used on a module, all structs, impls, and mods will be included, with exceptions.

## Historic
> * `Update(1.1.0)`: Methods with parameters of the same type as the presented structure can now be freely implemented.
> * `Update(1.2.0)`: an attribute called ``wrapper_method`` is added that goes into the wrapper "scope" of the framework.
> * `Update(1.2.1)`: Methods with conditional visibility or visibility other than public and private (like pub(crate)) are treated the same as public methods.
> * `Update(1.3.0)`: A new attribute, called ``visible_to_wrapper`` is added to private methods so that they are accessible to wrapper methods.
> * `Update(1.4.0)`: The wrapper now implements PartialEq.
> * `Update(1.4.1)`: Fixed compatibility with `derive` macros and other macros. Fixed attribute recognition failure.

## Contribution

The WarcRwLock project is mainly maintained by a single developer known as PFP but welcomes contributions from the community. However, it's essential that contributions stay within the scope of the project's main function.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
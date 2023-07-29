# WarcRwLock

[![Crates.io](https://img.shields.io/crates/v/warcrwlock.svg)](https://crates.io/crates/warcrwlock)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

The WarcRwLock crate is a Rust library that provides a macro attribute for mods, structs, and impls. The purpose of this library is to generate a wrapper that allows the struct to be used with the asynchronous reference control called Arc and the RwLock for asynchronous mutation control.

## Installation

To use the WarcRwLock crate, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
warcrwlock = "1.3.1"
```

## Example Usage

Here's a simple example of using WarcRwLock:

```rust
#[warcrwlock]
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

unsafe impl Send for MyStruct {}
unsafe impl Sync for MyStruct {}
```

After using the `#[warcrwlock]` attribute, the `MyStruct` will be automatically rewritten with the addition of a `base` field containing an `Arc<RwLock<MyStructBase>>`. The functions of `MyStruct` will then be implemented to safely access the `base` field.

### Wrapper Methods
This type of method happens for two reasons: when the method has parameters of type ``Self`` or when the ``wrapper_attribute`` attribute is added.

### Method read and write
Similar to the methods in `RwLock<T>`, these functions are used to lock the usage and gain access to read or write functions, as shown in the example below:
```rust
fn main() {
    use my_module::MyStruct;
    let mut a = MyStruct::new();
    *a.write().unwrap().value_mut() = 10;
    assert_eq!(a.read().unwrap().get_value(), 10);
    let a = MyStruct::plus(a, MyStruct::new(50));    
    assert_eq!(a.read().unwrap().get_value(), 60);    
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

> The use of the attribute may not work well with other attributes.
## Historic
> * `Update(1.1.0)`: Methods with parameters of the same type as the presented structure can now be freely implemented.
> * `Update(1.2.0)`: an attribute called ``wrapper_method`` is added that goes into the wrapper "scope" of the framework.
> * `Update(1.2.1)`: Methods with conditional visibility or visibility other than public and private (like pub(crate)) are treated the same as public methods.
> * `Update(1.3.0)`: A new attribute, called ``visible_to_wrapper`` is added to private methods so that they are accessible to wrapper methods.

## Contribution

The WarcRwLock project is mainly maintained by a single developer known as PFP but welcomes contributions from the community. However, it's essential that contributions stay within the scope of the project's main function.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
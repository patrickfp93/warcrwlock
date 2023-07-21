# WarcRwLock

[![Crates.io](https://img.shields.io/crates/v/warcrwlock.svg)](https://crates.io/crates/warcrwlock)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Description

The WarcRwLock crate is a Rust library that provides a macro attribute for mods, structs, and impls. The purpose of this library is to generate a wrapper that allows the struct to be used with the asynchronous reference control called Arc and the RwLock for asynchronous mutation control.

## Installation

To use the WarcRwLock crate, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
warcrwlock = "1.0.0"
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
    pub fn new() -> Self {
        Self {
            value: 0,
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
}
```

After applying the `#[warcrwlock]` attribute, the code is transformed into:

```rust
pub struct MyStructBase {
    value: usize,
}

impl MyStructBase {
    pub fn new() -> Self {
        Self {
            value: 0,
        }
    }

    fn reset(&mut self) {
        self.value = 0;
    }

    pub fn value_mut(&mut self) -> &mut usize {
        &mut self.value
    }

    fn get_value(&self) -> usize {
        self.value
    }
}

pub struct MyStruct {
    base: Arc<RwLock<MyStructBase>>,
}

impl MyStruct {
    pub fn new() -> Self {
        Self {
            base: Arc::new(RwLock::new(MyStructBase::new())),
        }
    }

    pub fn reset(&mut self) {
        self.base.write().unwrap().reset();
    }

    pub fn get_value(&self) -> usize {
        self.base.read().unwrap().get_value()
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

### Method read and write
Similar to the methods in `RwLock<T>`, these functions are used to lock the usage and gain access to read or write functions, as shown in the example below:
```rust
fn main() {
    use my_module::MyStruct;
    let mut a = MyStruct::new();
    *a.write().unwrap().value_mut() = 10;
    assert_eq!(a.read().unwrap().get_value(), 10);
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

## Contribution

The WarcRwLock project is mainly maintained by a single developer known as PFP but welcomes contributions from the community. However, it's essential that contributions stay within the scope of the project's main function.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
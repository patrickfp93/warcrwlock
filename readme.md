# WarcRwLock

[![Crates.io](https://img.shields.io/crates/v/warcrwlock.svg)](https://crates.io/crates/warcrwlock)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
## Installation

To use the WarcRwLock crate, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
warcrwlock = "1.6.4"
```
## Description
Warcrwlock is an ``abstraction`` that turns a simple data structure into an atomic self-reference, enabling simultaneous read and write control using ``Arc`` and ``RwLock``. The potential for software designed or partially implemented in Rust is significant in terms of security and performance, comparable to ``C++``. Therefore, the primary motivation is to increase productivity in these applications by abstracting the ability to share data ``asynchronously`` with ``safety``.

---

### Warcrwock Structural Attribute
Capable of transforming a simple `struct` into two structures. The "core" is the part that retains the original characteristics but is placed in a module called "core". The reason for this is to allow it to be used with derivatives without compromising the main structure. See the example below:
```rust
mod core{
    #[derive(Debug)]
    pub(super) struct MyStruct {
        pub(super) value: usize,
    }
}
```
Using the same identifier, another struct is created that will contain a single field, as shown in the example below:
~~~rust
struct MyStruct {
    core : Arc<RwLock<core::MyStruct>>,
}
//impls...
~~~

#### Accessors
To facilitate implementation, it is necessary to use accessors. Accessors are methods that are automatically generated for each field of the original structure. Using the `MyStruct` from the previous examples and adding a field, we would have 4 generated methods.
> `public_read_only` is an attribute that must be used on a field to make the mutable access method private.
>An important detail is that accessors are generated with the same visibility type as the field, as shown in the example below:
~~~rust
    #[warcrwlock]
    pub struct MyStruct {
        pub value: usize,
        #[public_read_only]
        pub value_2 : usize
        value_3 : usize
    }
~~~
>Result...

~~~rust
    impl MyStruct {
        pub fn value(&self) -> MyStructRefLock<usize>{...}
        pub fn value_mut(&mut self) -> MyStructMutLock<usize>{...}
        pub fn value_2(&self) -> MyStructRefLock<usize>{...}
        fn value_2_mut(&mut self) -> MyStructMutLock<usize>{...}
        fn value_3(&self) -> MyStructRefLock<usize>{...}
        fn value_3_mut(&mut self) -> MyStructMutLock<usize>{...}
    }
~~~

#### Constructor
To facilitate the construction of the wrapper, a private `builder` method is generated, which will receive all the values, allowing you the freedom to implement your own constructor methods as you wish.
> To illustrate, it will be demonstrated following the context of the previous examples:
~~~rust
    impl MyStruct {
        fn builder(value : usize, value : usize) -> Self{...}
    }
~~~

>**Limitations**
>* Field values are accessible only through methods.
>* The structure cannot have associated `lifetime`.

### Warcrwock Implementation Attribute
A user's implementation, writing and reading the structure's fields, is done through accessors and the constructor. The problem is that this can become tedious and less adaptable in some situations. For this reason, if the ``warcrwlock`` attribute is also placed in the implementations, it will be possible to program in the original context without the need to use ``accessors`` and ``constructors``.

>**Limitations**
>* The methods cannot have associated `lifetime`.
>* The methods in the implementation without the ``warcrwlock`` attribute are invisible.
>* It is not allowed to return types that have ``Self`` as a generic, except for ``Vec<Self>``.

## Historic
> * `1.1.0`: Methods with parameters of the same type as the presented structure can now be freely implemented.
> * `1.2.0`: an attribute called ``wrapper_method`` is added that goes into the wrapper "scope" of the framework.
> * `1.2.1`: Methods with conditional visibility or visibility other than public and private (like pub(crate)) are treated the same as public methods.
> * `1.3.0`: A new attribute, called ``visible_to_wrapper`` is added to private methods so that they are accessible to wrapper methods.
> * `1.4.0`: The wrapper now implements PartialEq.
> * `1.4.1`: Fixed compatibility with `derive` macros and other macros.
> * `1.4.2`: Fixed attribute and type recognition failure in the syntax tree..
> * `1.5.0`: Restructuring and bug fixes with removal of unnecessary attributes added from version `1.2.0`.
> * `1.5.2`: Generic support fixed.
> * `1.6.0`: Added "public_read_only" attribute.
> * `1.6.1`: Fixed access guard return type.
> * `1.6.2`: Fixed guards' access methods.
> * `1.6.3`: Fixed guard access methods regarding common parameters.
> * `1.6.4`: Replaced `static_init!` with `lazy_static` and   `Mutex` for global identifier, improving compatibility and safety.

## Contribution

The WarcRwLock project is mainly maintained by a single developer known as PFP but welcomes contributions from the community. However, it's essential that contributions stay within the scope of the project's main function.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
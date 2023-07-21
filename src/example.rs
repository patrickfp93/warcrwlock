//! # WarcRwLock
//!
//! The `warcrwlock` crate provides a convenient attribute `#[warcrwlock]` that can be used on mods, structs, and impls in Rust code.
//!
//! ## Description
//!
//! The purpose of this crate is to generate a wrapper around structs that allows them to be used with asynchronous reference control (`Arc`) and the `RwLock` for asynchronous mutation control. The generated code handles the synchronization and locking mechanisms, making it easier to work with concurrent data in multi-threaded applications.
//!
//! ## Usage
//!
//! To use the `#[warcrwlock]` attribute, simply add it to a mod, struct, or impl definition. The code inside the mod, struct, or impl will be automatically transformed to use `Arc` and `RwLock` for thread-safe access.
//!
//! ## Attribute
//!
//! ```rust
//! #[warcrwlock]
//! ```
//!
//! The `#[warcrwlock]` attribute is used to mark mods, structs, or impls that need to be transformed for concurrent access.
//!
//! ## Example
//!
//! ```rust
//! // This example will be included from the file "example.rs"
//! #[doc(include = "./example.rs")]
//! ```
//!
//! After applying the `#[warcrwlock]` attribute, the code is transformed into:
//!
//! ```rust
//! use std::sync::{Arc, RwLock};
//!
//! pub struct MyStructBase {
//!     value: usize,
//! }
//!
//! impl MyStructBase {
//!     pub fn new() -> Self {
//!         Self {
//!             value: 0,
//!         }
//!     }
//!
//!     pub fn increment(&mut self) {
//!         self.value += 1;
//!     }
//!
//!     pub fn get_value(&self) -> usize {
//!         self.value
//!     }
//! }
//!
//! pub struct MyStruct {
//!     base: Arc<RwLock<MyStructBase>>,
//! }
//!
//! impl MyStruct {
//!     pub fn new() -> Self {
//!         Self {
//!             base: Arc::new(RwLock::new(MyStructBase::new())),
//!         }
//!     }
//!
//!     pub fn increment(&mut self) {
//!         self.base.write().unwrap().increment();
//!     }
//!
//!     pub fn get_value(&self) -> usize {
//!         self.base.read().unwrap().get_value()
//!     }
//! }
//! ```
//!
//! The `MyStruct` struct is automatically rewritten with the addition of a `base` field containing an `Arc<RwLock<MyStructBase>>`, and the methods are implemented to safely access the `base` field.
//!
//! ## Contribution
//!
//! The WarcRwLock project is mainly maintained by a single developer known as PFP but welcomes contributions from the community. However, it's essential that contributions stay within the scope of the project's main function.
//!
//! ## License
//!
//! This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for more details.

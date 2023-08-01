use helpers::{module::extend_mod, struture::extend_struct};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Item};

mod helpers;

#[proc_macro_attribute]
pub fn warcrwlock(_: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    //obtem do o módulo
    let item: Item = parse_macro_input!(input as Item);
    match item {
        Item::Impl(item_impl) => helpers::implementation::extend_impl(item_impl).into(),
        Item::Mod(sub_mod) => extend_mod(sub_mod).into(),
        Item::Struct(item_struct) => extend_struct(item_struct).into(),
        _ => panic!("This macro can only be used in structs, impl Blocks and mods!"),
    }
}

#[proc_macro_attribute]
pub fn wrapper_method(_: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    input.into()
}

#[proc_macro_attribute]
pub fn visible_to_wrapper(_: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    input.into()
}

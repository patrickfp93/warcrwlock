use helpers::{implementation, struture};
use proc_macro::TokenStream;
use syn::{parse_macro_input, Item, Type};
use crate::helpers::traits;

mod helpers;

#[proc_macro_attribute]
pub fn warcrwlock(_: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(input);
    let reply = match item {
        /*Item::Impl(item_impl) => {
            let item_impl_clone = item_impl.clone();
            let self_ty = item_impl_clone.self_ty.as_ref();
            let original_type_name = if let Type::Path(type_path) = self_ty {
                type_path.path.segments.last().unwrap().ident.clone()
            } else {
                panic!("The implementation type is irregular!")
            };
            implementation::expantion(item_impl, &original_type_name.to_string()).into()
        },*/
        Item::Trait(item_trait) => traits::expansion(item_trait).into(),
        Item::Struct(item_struct) => struture::expansion(item_struct, None).into(),
        _ => panic!("This attribute can only be used in structs and implementations."),
    };
    reply
}


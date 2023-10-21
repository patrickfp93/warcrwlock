use helpers::{implementation, struture};
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Item, Type, FieldsNamed};

mod helpers;
#[cfg(test)]
mod tests;

#[proc_macro_attribute]
pub fn warcrwlock(_: TokenStream, input: TokenStream) -> TokenStream {
    let item: Item = parse_macro_input!(input);
    match item {
        Item::Impl(item_impl) => {
            let item_impl_clone = item_impl.clone();
            let self_ty = item_impl_clone.self_ty.as_ref();
            let original_type_name = if let Type::Path(type_path) = self_ty {
                type_path.path.segments.last().unwrap().ident.clone()
            } else {
                panic!("The implementation type is irregular!")
            };
            implementation::expantion(item_impl, &original_type_name.to_string()).into()
        }
        Item::Struct(item_struct) => struture::expantion(item_struct, None).into(),
        _ => panic!("This attribute can only be used in structs and implementations."),
    }
}

/*#[proc_macro_attribute]
pub fn warcrwlock_with_reader(
    possible_custom_name: TokenStream,
    input: TokenStream,
) -> TokenStream {
    let possible_custom_name: Option<syn::Ident> = parse_macro_input!(possible_custom_name);
    let item: Item = parse_macro_input!(input);
    match item {
        /*Item::Impl(item_impl) => {
            let item_impl_clone = item_impl.clone();
            let self_ty = item_impl_clone.self_ty.as_ref();
            let original_type_name = if let Type::Path(type_path) = self_ty {
                type_path.path.get_ident().unwrap()
            } else {
                panic!("The implementation type is irregular!")
            };
            implementation::expantion(item_impl, &original_type_name.to_string()).into()
        }*/
        Item::Struct(item_struct) => {
            let reader_ident: Ident = if let Some(custom_name) = possible_custom_name {
                custom_name
            } else {
                parse_str(&format!(
                    "{}{}",
                    item_struct.ident,
                    helpers::DEFAULT_REFERENCE_STRUTURE_NAME
                ))
                .unwrap()
            };
            struture::expantion(item_struct, Some(reader_ident)).into()
        }
        _ => panic!("This attribute can only be used in structs."),
    }
}*/



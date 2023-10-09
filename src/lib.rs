use helpers::{implementation, struture};
use proc_macro::TokenStream;
use syn::{Item, Type, parse_macro_input};

#[cfg(test)]
mod tests; 
mod helpers;

#[proc_macro_attribute]
pub fn warcrwlock(possible_prelude: TokenStream, input: TokenStream) -> TokenStream {
    //init_project_context();
    let item: Item = parse_macro_input!(input);
    match item {
        Item::Impl(item_impl) => {
            let item_impl_clone = item_impl.clone();
            let self_ty = item_impl_clone.self_ty.as_ref();
            let original_type_name = if let Type::Path(type_path) = self_ty{                
                type_path.path.get_ident().unwrap()
            }else{
                panic!("The implementation type is irregular!")
            };
            implementation::expantion(item_impl, &original_type_name.to_string(),possible_prelude.into()).into()
        },
        Item::Struct(item_struct) => {
            struture::expantion(item_struct,possible_prelude.into()).into()
        },
        _ => panic!("This attribute can only be used in structs and implementations."),
    }        
}


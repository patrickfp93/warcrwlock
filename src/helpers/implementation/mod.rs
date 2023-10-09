use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_quote, parse_str, ImplItem, ItemImpl};

use crate::helpers::contains_isolated_name;

//use self::wrapper_methods::generate_helper;

use super::{get_mod_ident, to_token_stream, full_base_struct_name};

mod base_methods;
mod wrapper_methods;

pub fn expantion(
    item_impl: ItemImpl,
    original_type_name: &str,
    prelude: TokenStream,
) -> TokenStream {
    let mut base_impl_item = item_impl.clone();
    base_impl_item.items.iter_mut().for_each(|item_impl| {
        if let ImplItem::Fn(method) = item_impl {
            base_methods::method_normalization(method, original_type_name);
        }
    });
    match base_impl_item.self_ty.as_mut() {
        syn::Type::Path(path) => {
            let last_segment = path.path.segments.last_mut().unwrap();
            let last_segment_str = last_segment.to_token_stream().to_string();
            if contains_isolated_name(&last_segment_str, original_type_name) {
                if last_segment_str == original_type_name {
                    last_segment.ident = parse_str(&full_base_struct_name(original_type_name)).unwrap();
                } else {
                    panic!("This implementation block is not pointing to the correct type!");
                }
            }
        }
        _ => panic!("The implementation type is incorrect! Only Path types should be allowed."),
    }
    let mut wrapper_impl_item = item_impl;
    let generics = &wrapper_impl_item
        .generics
        .params
        .to_token_stream()
        .to_string();
    wrapper_impl_item.items = wrapper_impl_item
        .items
        .iter()
        .filter_map(|impl_item| {
            if let ImplItem::Fn(method) = impl_item {
                let mut method = method.clone();                
                wrapper_methods::method_normalization(&mut method, original_type_name, &generics);
                return Some(ImplItem::Fn(method.clone()));
            }
            None
        })
        .collect();

    let base = to_token_stream(full_base_struct_name(original_type_name));
    let original_name = to_token_stream(original_type_name);
    let generics = wrapper_impl_item.generics.clone();
    wrapper_impl_item.self_ty = parse_quote!(#original_name #generics);
    base_impl_item.self_ty = parse_quote!(#original_name #generics);
    let mod_ident = get_mod_ident();
    let imports = quote! {        
        use super::*;
        use super:: #base as #original_name;
    };
    quote! {
        mod #mod_ident{
            #prelude
            #imports
            #base_impl_item
        }
        #wrapper_impl_item
    }
    .into()
}

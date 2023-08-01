use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Attribute, Item, Type, ItemMod, parse_str};

use super::{ATTRIBUTE_NAME, struture::extend_struct, implementation};

pub fn has_warcrwlock_attribute(item : &Item) -> bool{
    if let Item::Struct(struct_) = item{
        return struct_.attrs.iter().find(|&attr| {
            let name = get_name_attribute(attr).unwrap();
            name.contains(ATTRIBUTE_NAME)
        }).is_some()
    }else if let Item::Mod(mod_) = item{
        return mod_.attrs.iter().find(|&attr| {
            let name = get_name_attribute(attr).unwrap();
            name.contains(ATTRIBUTE_NAME)
        }).is_some()
    }
    else if let Item::Impl(impl_) = item{
        return impl_.attrs.iter().find(|&attr| {
            let name = get_name_attribute(attr).unwrap();
            name.contains(ATTRIBUTE_NAME)
        }).is_some()
    }
    false
}

fn get_name_attribute(attribute: &Attribute) -> Option<String> {
    if let Some(path) = attribute.path().get_ident() {
        let attribute_name = path.to_string();
        return Some(attribute_name);
    }
    None
}

pub fn get_type_name(type_ : Box<Type>) -> String{
    type_.into_token_stream().to_string()
}


pub fn extend_mod(item_mod: ItemMod) -> TokenStream {
    let mut tokens = format!(
        "{} mod {}",
        item_mod.vis.into_token_stream().to_string(),
        item_mod.ident.to_string()
    );
    tokens += "{";
    for item in item_mod.content.unwrap().1.iter() {
        if !has_warcrwlock_attribute(&item) {
            match item.clone() {
                Item::Impl(item_impl) => tokens += &implementation::extend_impl(item_impl.clone()).to_string(),
                Item::Mod(sub_mod) => tokens += &extend_mod(sub_mod.clone()).to_string(),
                Item::Struct(item_struct) => {
                    tokens += &extend_struct(item_struct.clone()).to_string()
                }
                _ => tokens += &item.into_token_stream().to_string(),
            }
        }
    }
    tokens += "}";
    let output: proc_macro2::TokenStream = parse_str(&tokens).unwrap();
    output.into()
}

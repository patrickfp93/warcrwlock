use quote::ToTokens;
use syn::{ItemStruct, parse_quote, parse_str, Fields, Ident, Type};

use crate::helpers::full_base_struct_name;

pub fn core_normalization(base: ItemStruct) -> ItemStruct {
    let mut base = base;
    base.vis = parse_quote!(pub);
    if let Fields::Named(field_named) = &mut base.fields{
        field_named.named.iter_mut().for_each(|field|{
            field.attrs.clear();
            field.vis = parse_quote!(pub(super));
        });   
    }
    base.attrs.push(parse_quote!(#[doc(hidden)]));
    let name_macro = crate::helpers::ATTRIBUTE_NAME;
    if base.generics.lifetimes().count() > 0 {
        panic!("The {name_macro} macro does not support \"lifetimes\"!");
    }
    base.clone()
}

pub fn wrapper_normalization(wrapper: ItemStruct) -> ItemStruct {
    let mut wrapper = wrapper;
    wrapper.attrs.clear();
    let bfn = crate::helpers::BASE_FIELD_NAME;
    let mut bsn = full_base_struct_name(wrapper.ident.clone());
    if wrapper.generics.params.len() > 0 {
        let generics = crate::helpers::filter_generics(&wrapper
            .generics)
            .params
            .clone()
            .into_token_stream()
            .to_string();
        bsn = format!("{bsn}<{generics}>");
    }
    let field_name: Ident = parse_str(&format!("{bfn}")).unwrap();
    let field_type: Type = parse_str(&format!("Arc<RwLock<{bsn}>>")).unwrap();
    let aux: ItemStruct = parse_quote!(
        pub struct Wrapper{
            #field_name : #field_type
        }
    );
    wrapper.attrs.push(parse_quote!(#[repr(C)]));
    wrapper.fields = aux.fields;
    wrapper.vis = aux.vis;
    wrapper.clone()
}

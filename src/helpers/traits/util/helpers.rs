use proc_macro2::Ident;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    ItemTrait
    , Visibility,
};

use crate::helpers::{camel_to_snake_case, to_token_stream};
use crate::helpers::traits::util::generator_method::GeneratorMethod;

pub fn get_ident_headers(
    item_trait: &ItemTrait,
) -> (&Ident, TokenStream, TokenStream, &Visibility) {
    let ident_trait = &item_trait.ident;
    let trait_visibility = &item_trait.vis;
    let ident_abs_module =
        to_token_stream(format!("{}_abstract", camel_to_snake_case(ident_trait)));
    let ident_abs_structure = to_token_stream(format!("{}Abstract", ident_trait));
    (
        ident_trait,
        ident_abs_module,
        ident_abs_structure,
        trait_visibility,
    )
}

pub fn access_methods(item_trait: &ItemTrait, ident_trait: &Ident) -> TokenStream {
    let mut methods_in_token_stream= vec![];
    for trait_item in &item_trait.items {
        if let Some(gm) = GeneratorMethod::possible_new(ident_trait,trait_item){
            methods_in_token_stream.push(gm.to_token_stream());
        }
    }
    quote!{ #(#methods_in_token_stream)* }
}
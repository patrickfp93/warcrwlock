use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemTrait;
use util::{has_mutation, ref_abstract_module};
use crate::helpers::traits::util::mut_abstract_module;

mod util;

pub fn expansion(item_trait : ItemTrait) -> TokenStream {
    let has_mutation = has_mutation(&item_trait);
    let abstract_module = if !has_mutation {ref_abstract_module(&item_trait)}
     else{mut_abstract_module(&item_trait)};
    let reply = quote!{
        #item_trait
        #abstract_module
    };
    reply
}



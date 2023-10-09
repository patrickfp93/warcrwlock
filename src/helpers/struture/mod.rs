use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use crate::helpers::{to_token_stream, full_base_struct_name};

mod util;

pub fn expantion(item_struct: ItemStruct,prelude : TokenStream) -> TokenStream {
    let core_struture = util::core_normalization(item_struct.clone());
    let wrapper_struture = util::wrapper_normalization(item_struct.clone());

    let imports = quote! {
        use std::{
            fmt::Debug,
            ops::{Deref, DerefMut},
            sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
        };        
    };

    let vis = item_struct.vis.clone();
    let helpers =
        util::generation_help_strutures_and_impls(&wrapper_struture);
    //panic!("helpes : \n{}",helpers.to_string());
    let fields = item_struct.fields.clone();
    let original_ident = item_struct.ident.clone();    
    let core_ident = to_token_stream(full_base_struct_name(original_ident.clone()));
    if let syn::Fields::Named(fields_named) = fields {
        let access_methods = util::generation_access_fields_for_wrapper(
            item_struct.ident.clone(),
            fields_named.clone(),
            item_struct.generics.clone(),
        );
        //panic!("debug_implemetations : {}",debug_implemetations.to_string());        
        let struct_is_root = crate::helpers::struct_is_root(&item_struct).unwrap();
        let use_acessor = if struct_is_root{
            quote!(use crate::*;)
        }else{
            quote!(use super::*;)
        };
        let reply = quote! {
            #vis type #core_ident = _core:: #original_ident;
                #imports
                mod _core{                    
                    #use_acessor
                    #prelude                    
                    #core_struture
                }
                #wrapper_struture
                #access_methods
                #helpers
        };
        return reply;
    }
    panic!("Structure fields must be named.")
}

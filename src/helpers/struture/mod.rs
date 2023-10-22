use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemStruct, Ident};

use crate::helpers::{to_token_stream, full_base_struct_name, filter_generics};

mod util;

pub fn expantion(item_struct: ItemStruct, possible_reader_ident : Option<Ident>) -> TokenStream {
    let core_struture = util::normalizations::core_normalization(item_struct.clone());
    let wrapper_struture = util::normalizations::wrapper_normalization(item_struct.clone());
    let possible_reader_struture = if let Some(reader_ident) = possible_reader_ident {
        Some(util::reader_normalization::wrapper_normalization(item_struct.clone(),Some(reader_ident)))
    }else{None};
    //let import_other_atributte = to_token_stream(&format!("warcrwlock::{}",crate::helpers::ONLY_READ));
    let imports = quote! {
        use std::{
            fmt::Debug,
            ops::{Deref, DerefMut},
            sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
        };              
    };
    
    let vis = item_struct.vis.clone();
    let helpers =
    util::generations::generation_help_strutures_and_impls(&wrapper_struture);
    //panic!("helpes : \n{}",helpers.to_string());
    let fields = item_struct.fields.clone();
    let original_ident = item_struct.ident.clone();    
    let core_ident = to_token_stream(full_base_struct_name(original_ident.clone()));
    if let syn::Fields::Named(fields_named) = fields {
        let access_methods = util::generations::generation_access_fields_for_wrapper(
            item_struct.ident.clone(),
            fields_named.clone(),
            item_struct.generics.clone(),
            false
        );
        let possible_reader_access_methods = if let Some(reader_struture) = possible_reader_struture.clone(){
            Some(util::generations::generation_access_fields_for_wrapper(reader_struture.ident, fields_named, reader_struture.generics, true))
        } else{
            None
        };
              
        let struct_is_root = crate::helpers::struct_is_root(&item_struct).unwrap();
        let use_acessor = if struct_is_root{
            quote!(use crate::*;)
        }else{
            quote!(use super::*;)
        };
        let generics = item_struct.generics.clone();
        let filted_generics = filter_generics(&generics);
        let reply = quote! {
            #vis type  #core_ident #generics = _core:: #original_ident #filted_generics;
                #imports
                mod _core{                 
                    #use_acessor             
                    #core_struture
                }
                #wrapper_struture
                #access_methods
                #possible_reader_struture
                #possible_reader_access_methods
                #helpers
        };
        return reply;
    }
    panic!("Structure fields must be named.")
}

/*pub fn reader_expantion(item_struct: ItemStruct) -> TokenStream {
    
}*/
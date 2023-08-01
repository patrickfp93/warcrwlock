use proc_macro2::{TokenStream, Ident};
use syn::{ItemStruct, DeriveInput, parse_quote, Data, DataStruct, Field, parse_str, ItemImpl, Visibility};

use quote::quote;

pub fn extend_struct(item_struct: ItemStruct) -> TokenStream {
    let input: DeriveInput = parse_quote! {
        #item_struct
    };
    let vis = &input.vis;
    let field_names = extract_fields(&input);
    // Gere um TokenStream contendo apenas os nomes dos campos impl Iterator<Item = TokenStream>
    let field_tokens = field_names.iter().map(|field_name| {
        let new_field_name = if let Visibility::Public(_) = field_name.vis {
            Field {
                attrs: field_name.attrs.clone(),
                vis: Visibility::Inherited,
                mutability: field_name.mutability.clone(),
                ident: field_name.ident.clone(),
                colon_token: field_name.colon_token,
                ty: field_name.ty.clone(),
            }
        } else {
            field_name.clone()
        };
        quote! { #new_field_name }
    });
    // Clone the struct name identifier and assign a new name
    let original_struct_name = input.ident.clone();
    let base_name = original_struct_name.to_string() + "Base";
    let base_name: Ident = parse_str(&base_name).unwrap();

    // Generate the transformed struct definition
    let transformed_struct = quote! {
        pub struct #base_name {
            // Fields remain intact
            #(#field_tokens),*
        }
    };
    // Generate the second struct definition
    let secund_struct = quote! {
        #vis struct #original_struct_name {
            // New field named 'base' of type 'Arc<Base>'
            base: std::sync::Arc<std::sync::RwLock<#base_name>>,
        }
    };

    let lock_impl: ItemImpl = parse_quote!(
        impl #original_struct_name{
            pub fn write(&mut self) -> LockResult<RwLockWriteGuard<'_, #base_name>> {
                self.base.write()
            }
            pub fn read(&self) -> LockResult<RwLockReadGuard<'_, #base_name>> {
                self.base.read()
            }
        }
    );

    let clone_impl: ItemImpl = parse_quote!(
        impl Clone for #original_struct_name{
            fn clone(&self) -> Self {
                Self { base: self.base.clone() }
            }
        }
    );

    let partial_eq_impl : ItemImpl = parse_quote!(
        impl PartialEq for #original_struct_name{
            fn eq(&self, other: &Self) -> bool {
                let ptr_usize_a = (self.base.as_ref() as *const RwLock<#base_name>) as usize;        
                let ptr_usize_b = (other.base.as_ref() as *const RwLock<#base_name>) as usize;
                ptr_usize_a == ptr_usize_b
            }
        }
    );

    let send_impl: ItemImpl = parse_quote!(
        unsafe impl Send for #original_struct_name{}
    );

    let sync_impl: ItemImpl = parse_quote!(
        unsafe impl Sync for #original_struct_name{}
    );

    // Combine the transformed struct and the original struct into the output tokens
    let output = quote! {
        use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard, LockResult};
        #transformed_struct
        #secund_struct
        #lock_impl
        #clone_impl
        #partial_eq_impl
        #send_impl
        #sync_impl
    };
    output.into()
}

pub fn extract_fields(input: &DeriveInput) -> Vec<Field> {
    if let Data::Struct(DataStruct { fields, .. }) = &input.data {
        let fields = fields
            .iter()
            .map(|field| field.clone())
            .collect::<Vec<Field>>();

        return fields;
    } else {
        // Retorne um TokenStream vazio se não for uma estrutura
        vec![]
    }
}
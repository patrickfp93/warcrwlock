use helpers::{
    attributes::{get_type_name, has_warcrwlock_attribute},
    fields::extract_fields,
    methods::{change_block_method, remove_pub_from_impl_item, transform_method_return_type},
};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, parse_str, DeriveInput, Field, FnArg, ImplItem, Item, ItemImpl,
    ItemMod, ItemStruct, Type, Visibility,
};

mod helpers;

#[doc = include_str!("./example.rs")]
#[proc_macro_attribute]
pub fn warcrwlock(_: TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    //obtem do o módulo
    let item: Item = parse_macro_input!(input as Item);
    match item {
        Item::Impl(item_impl) => extend_impl(item_impl).into(),
        Item::Mod(sub_mod) => extend_mod(sub_mod).into(),
        Item::Struct(item_struct) => extend_struct(item_struct).into(),
        _ => panic!("This macro can only be used in structs, impl Blocks and mods!"),
    }
}

fn extend_mod(item_mod: ItemMod) -> TokenStream {
    let mut tokens = format!(
        "{} mod {}",
        item_mod.vis.into_token_stream().to_string(),
        item_mod.ident.to_string()
    );
    tokens += "{";
    for item in item_mod.content.unwrap().1.iter() {
        if !has_warcrwlock_attribute(&item) {
            match item.clone() {
                Item::Impl(item_impl) => tokens += &extend_impl(item_impl.clone()).to_string(),
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

fn extend_struct(item_struct: ItemStruct) -> TokenStream {
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
        #send_impl
        #sync_impl
    };
    output.into()
}

fn is_self_type_arg(arg: &FnArg, original_struct_name: &str) -> bool {
    let mut possible_name_type = None;
    match arg {
        FnArg::Typed(pat_type) => {
            if let Type::Path(type_path) = &*pat_type.ty {
                if let Some(segment) = type_path.path.segments.last() {
                    possible_name_type = Some(segment.ident.to_string());
                }
            }
        }
        _ => {}
    };
    if let Some(name_type) = possible_name_type {
        if &name_type == original_struct_name {
            return true;
        }
    }
    return false;
}

fn has_self_type_args(impl_item: &ImplItem, original_struct_name: &str) -> bool {
    if let ImplItem::Fn(method) = impl_item {
        return method
            .sig
            .inputs
            .iter()
            .any(|arg| is_self_type_arg(arg, original_struct_name));
    }
    false
}

fn extend_impl(item_impl: ItemImpl) -> TokenStream {
    let original_name = item_impl.self_ty.clone();
    let original_name_str = get_type_name(original_name.clone()).expect("Could not get type name.");
    let base_name_str = original_name_str.clone() + "Base";
    let base_name: Ident = parse_str(&base_name_str).unwrap();

    // Obter todos os itens de implementação
    let mut original_impl_items = vec![];
    let mut secund_impl_items: Vec<ImplItem> = vec![];
    for impl_item in &item_impl.items {
        if has_self_type_args(&impl_item, &original_name_str) {
            secund_impl_items.push(impl_item.clone());
        } else {
            original_impl_items.push(impl_item.clone());
        }
    }

    for ii in original_impl_items.iter_mut() {
        transform_method_return_type(ii, original_name_str.clone());
    }

    let first_self_ty = parse_quote!(#base_name);
    let sencund_self_ty = parse_quote!(#original_name);

    let mut first_impl_items: Vec<ImplItem> = vec![];
    for original_impl_item in original_impl_items {
        if let Some(item) = change_block_method(&original_impl_item, base_name_str.clone()) {
            secund_impl_items.push(item);
            let mut modificated_impl_item = original_impl_item.clone();
            remove_pub_from_impl_item(&mut modificated_impl_item);
            first_impl_items.push(modificated_impl_item);
        } else {
            first_impl_items.push(original_impl_item.clone());
        }
    }

    let secund_impl_block = ItemImpl {
        attrs: item_impl.attrs.clone(),
        defaultness: item_impl.defaultness,
        unsafety: item_impl.unsafety,
        impl_token: item_impl.impl_token,
        generics: item_impl.generics.clone(),
        trait_: item_impl.trait_.clone(),
        self_ty: sencund_self_ty,
        brace_token: item_impl.brace_token,
        items: secund_impl_items,
    };
    // Crie um novo bloco impl com o nome modificado do tipo
    let renamed_impl_block = ItemImpl {
        attrs: item_impl.attrs,
        defaultness: item_impl.defaultness,
        unsafety: item_impl.unsafety,
        impl_token: item_impl.impl_token,
        generics: item_impl.generics,
        trait_: item_impl.trait_,
        self_ty: first_self_ty,
        brace_token: item_impl.brace_token,
        items: first_impl_items,
    };
    // Retorne o código modificado como TokenStream
    let output = quote! {
        #renamed_impl_block
        #secund_impl_block
    };
    output.into()
}

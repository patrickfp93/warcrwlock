use quote::{quote, ToTokens};
use syn::{
    parse_quote, punctuated::Punctuated, FieldsNamed, FnArg, Generics, Ident, ImplItem, ItemImpl,
    ItemStruct, Visibility,
};

use crate::helpers::{
    full_base_struct_name, get_mut_guard_name, get_ref_guard_name, to_token_stream, filter_generics,
};

pub fn generation_help_strutures_and_impls(wrapper: &ItemStruct) -> proc_macro2::TokenStream {
    let mut wsn = wrapper.ident.to_string();
    let mut bsn = full_base_struct_name(wsn.clone());
    let bfn = crate::helpers::BASE_FIELD_NAME.to_string();
    let mut generics = "".to_string();
    let mut filted_generics = "".to_string();
    let mut impl_generics = "".to_string();
    if wrapper.generics.params.len() > 0 {
        filted_generics = crate::helpers::filter_generics(&wrapper
            .generics)
            .params
            .clone()
            .into_token_stream()
            .to_string();
        generics = wrapper
            .generics
            .params
            .clone()
            .into_token_stream()
            .to_string();
        impl_generics = format!("<{generics}>");
        bsn = format!("{bsn}<{filted_generics}>");
        wsn = format!("{wsn}<{filted_generics}>");
    }
    let ref_guard_name = get_ref_guard_name(&wrapper.ident.to_string());
    let mut_guard_name = get_mut_guard_name(&wrapper.ident.to_string());
    //let vis_guards = wrapper.vis.clone();
    //to tokens Streams
    let wsn = to_token_stream(wsn);
    let bsn = to_token_stream(bsn);
    let bfn = to_token_stream(bfn);
    let generics = to_token_stream(generics);
    let full_ref_guard_name = to_token_stream(format!("{ref_guard_name}<'a,TField,{generics}>"));
    let filter_ref_guard_name = to_token_stream(format!("{ref_guard_name}<'a,TField,{filted_generics}>",));
    let full_mut_guard_name = to_token_stream(format!("{mut_guard_name}<'a,TField,{generics}>"));
    let filter_mut_guard_name = to_token_stream(format!("{mut_guard_name}<'a,TField,{filted_generics}>",));
    let impl_generics = to_token_stream(impl_generics);
    //(
    quote::quote! {

        impl #impl_generics #wsn{
            pub fn extract(&self) -> #bsn {
                self.clone().into()
            }
        }

        impl #impl_generics From<#wsn> for #bsn {
            fn from(wrapper: #wsn) -> Self {
                let guard = wrapper.#bfn.write().unwrap();
                let ptr = &*guard as *const #bsn;
                let reply = unsafe { std::ptr::read(ptr) };
                drop(guard);
                return reply;
            }
        }


        impl<'a,#generics> From<#bsn> for #wsn{
            fn from(#bfn: #bsn) -> Self {
                return Self {
                    #bfn: Arc::new(RwLock::new(#bfn))
                };
            }
        }

        impl #impl_generics PartialEq for #wsn{
            fn eq(&self, other: &Self) -> bool {
                let ptr_number = self.#bfn.as_ref() as *const RwLock<#bsn> as usize;
                let other_ptr_number = other.#bfn.as_ref() as *const RwLock<#bsn> as usize;
                return ptr_number == other_ptr_number;
            }
        }

        impl #impl_generics Clone for #wsn{
            fn clone(&self) -> Self {
                return Self { #bfn: self.#bfn.clone() };
            }
        }

        unsafe impl #impl_generics Send for #wsn{}

        unsafe impl #impl_generics Sync for #wsn{}

        /////////////////////////////////////////////
        impl #impl_generics #wsn{
            fn into_vec_wrapper(bases : Vec<#bsn>) -> Vec<#wsn>{
                bases.into_iter().map(|b| b.into()).collect()
            }

            fn into_vec_base(multiples : Vec<#wsn>) -> Vec<#bsn>{
                multiples.iter().map(|w| (*w).clone().into()).collect()
            }
        }

        ////////////////////////////////////////////
        pub struct #full_ref_guard_name{
            _guard : RwLockReadGuard<'a,#bsn>,
            reference : &'a TField,
        }

        impl<'a,TField,#generics> #filter_ref_guard_name{
            fn new( ptr : *const TField, guard : RwLockReadGuard<'a,#bsn>)->Self{
                let reference = unsafe {&*ptr};
                return Self {_guard : guard , reference };
            }
        }

        impl<'a, TField,#generics> Deref for #filter_ref_guard_name {
            type Target = TField;

            fn deref(&self) -> &Self::Target {
                return self.reference;
            }
        }

        pub struct #full_mut_guard_name{
            _guard : RwLockWriteGuard<'a,#bsn>,
            reference : &'a TField,
            reference_mutable : &'a mut TField,
        }

        impl<'a,TField,#generics>  #filter_mut_guard_name{
            fn new( ptr : *mut TField, guard : RwLockWriteGuard<'a,#bsn>)->Self{
                let reference = unsafe {&*ptr};
                let reference_mutable = unsafe {&mut *ptr};
                return Self {_guard : guard , reference,reference_mutable };
            }
        }

        impl<'a, TField,#generics> Deref for  #filter_mut_guard_name {
            type Target = TField;

            fn deref(&self) -> &Self::Target {
                return self.reference;
            }
        }

        impl<'a, TField,#generics> DerefMut for  #filter_mut_guard_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                return self.reference_mutable;
            }
        }

    } /*,
          full_mut_guard_name,
          full_ref_guard_name,
          wsn,
      )*/
}

pub fn generation_access_fields_for_wrapper(
    original_ident_struct: Ident,
    fields_named: FieldsNamed,
    generics: Generics,
    is_reader: bool,
) -> ItemImpl {
    let filted_generics = filter_generics(&generics);
    let mut impl_items = vec![];
    let mut_guard_name = to_token_stream(get_mut_guard_name(&original_ident_struct.to_string()));
    let ref_guard_name = to_token_stream(get_ref_guard_name(&original_ident_struct.to_string()));
    //let minimal_default_vis = Visibility::Inherited;
    let bsn = to_token_stream(full_base_struct_name(original_ident_struct.clone()));
    let mut params = Punctuated::<FnArg, syn::Token![,]>::new();
    let mut instance_fields = Punctuated::<syn::FieldValue, syn::Token![,]>::new();
    let bfn = to_token_stream(crate::helpers::BASE_FIELD_NAME);
    for field in fields_named.named.iter() {
        let vis = field.vis.clone();
        let mut mut_vis = field.vis.clone();
        let ident = field.ident.clone().unwrap();
        let mut_ident_method = to_token_stream(format!("{ident}_mut"));
        let ref_ident_method = to_token_stream(format!("{ident}"));
        if field.attrs.iter().find(|attr| attr.path().is_ident(crate::helpers::ONLY_READ)).is_some(){
            mut_vis = Visibility::Inherited;
        }
        let ty = field.ty.clone();
        let mut guard_generics = filted_generics.clone();
        guard_generics.params.insert(0, parse_quote!(#ty));
        if !is_reader {
            let impl_item_mut: ImplItem = parse_quote! {
                #mut_vis fn #mut_ident_method(&mut self) -> #mut_guard_name #guard_generics{
                    let mut guard = self.#bfn.write().unwrap();
                    let value = &mut guard. #ident;
                    let value = (value as *const #ty) as *mut #ty;
                    return #mut_guard_name::new(value, guard);
                }
            };
            impl_items.push(impl_item_mut);
        }
        let impl_item_ref: ImplItem = parse_quote! {
            #vis fn #ref_ident_method(&self) -> #ref_guard_name #guard_generics{
                let guard = self.#bfn.read().unwrap();
                let value = &guard. #ident;
                let value = value as *const #ty;
                return #ref_guard_name::new(value, guard);
            }
        };
        impl_items.push(impl_item_ref);
        params.push(parse_quote!(#ident: #ty));
        instance_fields.push(parse_quote!(#ident));
    }
    //make builder
    let builder_generics = if generics.params.len() > 0{
        Some(quote!(:: #filted_generics))
    }else {None};
    let impl_item_builder: ImplItem = parse_quote! {
        fn builder(#params) -> Self{
            return #bsn #builder_generics{#instance_fields}.into();
        }
    };
    impl_items.push(impl_item_builder);

    let wsn = to_token_stream(format!(
        "{}{}",
        original_ident_struct,
        filted_generics.to_token_stream().to_string()
    ));
    let item_impl: ItemImpl = parse_quote! {
        impl #generics #wsn{
            #(#impl_items)*
        }
    };
    //panic!("item_impl: \n {}",item_impl.to_token_stream().to_string());

    item_impl
}

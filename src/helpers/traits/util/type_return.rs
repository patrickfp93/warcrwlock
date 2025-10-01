use std::ops::Deref;
use proc_macro2::Ident;
use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::{ReturnType, TraitItemFn, Type};
use crate::helpers::to_token_stream;
use crate::helpers::traits::util::{MUTEX_GUARD_SUFFIX, REF_GUARD_SUFFIX};

pub struct TypeInMethod {
    ty_ts: TokenStream,
    possible_reference_and_mut: Option<bool>,
}

impl TypeInMethod {
    pub fn get_normalized_return(&self, ident_trait: &Ident) -> TokenStream {
        return match self.possible_reference_and_mut {
            None => {
                self.ty_ts.clone()
            }
            Some(is_mutable) => {
                let inner_type = self.ty_ts.clone();
                return if is_mutable {
                    let ident_mut_guard = to_token_stream(format!("{}MutGuard", ident_trait));
                    quote! {#ident_mut_guard<#inner_type>}
                } else {
                    let ident_ref_guard = to_token_stream(format!("{}RefGuard", ident_trait));
                    quote! {#ident_ref_guard<#inner_type>}
                };
            }
        };
    }

    pub fn is_only_ref(&self) -> bool{
        return if let Some(mutable) = self.possible_reference_and_mut{
            !mutable
        }else{
            false
        }

    }

    pub fn is_ref_mut(&self) -> bool{
        return if let Some(mutable) = self.possible_reference_and_mut{
            mutable
        }else{
            false
        }
    }

    pub fn get_ident_flag(id: u8) -> TokenStream {
        to_token_stream(format!("flag_{}", id))
    }

    pub fn get_flag_line(&self, id: u8) -> Option<TokenStream> {
        return if self.possible_reference_and_mut.is_none() {
            None
        } else if !self.possible_reference_and_mut.unwrap() {
            let ident_flag = Self::get_ident_flag(id);
            Some(quote! {let #ident_flag = self.flag.read().unwrap();})
        } else {
            let ident_flag = Self::get_ident_flag(id);
            Some(quote! {let #ident_flag = self.flag.write().unwrap();})
        };
    }

    pub fn get_variable_for_reply(&self, ident_variable: String, ident_trait: &Ident, flag_ident: TokenStream) -> TokenStream {
        let ident_variable = to_token_stream(ident_variable);
        let ident_type = self.ty_ts.clone();
        let flag_ident = to_token_stream(flag_ident);
        return if self.possible_reference_and_mut.is_none() {
            quote! {#ident_variable}
        } else if self.possible_reference_and_mut.unwrap() {
            let ident_mut_guard = to_token_stream(format!("{}{}", ident_trait, MUTEX_GUARD_SUFFIX));
            quote! {#ident_mut_guard::new(#ident_variable as *mut #ident_type, #flag_ident)}
        } else {
            let ident_ref_guard = to_token_stream(format!("{}{}", ident_trait, REF_GUARD_SUFFIX));
            quote! {#ident_ref_guard::new(#ident_variable as *const #ident_type, #flag_ident)}
        };
    }
    pub fn extract_type_in_methods(trait_item_fn: &TraitItemFn) -> Vec<Self> {
        let mut types_in_method = Vec::new();
        if let ReturnType::Type(_,ty) = &trait_item_fn.sig.output {
            if let Type::Tuple(type_tuple) = ty.deref() {
                for ty in type_tuple.elems.iter() {
                    match ty.try_into() {
                        Ok(tim) => types_in_method.push(tim),
                        Err(err) => { panic!("{}", err) }
                    }
                }
            } else {
                match ty.deref().try_into() {
                    Ok(tim) => types_in_method.push(tim),
                    Err(err) => { panic!("{}", err) }
                }
            }
        }
        types_in_method
    }
}

const SCOPE_ERROR: &'static str = "Only three types of type can be accepted: path and reference.";

impl<'a> TryFrom<&'a Type> for TypeInMethod {
    type Error = &'static str;

    fn try_from(value: &'a Type) -> Result<Self, Self::Error> {
        match value {
            Type::Path(path) => {
                let ty = path.to_token_stream();
                Ok(Self {
                    ty_ts: ty,
                    possible_reference_and_mut: None,
                })
            }
            Type::Reference(reference) => {
                let ty_ts = reference.elem.to_token_stream();
                let possible_reference_and_mut = Some(reference.mutability.is_some());
                Ok(Self {
                    ty_ts,
                    possible_reference_and_mut,
                })
            }
            _ => { Err(SCOPE_ERROR) }
        }
    }
}

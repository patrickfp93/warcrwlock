use std::ops::Deref;

use proc_macro2::{
    Ident,
    TokenStream
};
use quote::quote;
use quote::ToTokens;
use syn::{FnArg, PatIdent, PatType, TraitItem, TraitItemFn};

use crate::helpers::to_token_stream;
use crate::helpers::traits::util::type_return::TypeInMethod;

pub struct GeneratorMethod<'a> {
    ident_trait: &'a Ident,
    trait_item_fn: &'a TraitItemFn,
    types_in_method: Vec<TypeInMethod>,
}

impl<'a> GeneratorMethod<'a> {

    const MULTI_MUTABLE_REFS: &'static str = "A return tuple cannot have more than one mutable reference.";
    const OCCURRENCE_REF_AND_MUT: &'static str = "A return tuple cannot have both a mutable reference and an immutable reference.";

    pub fn possible_new(ident_trait: &'a Ident, item_trait: &'a TraitItem) -> Option<Self> {
        if let TraitItem::Fn(trait_item_fn) = item_trait {
            let types_in_method = TypeInMethod::extract_type_in_methods(trait_item_fn);
            return Some(Self {
                ident_trait,
                trait_item_fn,
                types_in_method,
            });
        };
        None
    }

    fn transform_fn_args(&self) -> TokenStream {
        let args = self.trait_item_fn.sig.inputs.iter().filter_map(|arg| {
            if let FnArg::Typed(PatType { pat, .. }) = arg {
                if let syn::Pat::Ident(PatIdent { ident, .. }) = &**pat {
                    return Some(ident);
                }
            }
            None
        });

        quote! { (#(#args),*) }
    }

    fn is_self_mutable(trait_item_fn: &TraitItemFn) -> bool {
        let mut reply = false;
        for input in &trait_item_fn.sig.inputs {
            if let FnArg::Receiver(receiver) = input
            {
                reply = receiver.mutability.is_some();
                break;
            }
        }
        reply
    }

    fn return_type_normalizer(&self) -> TokenStream {
        let normalized_types: Vec<TokenStream> = self.types_in_method.deref()
            .iter().map(|a| a.get_normalized_return(self.ident_trait)).collect();
        //check references
        return if normalized_types.len() > 1 {
            let mut mut_refs = 0;
            let mut refs = 0;
            for tim in self.types_in_method.iter() {
                if mut_refs > 1{ self.emitter_panic(Self::MULTI_MUTABLE_REFS); }
                else if refs > 0 && mut_refs == 1 { self.emitter_panic(Self::OCCURRENCE_REF_AND_MUT);}
                if tim.is_ref_mut(){ mut_refs += 1;}
                else if tim.is_only_ref() { refs += 1;}
            }
            quote! { (#(#normalized_types),*) }
        } else if normalized_types.len() == 1 {
            let normalized_type = normalized_types[0].clone();
            quote! { #normalized_type }
        } else {
            quote! { () }
        };
    }

    fn emitter_panic(&self, message : &str) {
        panic!("Error(on {}):{}", self.trait_item_fn.sig.to_token_stream().to_string(), message)
    }

    fn extract_fn_signature(&self) -> TokenStream {
        let normalized_return = self.return_type_normalizer();
        let sig = &self.trait_item_fn.sig;
        let fn_token = &sig.fn_token;
        let ident = &sig.ident;
        let generics = &sig.generics;
        let inputs = &sig.inputs;

        quote! {pub #fn_token #ident #generics(#inputs) -> #normalized_return}
    }

    fn access_base_line(&self) -> TokenStream {
        let args = self.transform_fn_args();
        let base_ident = "(*self.base)";
        let start_unsafe_block = "unsafe{";
        let end_unsafe_block = "};";
        to_token_stream(format!("\n let reply = {start_unsafe_block} (*self.base).{}{} {end_unsafe_block}",self.trait_item_fn.sig.ident, args.to_string()))
    }

    fn get_flag_line(&self) -> TokenStream {
        let mut flag_lines = vec!();
        for (i,tim) in self.types_in_method.iter().enumerate() {
            if let Some(flag_line) = tim.get_flag_line(i as  u8){
                flag_lines.push(flag_line);
            }
        };
        if flag_lines.len() == 0 {
            return if Self::is_self_mutable(self.trait_item_fn) {
                quote! {let _flag = self.flag.write().unwrap();}
            } else {
                quote!(let _flag = self.flag.read().unwrap();)
            };
        }
        quote! {#(#flag_lines) *}
    }

    fn get_reply_line(&self) -> TokenStream {
        return if self.types_in_method.len() > 1 {
            let vrs: Vec<TokenStream> =
                self.types_in_method.iter().enumerate()
                    .map(|(i, tim)|
                    tim.get_variable_for_reply(format!("reply.{}", i), self.ident_trait,
                                               TypeInMethod::get_ident_flag(i as u8))).collect();
            quote! { return (#(#vrs),*);}
        } else if self.types_in_method.len() == 1{
            let reply = if self.types_in_method[0].is_ref_mut(){
                self.types_in_method[0].get_variable_for_reply("reply".into(), self.ident_trait,quote!{flag_0} )
            }else{
                self.types_in_method[0].get_variable_for_reply("reply".into(), self.ident_trait,quote!{flag_0} )
            };
            quote! {return #reply;}
        }else{
            quote!{ return reply;}
        };
    }
}

impl<'a> ToTokens for GeneratorMethod<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let sig = self.extract_fn_signature();
        let flag_lines = self.get_flag_line();
        let access_line = self.access_base_line();
        let reply_line = self.get_reply_line();
        *tokens = quote! {
                /////////////////////////////////////////
                #sig {
                    #flag_lines
                    #access_line
                    #reply_line
                }
            };
    }
}
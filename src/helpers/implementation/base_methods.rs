use quote::ToTokens;
use syn::{ImplItemFn, Type, parse_str, PathArguments, FnArg, ReturnType, parse_quote};

use crate::helpers::contains_isolated_name;

pub fn method_normalization(method : &mut ImplItemFn, original_type_name : &str){
    //check signature inputs
    method.vis = parse_quote!(pub(super));
    method.sig.inputs.iter_mut().for_each(|fn_arg| {
        if let FnArg::Typed(typed)  = fn_arg {
            let type_ = typed.ty.as_mut();
            arg_method_normalization(type_,original_type_name);
        }        
    });
    //check returns
    if let ReturnType::Type(_,type_ ) = &mut method.sig.output  {
        return_method_normalization(type_, original_type_name, &method.sig.ident.to_string());        
    }
}

fn return_method_normalization(type_ : &mut Type, original_type_name : &str, method_name: &str){
    let message_error = 
    format!("The return types with Self allowed in the {method_name} 
    function are: Self, &Self, &mut Self, &[Self].
    Other formats are not supported!");
    match type_ {
        Type::Path(path) => {
            let last_segment = path.path.segments.last_mut().unwrap();
            let last_segment_str = last_segment.to_token_stream().to_string();
            if contains_isolated_name(&last_segment_str, "Self") ||
            contains_isolated_name(&last_segment_str, original_type_name){
                if last_segment_str == "Vec<Self>" || last_segment_str == format!("Vec<{original_type_name}>"){
                    *last_segment = parse_quote!(Vec<Self>);
                }else if last_segment_str == "Self" || last_segment_str == original_type_name{
                    *last_segment = parse_quote!(Self);
                }else{
                    panic!("{message_error}");
                }
            }
            
        },
        Type::Reference(reference) =>{
            let elem_str = reference.elem.to_token_stream().to_string();
            if contains_isolated_name(&elem_str, "Self") ||
            contains_isolated_name(&elem_str, original_type_name){
                if elem_str == "Self" || elem_str == original_type_name{
                    *reference.elem = parse_str("Self").unwrap();
                }else{
                    panic!("{message_error}");
                }
            }
        },
        Type::Slice(slice) => {
            let elem_str = slice.elem.to_token_stream().to_string();
            if contains_isolated_name(&elem_str, "Self") ||
            contains_isolated_name(&elem_str, original_type_name){
                if elem_str == "Self" || elem_str == original_type_name{
                    *slice.elem = parse_str("Self").unwrap();
                }else{
                    panic!("{message_error}");
                }
            }
        },
        _ => {
            let type_str = type_.to_token_stream().to_string();
            if contains_isolated_name(&type_str, "Self") ||
            contains_isolated_name(&type_str, original_type_name){
                panic!("{message_error}");
            }
        }
    }
}

fn arg_method_normalization( type_ : &mut Type ,original_type_name : &str){
    match type_ {
        syn::Type::Path(path) =>{
            let last_segment = path.path.segments.iter_mut().last().expect("A Path has no recognized segments.");
            if let PathArguments::AngleBracketed(a) = &mut last_segment.arguments {
                for arg in a.args.iter_mut(){
                    if let syn::GenericArgument::Type(type_) = arg{
                        arg_method_normalization(type_,original_type_name)
                    }
                }
            }
        },
        syn::Type::Reference(reference) => {
            let type_str = reference.elem.clone().to_token_stream().to_string();
            if type_str == "Self" || type_str == original_type_name{
                if reference.mutability.is_some(){
                    *type_ = parse_str("&mut RwLockWriteGuard<Self>").unwrap()
                }else{
                    *type_ = parse_str("RwLockReadGuard<Self>").unwrap()
                }
            }
        },syn::Type::Slice(slice) => {
            arg_method_normalization(slice.elem.as_mut(), original_type_name)
        },
        _ => return,
    }
}


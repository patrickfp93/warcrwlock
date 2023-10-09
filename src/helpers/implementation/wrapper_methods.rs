use quote::ToTokens;
use syn::{parse_quote, parse_str, FnArg, ImplItemFn, Pat, ReturnType, Stmt, Type};

use crate::helpers::{
    contains_isolated_name, get_mut_guard_name, get_ref_guard_name, BASE_FIELD_NAME,
    full_base_struct_name,
};

pub fn method_normalization(method: &mut ImplItemFn, origin_type_name: &str, generics: &str) {
    if &method.vis.to_token_stream().to_string() == "pub(super)" {
        method.vis = parse_quote!(pub(in super::super));
    }
    //Clear block
    let mut stmts = &mut method.block.stmts;
    stmts.clear();
    let mut has_receiver = false;
    //check method has self,&self, &mut self etc and generate guard.
    if let Some(receiver) = method.sig.receiver() {
        //stmts.push(parse_str(&format!("let {BASE_FIELD_NAME} = self.{BASE_FIELD_NAME}();")).unwrap());
        if receiver.mutability.is_some() {
            stmts.push(
                parse_str(&format!(
                    "let mut guard = self.{BASE_FIELD_NAME}.write().unwrap();"
                ))
                .unwrap(),
            )
        } else {
            stmts.push(
                parse_str(&format!(
                    "let guard = self.{BASE_FIELD_NAME}.read().unwrap();"
                ))
                .unwrap(),
            )
        }
        has_receiver = true;
    }
    let mut args = "".to_string();
    //check signature inputs and create possibles convertions.
    let len_args = method.sig.inputs.len();
    method
        .sig
        .inputs
        .iter_mut()
        .enumerate()
        .for_each(|(index, fn_arg)| {
            if let FnArg::Typed(typed) = fn_arg {
                let name_fn_arg = extract_name(&typed.pat, &mut args, index, len_args);
                arg_method_normalization(
                    typed.ty.as_mut(),
                    &name_fn_arg,
                    origin_type_name,
                    stmts,
                    &method.sig.ident.to_string(),
                )
            }
        });
    if has_receiver {
        let method_ident_str = method.sig.ident.to_string();
        let return_type = &mut method.sig.output;
        if let ReturnType::Type(_, type_) = return_type {
            return_method_normalization(
                type_.as_mut(),
                &mut stmts,
                &format!("guard.{method_ident_str}({args})"),
                origin_type_name,
                &method_ident_str,
                generics,
            );
        } else {
            stmts.push(parse_str(&format!("guard.{method_ident_str}({args});")).unwrap())
        }
    } else {
        let return_type = &mut method.sig.output;
        let method_ident_str = method.sig.ident.to_string();
        if let ReturnType::Type(_, type_) = return_type {
            return_method_normalization(
                type_.as_mut(),
                &mut stmts,
                &format!("{}::{method_ident_str}({args})",full_base_struct_name(origin_type_name)),
                origin_type_name,
                &method_ident_str,
                generics,
            );
        } else {
            stmts.push(
                parse_str(&format!(
                    "{}::{method_ident_str}({args});",full_base_struct_name(origin_type_name)
                ))
                .unwrap(),
            )
        }
    }
    
}

pub fn return_method_normalization(
    type_: &mut Type,
    stmts: &mut Vec<Stmt>,
    call_method_str: &str,
    original_type_name: &str,
    method_name: &str,
    generics: &str,
) {
    let message_error = &format!(
        "The return Type with Self allowed in the {method_name} 
            function are: Self, &Self, &mut Self,Vec<Self>. Other formats are not supported! Slice "
    );
    let ref_guard_name = get_ref_guard_name(original_type_name);
    let mut_guard_name = get_mut_guard_name(original_type_name);
    match type_ {
        Type::Path(path) => {
            let last_segment = path.path.segments.last_mut().unwrap();
            let last_segment_str = last_segment.to_token_stream().to_string();
            if contains_isolated_name(&last_segment_str, "Self")
                || contains_isolated_name(&last_segment_str, original_type_name)
            {
                if last_segment_str == "Vec<Self>"
                    || last_segment_str == format!("Vec<{original_type_name}>")
                {
                    stmts.push(parse_str(&format!("let reply = {call_method_str};")).unwrap());
                    stmts.push(
                        parse_str(&format!("let reply = Self::into_vec_wrapper(reply);")).unwrap(),
                    );
                    stmts.push(parse_str(&format!("return reply;")).unwrap());
                } else if last_segment_str == "Self" || last_segment_str == original_type_name {
                    stmts.push(
                        parse_str(&format!("let reply = {call_method_str}.into();")).unwrap(),
                    );
                    stmts.push(parse_str(&format!("return reply;")).unwrap());
                } else {
                    panic!("{message_error}");
                }
            } else {
                stmts.push(parse_str(&format!("let reply = {call_method_str};")).unwrap());
                stmts.push(parse_str(&format!("return reply;")).unwrap());
            }
        }
        Type::Reference(reference) => {
            let elem_str = reference.elem.to_token_stream().to_string();
            if contains_isolated_name(&elem_str, "Self")
                || contains_isolated_name(&elem_str, original_type_name)
            {
                if elem_str == "Self" || elem_str == original_type_name {
                    stmts.push(parse_str(&format!("{call_method_str};")).unwrap());
                    stmts.push(parse_str(&format!("drop(guard);")).unwrap());
                    stmts.push(parse_str(&format!("return self;")).unwrap());
                } else {
                    panic!("{message_error}");
                }
            } else {
                if reference.mutability.is_some() {
                    stmts.push(parse_str(&format!("let value = {call_method_str};")).unwrap());
                    stmts.push(
                        parse_str(&format!(
                            "let value = (value as *const {elem_str}) as *mut {elem_str};"
                        ))
                        .unwrap(),
                    );
                    stmts.push(
                        parse_str(&format!("return {mut_guard_name}::new(value, guard);")).unwrap(),
                    );
                    *type_ = parse_str(&format!("{mut_guard_name}<{elem_str}>")).unwrap()
                } else {
                    stmts.push(parse_str(&format!("let value = {call_method_str};")).unwrap());
                    stmts.push(
                        parse_str(&format!("let value = value as *const {elem_str};")).unwrap(),
                    );
                    stmts.push(
                        parse_str(&format!("return {ref_guard_name}::new(value, guard);")).unwrap(),
                    );
                    *type_ = parse_str(&format!("{ref_guard_name}<{elem_str},{generics}>")).unwrap()
                }
            }
        }
        Type::Slice(slice) => {
            let elem_str = slice.elem.to_token_stream().to_string();
            if contains_isolated_name(&elem_str, "Self")
                || contains_isolated_name(&elem_str, original_type_name)
            {
                if elem_str == "Self" || elem_str == original_type_name {
                    stmts.push(parse_str(&format!("let reply = {call_method_str};")).unwrap());
                    stmts.push(
                        parse_str(&format!(
                            "let reply = Self::into_vec_wrapper(reply.to_vec());"
                        ))
                        .unwrap(),
                    );
                    stmts.push(parse_str(&format!("return reply;")).unwrap());
                    *type_ = parse_str(&format!("Vec<Self>")).unwrap();
                } else {
                    panic!("{message_error}");
                }
            } else {
                stmts.push(parse_str(&format!("let value = {call_method_str};")).unwrap());
                stmts.push(parse_str(&format!("let value = value as *const {elem_str}")).unwrap());
                stmts.push(
                    parse_str(&format!("return {ref_guard_name}::new(value, guard);")).unwrap(),
                );
                *type_ = parse_str(&format!("{ref_guard_name}<{elem_str},{generics}>")).unwrap()
            }
        }
        _ => {
            let type_str = type_.into_token_stream().to_string();
            if contains_isolated_name(&type_str, "Self")
                || contains_isolated_name(&type_str, original_type_name)
            {
                panic!("{message_error}");
            } else {
                stmts.push(parse_str(&format!("let reply = {call_method_str};")).unwrap());
                stmts.push(parse_str(&format!("return reply;")).unwrap());
            }
        }
    }
}

fn arg_method_normalization(
    type_: &mut Type,
    arg_name: &str,
    origin_type_name: &str,
    stmts: &mut Vec<Stmt>,
    method_name: &str,
) {
    let error_message = &format!("The types of parameters with Self allowed in the {method_name} function are: Self, &Self, &mut Self, &[Self], Vec<Self>. Other formats are not supported!");
    match type_ {
        syn::Type::Path(type_path) => {
            let last_path_segment = type_path.path.segments.last().unwrap().clone();
            let last_path_segment_str = type_path
                .path
                .segments
                .last()
                .unwrap()
                .into_token_stream()
                .to_string();
            if contains_isolated_name(&last_path_segment_str, "Self")
                || contains_isolated_name(&last_path_segment_str, origin_type_name)
            {
                if last_path_segment.ident.to_string() == "Vec"
                    && (last_path_segment.arguments.to_token_stream().to_string() == "Self"
                        || last_path_segment.arguments.to_token_stream().to_string()
                            == origin_type_name)
                {
                    let stmt = parse_str(&format!(
                        "let {arg_name} = Self::into_vec_{BASE_FIELD_NAME}({arg_name});"
                    ))
                    .unwrap();
                    stmts.push(stmt);
                } else if last_path_segment_str == "Self"
                    || last_path_segment_str == origin_type_name
                {
                    let stmt = parse_str(&format!("let {arg_name} = {arg_name}.into();")).unwrap();
                    stmts.push(stmt);
                } else {
                    panic!("{error_message}")
                }
            }
        }
        syn::Type::Slice(slice) => {
            if let Some(approved) =
                self_checker(&slice.elem.to_token_stream().to_string(), origin_type_name)
            {
                if approved {
                    let stmt = parse_str(&format!(
                        "let {arg_name} = &Self::into_vec_{BASE_FIELD_NAME}({arg_name}.to_vec());"
                    ))
                    .unwrap();
                    stmts.push(stmt);
                } else {
                    panic!("{error_message}")
                }
            }
        }
        syn::Type::Reference(reference) => {
            if let Some(approved) = self_checker(
                &reference.elem.to_token_stream().to_string(),
                origin_type_name,
            ) {
                if approved {
                    /*stmts.push(parse_str(&format!(
                        "let {arg_name}_{BASE_FIELD_NAME} = {arg_name}.{BASE_FIELD_NAME}();"
                    )).unwrap());*/
                    let stmt: Stmt;
                    if reference.mutability.is_some() {
                        stmt = parse_str(&format!(
                            "let {arg_name} = &mut {arg_name}.{BASE_FIELD_NAME}.write().unwrap();"
                        ))
                        .unwrap();
                    } else {
                        stmt = parse_str(&format!(
                            "let {arg_name} = {arg_name}.{BASE_FIELD_NAME}.read().unwrap();"
                        ))
                        .unwrap();
                    }
                    stmts.push(stmt);
                } else {
                    panic!("{error_message}")
                }
            }
        }
        _ => {
            if contains_isolated_name(&type_.to_token_stream().to_string(), "Self")
                || contains_isolated_name(&type_.to_token_stream().to_string(), origin_type_name)
            {
                panic!("{error_message}")
            }
            return;
        }
    }
}

fn self_checker(element_str: &str, origin_type_name: &str) -> Option<bool> {
    if element_str == "Self" || element_str == origin_type_name {
        return Some(true);
    } else if contains_isolated_name(&element_str, "Self")
        || contains_isolated_name(&element_str, origin_type_name)
    {
        return Some(false);
    }
    None
}

fn extract_name(pat: &Pat, args: &mut String, index: usize, len_args: usize) -> String {
    let mut args_local = args.clone();
    if let Pat::Ident(pat_ident) = pat {
        let ident = pat_ident.ident.clone();
        args_local += &ident.to_string();
        if index < len_args - 1 {
            args_local += ",";
        }
        *args = args_local;
        return ident.to_string();
    }
    return "".into();
}

/*pub fn generate_helper(wrapper_impl_item: &ItemImpl) -> ImplItemFn {
    let wsn = extract_full_ident_type_from_impl(wrapper_impl_item).unwrap();
    let generics = wsn.arguments.clone();
    let bsn = wsn.to_token_stream();
    let bsn = quote::quote!(#bsn #generics);
    let bfn = to_token_stream(super::super::BASE_FIELD_NAME);
    let helper = parse_quote! {
        fn #bfn(&self) -> Arc<RwLock<#bsn>>{
            type W = Arc<RwLock<#bsn>>;
            let ptr = self as *const Self as *const u8;
            let size = size_of::<Self>();
            let bytes_struct: &[u8] = unsafe { std::slice::from_raw_parts(ptr, size) };
            let bytes_field: &[u8] = &bytes_struct[0..size_of::<W>()];
            let reply: W = unsafe { &*(bytes_field.as_ptr() as *const W) }.clone();
            reply
        }
    };
    return helper;

}*/

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_str, punctuated::Punctuated, token::Comma, FnArg, ImplItem, ImplItemFn, ItemImpl, Pat,
    Receiver, ReturnType, Signature, Type, Visibility,
};

use super::{ATTRIBUTE_WRAPPER_NAME, module::get_type_name, VISIBLE_METHOD};


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
        if &name_type == original_struct_name || &name_type == "Self" {
            return true;
        }
    }
    return false;
}

fn is_pre_wrapper(impl_item: &ImplItem, original_struct_name: &str) -> bool {
    if let ImplItem::Fn(method) = impl_item {
        return method
            .sig
            .inputs
            .iter()
            .any(|arg| is_self_type_arg(arg, original_struct_name))
            || method.attrs.iter().any(|atts| {
                atts.to_token_stream()
                    .to_string()
                    .contains(ATTRIBUTE_WRAPPER_NAME)
            });
    }
    false
}

fn is_builder(method: &ImplItemFn, original_struct_name: &str) -> bool {
    if let ReturnType::Type(_, type_) = method.sig.output.clone() {
        let type_name = get_type_name(type_);
        return type_name == "Self" || type_name == original_struct_name;
    }
    false
}

fn normalize_self_types(method: &mut ImplItemFn, original_struct_name: &str) {
    //change stmts in block
    let len = method.block.stmts.len();
    for (index, stmt) in method.block.stmts.iter_mut().enumerate() {
        let mut stmt_str = stmt.to_token_stream().to_string();
        stmt_str = stmt_str.replace(original_struct_name, "Self");
        if let None = stmt_str.find(';') {
            if index == len - 1 {
                stmt_str = format!("return {};", stmt_str);
            }
        }
        *stmt = parse_str(&stmt_str).expect(&format!(
            "The statement({}) is incorrect in \"{}\" method!",
            &stmt_str, method.sig.ident
        ));
    }
    if let ReturnType::Type(_, type_) = method.sig.output.clone() {
        if get_type_name(type_) == original_struct_name {
            method.sig.output = parse_str(" -> Self").unwrap();
        }
    }
}

fn convert_punctuated_to_string(args: &Punctuated<FnArg, Comma>) -> String {
    let mut reply = "".to_string();
    for (index, arg) in args.iter().enumerate() {
        if let FnArg::Typed(param) = arg {
            if let Pat::Ident(pat_ident) = &*param.pat {
                let ident = pat_ident.ident.clone();
                reply += &ident.to_string();
                if index < args.len() - 1 {
                    reply += ",";
                }
            }
        }
    }
    reply
}

fn build_wrapper_method_block(method: &mut ImplItemFn, original_struct_name: &str) {
    let mut new_block = method.block.clone();
    let name_method = method.sig.ident.to_string();
    let inputs = if method.sig.inputs.len() > 0 {
        convert_punctuated_to_string(&method.sig.inputs)
    } else {
        "".into()
    };
    new_block.stmts.clear();
    let new_stmt_str: String = format!(
        "return Self{{base: Arc::new(RwLock::new({}::{}({})))}};",
        format!("{}Base", original_struct_name),
        name_method,
        inputs
    );
    new_block.stmts.push(parse_str(&new_stmt_str).unwrap());
    method.block = new_block;
}

fn find_self(signature: &Signature) -> Option<Receiver> {
    // Verifica cada argumento na assinatura
    for input in &signature.inputs {
        if let FnArg::Receiver(r) = input {
            return Some(r.clone());
        }
    }
    None
}

fn has_reference_output(method: &ImplItemFn) -> bool {
    if let ReturnType::Type(_, type_) = method.sig.output.clone() {
        if let Type::Reference(_) = type_.as_ref() {
            return true;
        }
    }
    false
}

fn check_if_ideal_method(method: &ImplItemFn) -> bool{
    let mut reply = false;
    if let Visibility::Public(_) = method.vis {
        reply = true;
    } else if let Visibility::Restricted(_) = method.vis{
        reply = true;
    } else if method
        .attrs
        .iter()
        .any(|atts| atts.to_token_stream().to_string().contains(VISIBLE_METHOD))
    {
        reply = true;
    }
    reply
}

fn rebuild_implementations(
    base_impl_items: &mut Vec<ImplItem>,
    wrapper_impl_items: &mut Vec<ImplItem>,
    item_impl: ItemImpl,
    wrapper_self_ty_str: String,
    base_self_ty_str: String,
) {
    for impl_item in &item_impl.items {
        //is method?
        if let ImplItem::Fn(method) = impl_item {
            //is not private or visible to wrapper
            if check_if_ideal_method(method) {
                //does it have Self args?
                if is_pre_wrapper(impl_item, &wrapper_self_ty_str) {
                    wrapper_impl_items.push(impl_item.clone());
                } else if has_reference_output(method) {
                    base_impl_items.push(ImplItem::Fn(method.clone()));
                } else if is_builder(method, &wrapper_self_ty_str) {
                    let mut base_method = method.clone();
                    normalize_self_types(&mut base_method, &wrapper_self_ty_str);
                    base_impl_items.push(ImplItem::Fn(base_method));
                    let mut wrapper_method = method.clone();
                    build_wrapper_method_block(&mut wrapper_method, &wrapper_self_ty_str);
                    wrapper_impl_items.push(ImplItem::Fn(wrapper_method));
                } else {
                    let reference = if let Some(receiver) = find_self(&method.sig) {
                        if receiver.mutability.is_some() {
                            "self.base.write().unwrap().".to_string()
                        } else {
                            "self.base.read().unwrap().".to_string()
                        }
                    } else {
                        format!("{}::", base_self_ty_str)
                    };
                    base_impl_items.push(ImplItem::Fn(method.clone()));
                    let mut wrapper_method = method.clone();
                    wrapper_method.block.stmts.clear();
                    let mut stmt_str: String = if let ReturnType::Type(_, _) = method.sig.output {
                        "return".into()
                    } else {
                        "".into()
                    };
                    stmt_str = format!(
                        "{} {}{}({});",
                        stmt_str,
                        reference,
                        method.sig.ident.to_string(),
                        convert_punctuated_to_string(&method.sig.inputs)
                    );
                    wrapper_method
                        .block
                        .stmts
                        .push(parse_str(&stmt_str).unwrap());
                    wrapper_impl_items.push(ImplItem::Fn(wrapper_method));
                }
            } else {
                base_impl_items.push(impl_item.clone());
            }
        } else {
            base_impl_items.push(impl_item.clone());
        }
    }
}

pub fn extend_impl(item_impl: ItemImpl) -> TokenStream {
    let wrapper_self_ty = item_impl.self_ty.clone();
    let wrapper_self_ty_str = get_type_name(wrapper_self_ty.clone());
    let base_self_ty_str = wrapper_self_ty_str.clone() + "Base";
    let base_self_ty: Box<Type> = parse_str(&base_self_ty_str).unwrap();

    let mut wrapper_impl_items = vec![];
    let mut base_impl_items = vec![];
    rebuild_implementations(
        &mut base_impl_items,
        &mut wrapper_impl_items,
        item_impl.clone(),
        wrapper_self_ty_str,
        base_self_ty_str.to_string(),
    );
    let mut base_impl = item_impl.clone();
    base_impl.self_ty = base_self_ty;
    base_impl.items = base_impl_items;

    let mut wrapper_impl = item_impl.clone();
    wrapper_impl.self_ty = wrapper_self_ty;
    wrapper_impl.items = wrapper_impl_items;

    let output = quote! {
        #base_impl
        #wrapper_impl
    };
    output.into()
}

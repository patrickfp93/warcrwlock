use std::io::Read;

use proc_macro2::TokenStream;
use quote::ToTokens;
use static_init::dynamic;
use syn::{ItemStruct, parse_file, Item};

pub mod implementation;
pub mod struture;

pub(crate) const ATTRIBUTE_NAME: &str = "warcrwlock";
pub(crate) const BASE_STRUTURE_NAME: &str = "Core";
pub(crate) const BASE_FIELD_NAME: &str = "_core";

pub fn contains_isolated_name<C: ToString + ?Sized,T : ToString + ?Sized>(content: &C, target: &T) -> bool {
    get_isolated_name_index(content, target).is_some()
}

pub fn get_isolated_name_index<C: ToString + ?Sized,T : ToString + ?Sized>(content: &C, target: &T) -> Option<usize> {
    //refazer função
    // Enquanto houver ocorrências da substring na string de entrada
    let content = &content.to_string();
    let target = &target.to_string().replace(" ", "");
    if let Some(start) = content.find(target) {        
        // Verificar se a substring está cercada por caracteres não alfanuméricos
        let end = start + target.len();
        if start > 0{
            let slice = &content[start-1..start];
            let previus_char = slice.chars().nth(0).unwrap();
            if previus_char.is_alphabetic() || previus_char == '_'{
                return  None;
            }
        }
        if end < content.len() -1{
            let slice = &content[end..end+1];
            let next_char = slice.chars().nth(0).unwrap();
            //panic!("target: {target} target.len(): {} \n start: {start} \n slice: {slice}",target.len());
            if next_char.is_alphabetic() || next_char == '_'{
                return  None;
            }
        }
        return Some(start);
    }
    None
}

pub fn get_ref_guard_name(original_name_type: &str) -> String {
    format!("{original_name_type}RefLock")
}
pub fn get_mut_guard_name(original_name_type: &str) -> String {
    format!("{original_name_type}MutLock")
}
pub fn to_token_stream<T :ToString>(value :T) -> proc_macro2::TokenStream{
    value.to_string().parse().unwrap()
}

fn full_base_struct_name<T :ToString>(original_struct_name : T) -> String{
    format!("{}{BASE_STRUTURE_NAME}",original_struct_name.to_string())
}

#[dynamic] 
static mut IMPL_ID: usize = 0;
pub fn get_mod_ident()-> TokenStream{
    let mut impl_id = IMPL_ID.write();
    let current_id = *impl_id;
    *impl_id += 1;
    to_token_stream(format!("_into_{current_id}"))
}

pub fn struct_is_root(original_struture : &ItemStruct) -> Result<bool, Box<dyn std::error::Error>>{
    let mut path = std::env::current_dir().unwrap();
    path.push("src");
    path.push("lib.rs");
    if !path.exists(){
        path.pop();
        path.push("main.rs");
    }    
    //panic!("path:{}",path.to_str().unwrap());
    let mut file = "".to_string();
    std::fs::File::open(&path)?.read_to_string(&mut file)?;
    let file = parse_file(&file)?;
    let original_struture_str = original_struture.to_token_stream().to_string();
    let possible_struture = file.items.iter().find(|item|{
        if let Item::Struct(item_struct) = item{
            if item_struct.ident == original_struture.ident{
                if original_struture_str == item_struct.to_token_stream().to_string(){
                    return true;
                } 
            }
        }
        false
    });
    Ok(possible_struture.is_some())
}
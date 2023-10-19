use syn::{ItemStruct, parse_str, Ident};

pub fn wrapper_normalization(wrapper: ItemStruct, possible_custom_ident : Option<Ident>) -> ItemStruct {
    let mut reader_struture = super::normalizations::wrapper_normalization(wrapper);
    if let Some(custom_ident) = possible_custom_ident{
        reader_struture.ident = custom_ident;
    }else {
        reader_struture.ident = parse_str(&format!("{}{}",reader_struture.ident,crate::helpers::DEFAULT_REFERENCE_STRUTURE_NAME)).unwrap()
    }
    reader_struture
}

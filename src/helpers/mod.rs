pub mod module;
pub mod implementation;
pub mod struture;
const ATTRIBUTE_NAME : &str = "warcrwlock";
const ATTRIBUTE_WRAPPER_NAME : &str = "wrapper_method";
const VISIBLE_METHOD : &str = "visible_to_wrapper";
const DERIVE_MACRO : &str = "derive";


pub fn contains_isolated_name(input_string : &str, target : &str) -> bool{
    // Enquanto houver ocorrências da substring na string de entrada
    if let Some(idx) = input_string.find(target) {
        // Verificar se a substring está cercada por caracteres não alfanuméricos
        let is_isolated = {
            let before = input_string[..idx].chars().last();
            let after = input_string[idx + target.len()..].chars().next();
            before.map_or(true, |c| !c.is_ascii_alphanumeric())
                && after.map_or(true, |c| !c.is_ascii_alphanumeric())
        };

        // Se a substring estiver isolada, retornar true
        if is_isolated {
            return true;
        }
    }
    false
}

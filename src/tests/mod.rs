
#[test]
fn test_helpers_get_isolated_name_index(){
    let target = "case";
    //true case
    let content = "true::case::";
    let possible_index = crate::helpers::get_isolated_name_index(content,target);
    assert_eq!(possible_index, Some(6));
    //false case
    let content = "false_case";
    let possible_index = crate::helpers::get_isolated_name_index(content,target);
    assert_eq!(possible_index, None)
}

/*#[test]
fn test_helpers_replace_name_isolated(){
    let target = "case";
    let substitute = "super_case";
    let expected_true = "true super_case";
    let expected_false = "false_case";
    //true case
    let mut content = "true case".to_string();
    //println!("{content}");
    assert!(crate::helpers::replace_name_isolated(&mut content, target, substitute));
    //println!("{content}");
    assert_eq!(content,expected_true);
    //false case
    let mut content = "false_case".to_string();
    //println!("{content}");
    assert!(!crate::helpers::replace_name_isolated(&mut content, target, substitute));
    //println!("{content}");
    assert_eq!(content,expected_false);
}*/
use crate::wrapper_model::basic_impls::Wrapper;

#[test]
pub fn test_asref_arc() {
    let wrapper = Wrapper::new();
    let wrapper2 = Wrapper::new();
    println!("wrapper: {wrapper:?}");
    assert_ne!(wrapper, wrapper2)
}


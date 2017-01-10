use super::{fixtures, parser};

#[test]
fn it_extracts_const_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::CONST_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"alloc::boxed::HEAP".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"HEAP");
            assert_eq!(e.entry_type, *"Constant".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_enum_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::ENUM_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"collections::borrow::Cow".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"Cow");
            assert_eq!(e.entry_type, *"Enum".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_function_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::FUNCTION_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"std::fs::metadata".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"metadata");
            assert_eq!(e.entry_type, *"Function".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_macro_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::MACRO_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"std::println!".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"println!");
            assert_eq!(e.entry_type, *"Macro".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_trait_method_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TRAIT_METHOD_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    match entries[1] {
        Some(ref e) => {
            assert_eq!(e.entry_name,
                       *"collections::borrow::Borrow::borrow".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"borrow");
            assert_eq!(e.entry_type, *"Method".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_enum_method_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::ENUM_METHOD_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    match entries[3] {
        Some(ref e) => {
            assert_eq!(e.entry_name,
                       *"collections::borrow::Cow::to_mut".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"to_mut");
            assert_eq!(e.entry_type, *"Method".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_struct_method_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::STRUCT_METHOD_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 4);
    match entries[3] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"core::any::TypeId::of".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"of");
            assert_eq!(e.entry_type, *"Method".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_module_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::MODULE_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"collections".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"collections");
            assert_eq!(e.entry_type, *"Module".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_struct_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::STRUCT_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"std::io::Bytes".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"Bytes");
            assert_eq!(e.entry_type, *"Struct".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_trait_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TRAIT_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"collections::fmt::Binary".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"Binary");
            assert_eq!(e.entry_type, *"Trait".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_type_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TYPE_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"bool::Output".to_string());
            assert_eq!(*e.anchor_name.split("/").nth(4).unwrap(), *"Output");
            assert_eq!(e.entry_type, *"Type".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_impl_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::IMPL_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_type, *"Method".to_string());
            assert_eq!(e.is_section, true);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_field_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::FIELD_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);

    match entries[1] {
        Some(ref e) => {
            assert_eq!(e.entry_type, *"Field".to_string());
            assert_eq!(e.is_section, false);
            assert_eq!(e.entry_name, *"core::ops::Range::start".to_string());
        }
        _ => assert!(false),
    }
}

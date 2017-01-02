use super::{fixtures, parser};

#[test]
fn it_extracts_const_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::CONST_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"HEAP".to_string());
            assert_eq!(e.entry_type, *"constant".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_enum_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::ENUM_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"Cow".to_string());
            assert_eq!(e.entry_type, *"enum".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_function_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::FUNCTION_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"metadata".to_string());
            assert_eq!(e.entry_type, *"function".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_macro_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::MACRO_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"println!".to_string());
            assert_eq!(e.entry_type, *"macro".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_method_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::METHOD_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"hash".to_string());
            assert_eq!(e.entry_type, *"method".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_module_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::MODULE_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"collections".to_string());
            assert_eq!(e.entry_type, *"module".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_struct_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::STRUCT_SNIPPET);

    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"Bytes".to_string());
            assert_eq!(e.entry_type, *"struct".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_trait_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TRAIT_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"Binary".to_string());
            assert_eq!(e.entry_type, *"trait".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_type_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TYPE_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_name, *"Output".to_string());
            assert_eq!(e.entry_type, *"type".to_string());
            assert_eq!(e.is_section, false);
        }
        _ => assert!(false),
    }
}

#[test]
fn it_extracts_impl_entry_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::IMPL_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        Some(ref e) => {
            assert_eq!(e.entry_type, *"method".to_string());
            assert_eq!(e.is_section, true);
            assert_eq!(e.entry_name, *"impl<T: Clone> Arc<T>".to_string());
        }
        _ => assert!(false),
    }
}

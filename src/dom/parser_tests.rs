use super::{fixtures, parser};

#[test]
fn it_extracts_name_for_const_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::CONST_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Const(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"HEAP".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_enum_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::ENUM_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Enum(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"Cow".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_function_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::FUNCTION_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Function(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"metadata".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_macro_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::MACRO_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Macro(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"println!".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_method_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::METHOD_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Method(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"hash".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_module_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::MODULE_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Module(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"collections".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_struct_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::STRUCT_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Struct(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"Bytes".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_trait_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TRAIT_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Trait(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"Binary".to_string()),
        None => assert_eq!(true, false),
    }
}

#[test]
fn it_extracts_name_for_type_correctly() {
    let dom = fixtures::dom_from_snippet(fixtures::TYPE_SNIPPET);

    let mut entries: Vec<super::Entry> = Vec::new();
    parser::walk_tree(&dom.document, &mut entries);

    assert_eq!(entries.len(), 1);
    match entries[0] {
        super::Entry::Type(_) => assert!(true),
        _ => assert!(false),
    }

    match parser::extract_entry_name(&entries[0]) {
        Some(s) => assert_eq!(s, *"Output".to_string()),
        None => assert_eq!(true, false),
    }
}

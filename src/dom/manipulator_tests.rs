use html5ever::rcdom::NodeEnum::Element;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::rcdom::RcDom;

use super::{fixtures, manipulator, parser};

// our snippets contain the entry elements as first elements in the body, i.e. after dash
// anchors are inserted, the first body-element should be a dash anchor
// This code is pretty ugly ... but works for now
fn first_element_is_dash_anchor(dom: &RcDom) -> bool {
    let ref html_element = dom.document.borrow().children[0];
    for e in html_element.borrow().children.iter() {
        if let Element(ref name, _, _) = e.borrow().node {
            if &(*name.local.to_ascii_lowercase()) == "body" {
                for body_child in e.borrow().children.iter() {
                    if let Element(_, _, ref attrs) = body_child.borrow().node {
                        if let Some(class_attr) = attrs.iter()
                                                       .find(|ref x| {
                                                           x.name == qualname!("", "class")
                                                       })
                                                       .and_then(|c| {
                                                           Some(c.clone().value.to_string())
                                                       }) {
                            if class_attr == "dashAnchor" {
                                return true;
                            }
                            return false;
                        }
                    }
                }
            }
        }
    }
    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("document is missing dash anchor:\n{}", result);

    false
}

#[test]
fn it_inserts_dash_anchor_before_const_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::CONST_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_enum_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::ENUM_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_function_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::FUNCTION_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_macro_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::MACRO_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_method_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::STRUCT_METHOD_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_module_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::MODULE_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_struct_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::STRUCT_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_trait_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::TRAIT_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

#[test]
fn it_inserts_dash_anchor_before_type_entry() {
    let mut dom = fixtures::dom_from_snippet(fixtures::TYPE_SNIPPET);
    let mut entries: Vec<Option<super::Entry>> = Vec::new();
    parser::walk_tree(&dom.document, String::from(""), &mut entries);
    manipulator::add_dash_links(&mut dom, &entries);

    assert!(first_element_is_dash_anchor(&dom));
}

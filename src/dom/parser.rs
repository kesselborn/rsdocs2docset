use html5ever::rcdom::NodeEnum::{Element, Text};
use html5ever::rcdom::{Handle, RcDom};
use super::Entry;


pub fn find_entry_elements(dom: &mut RcDom) -> Vec<Entry> {
    let mut entries = vec![];
    walk_tree(&dom.document, &mut entries);
    entries
}

pub fn walk_tree(h: &Handle, entries: &mut Vec<Entry>) {
    for e in h.borrow().children.iter() {
        if let Element(ref name, _, ref attrs) = e.borrow().node {
            let tag = &(*name.local.to_ascii_lowercase());
            if let Some(class_attr) = attrs.iter()
                                           .find(|ref x| x.name == qualname!("", "class"))
                                           .and_then(|c| Some(c.clone().value.to_string())) {

                match (tag, class_attr.as_str()) {
                    ("h4", "method") => entries.push(Entry::Method(e.clone())),
                    ("h4", "type") => entries.push(Entry::Type(e.clone())),
                    ("section", "content constant") => entries.push(Entry::Const(e.clone())),
                    ("section", "content enum") => entries.push(Entry::Enum(e.clone())),
                    ("section", "content fn") => entries.push(Entry::Function(e.clone())),
                    ("section", "content macro") => entries.push(Entry::Macro(e.clone())),
                    ("section", "content mod") => entries.push(Entry::Module(e.clone())),
                    ("section", "content struct") => entries.push(Entry::Struct(e.clone())),
                    ("section", "content trait") => entries.push(Entry::Trait(e.clone())),
                    (_, _) => {}
                }
            }
        }
        walk_tree(e, entries);
    }
}

fn find_element_with_class(h: &Handle, class_value: &str) -> Option<Handle> {
    for e in h.borrow().children.iter() {
        if let Element(_, _, ref attrs) = e.borrow().node {
            if attrs.iter()
                    .find(|ref attr| attr.name == qualname!("", "class"))
                    .and_then(|attr| Some(attr.value.to_string() == class_value))
                    .unwrap_or(false) {
                return Some(e.clone());
            }
        }
        if let Some(e) = find_element_with_class(e, class_value) {
            return Some(e);
        }
    }
    None
}

fn get_text(h: &Handle) -> Option<String> {
    let node = h.borrow();
    for e in node.children.iter() {
        if let Text(ref t) = e.borrow().node {
            return Some(t.to_string());
        }
    }

    None
}

// ## and_then oder so verwenden
pub fn extract_entry_name(e: &Entry) -> Option<String> {
    let name_element = match *e {
        Entry::Const(ref e) => find_element_with_class(e, "constant"),
        Entry::Enum(ref e) => find_element_with_class(e, "enum"),
        Entry::Function(ref e) => find_element_with_class(e, "fn"),
        Entry::Method(ref e) => find_element_with_class(e, "fnname"),
        Entry::Macro(ref e) => find_element_with_class(e, "macro"),
        Entry::Module(ref e) => find_element_with_class(e, "mod"),
        Entry::Struct(ref e) => find_element_with_class(e, "struct"),
        Entry::Trait(ref e) => find_element_with_class(e, "trait"),
        Entry::Type(ref e) => find_element_with_class(e, "type"),
    };

    match name_element {
        Some(element) => get_text(&element),
        None => None,
    }
}

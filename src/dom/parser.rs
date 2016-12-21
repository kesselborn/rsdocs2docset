use html5ever::rcdom::NodeEnum::{Element, Text};
use html5ever::rcdom::{Handle, RcDom};
use super::Entry;


pub fn find_entry_elements(dom: &mut RcDom) -> Vec<Option<Entry>> {
    let mut entries = vec![];
    walk_tree(&dom.document, &mut entries);
    entries
}


pub fn walk_tree(h: &Handle, entries: &mut Vec<Option<Entry>>) {
    for e in h.borrow().children.iter() {
        if let Element(ref name, _, ref attrs) = e.borrow().node {
            let tag = &(*name.local.to_ascii_lowercase());
            if let Some(class_attr) = attrs.iter()
                                           .find(|ref x| x.name == qualname!("", "class"))
                                           .and_then(|c| Some(c.clone().value.to_string())) {

                match (tag, class_attr.as_str()) {
                    ("h4", "method") => entries.push(Entry::new(e.clone(),
                                                                "method",
                                                                extract_entry_name(&e, "fnname"))),
                    ("h4", "type") =>
                        entries.push(Entry::new(e.clone(), "type", extract_entry_name(&e, "type"))),
                    ("section", "content constant") =>
                        entries.push(Entry::new(e.clone(),
                                                "constant",
                                                extract_entry_name(&e, "constant"))),
                    ("section", "content enum") =>
                        entries.push(Entry::new(e.clone(), "enum", extract_entry_name(&e, "enum"))),
                    ("section", "content fn") =>
                        entries.push(Entry::new(e.clone(),
                                                "function",
                                                extract_entry_name(&e, "fn"))),
                    ("section", "content macro") =>
                        entries.push(Entry::new(e.clone(),
                                                "macro",
                                                extract_entry_name(&e, "macro"))),
                    ("section", "content mod") =>
                        entries.push(Entry::new(e.clone(), "module", extract_entry_name(&e, "mod"))),
                    ("section", "content struct") =>
                        entries.push(Entry::new(e.clone(),
                                                "struct",
                                                extract_entry_name(&e, "struct"))),
                    ("section", "content trait") =>
                        entries.push(Entry::new(e.clone(),
                                                "trait",
                                                extract_entry_name(&e, "trait"))),
                    (_, _) => {}
                }
            }
        }
        walk_tree(e, entries);
    }
}

pub fn extract_entry_name(e: &Handle, element_class: &str) -> Option<String> {
    find_element_with_class(e, element_class).and_then(|x| get_text(&x))
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

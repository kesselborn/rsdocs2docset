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
                    ("h3", "impl") => entries.push(Entry::new(e.clone(),
                                                              "Method",
                                                              extract_entry_name(&e, "in-band"),
                                                              true)),
                    ("h4", "method") => entries.push(Entry::new(e.clone(),
                                                                "Method",
                                                                extract_entry_name(&e, "fnname"),
                                                                false)),
                    ("h4", "type") => entries.push(Entry::new(e.clone(),
                                                              "Type",
                                                              extract_entry_name(&e, "type"),
                                                              false)),
                    ("section", "content constant") =>
                        entries.push(Entry::new(e.clone(),
                                                "Constant",
                                                extract_entry_name(&e, "constant"),
                                                false)),
                    ("section", "content enum") =>
                        entries.push(Entry::new(e.clone(),
                                                "Enum",
                                                extract_entry_name(&e, "enum"),
                                                false)),
                    ("section", "content fn") =>
                        entries.push(Entry::new(e.clone(),
                                                "Function",
                                                extract_entry_name(&e, "fn"),
                                                false)),
                    ("section", "content macro") =>
                        entries.push(Entry::new(e.clone(),
                                                "Macro",
                                                extract_entry_name(&e, "macro"),
                                                false)),
                    ("section", "content mod") =>
                        entries.push(Entry::new(e.clone(),
                                                "Module",
                                                extract_entry_name(&e, "mod"),
                                                false)),
                    ("section", "content struct") =>
                        entries.push(Entry::new(e.clone(),
                                                "Struct",
                                                extract_entry_name(&e, "struct"),
                                                false)),
                    ("section", "content trait") =>
                        entries.push(Entry::new(e.clone(),
                                                "Trait",
                                                extract_entry_name(&e, "trait"),
                                                false)),
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
                    .and_then(|attr| Some(attr.value.to_string() == class_value)) !=
               None {
                return Some(e.clone());
            }
        }

        // recurse down and check children
        if let Some(e) = find_element_with_class(e, class_value) {
            return Some(e);
        }
    }
    None
}

fn get_text(h: &Handle) -> Option<String> {
    let mut text_tokens = Vec::new();
    let node = h.borrow();

    // get all text from current and all descendent elements: <span>hallo <a href="http://heise.de">heise</a>!</span> -> hallo heise !
    for e in node.children.iter() {
        match e.borrow().node {
            Text(ref t) => text_tokens.push(t.to_string()),
            _ => text_tokens.push(get_text(e).unwrap_or(String::from(""))),
        }
    }

    // adjust extracted text: mainly remove white space and some unicode characters
    if text_tokens.len() > 0 {
        Some(String::from(text_tokens.join(" ").trim())
                 .replace("\u{a0}", "")
                 .replace(" >", ">")
                 .replace(" <", "<")
                 .replace("  ", " "))
    } else {
        None
    }
}

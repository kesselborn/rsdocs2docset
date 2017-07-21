use html5ever::rcdom::{Handle, RcDom};
use html5ever::rcdom::NodeEnum::{Element, Text};
use super::Entry;


pub fn find_entry_elements(dom: &mut RcDom) -> Vec<Option<Entry>> {
    let mut entries = vec![];
    walk_tree(&dom.document, String::from(""), &mut entries);
    entries
}


pub fn walk_tree(h: &Handle, context: String, entries: &mut Vec<Option<Entry>>) {
    let mut current_context: Option<String> = None;

    for e in &h.borrow().children {
        if let Element(ref name, _, ref attrs) = e.borrow().node {
            let tag = &(*name.local.to_ascii_lowercase());
            if let Some(class_attr) = attrs.iter()
                .find(|x| x.name == qualname!("", "class"))
                .and_then(|c| Some(c.clone().value.to_string())) {

                match (tag, class_attr.as_str()) {
                    ("h3", "impl") => {
                        entries.push(Entry::new(e.clone(),
                                                "Method",
                                                extract_entry_name(e, "in-band"),
                                                true))
                    }

                    ("h4", "method") | ("h3", "method") => {
                        if let Some(entry_name) = extract_entry_name(e, "fnname") {
                            entries.push(Entry::new(e.clone(),
                                                    "Method",
                                                    Some(format!("{}::{}", context, entry_name)),
                                                    false))
                        }
                    }

                    ("h3", "method stab") => {
                        if let Some(entry_name) = extract_entry_name(e, "fnname") {
                            entries.push(Entry::new(e.clone(),
                                                    "Method",
                                                    Some(format!("{}::{}", context, entry_name)),
                                                    false))
                        }
                    }

                    ("span", "variant") => {
                        if let Some(entry_name_with_parenthesis) =
                            extract_entry_name(e, "invisible") {
                            let entry_name = entry_name_with_parenthesis.split('(')
                                .nth(0)
                                .expect("split did not have any elements?");

                            entries.push(Entry::new(e.clone(),
                                                    "Variant",
                                                    Some(format!("{}::{}", context, entry_name)),
                                                    false))
                        }
                    }

                    ("span", "structfield") => {
                        if let Some(field_and_type) = extract_entry_name(e, "invisible") {
                            let entry_name = field_and_type.split(':')
                                .nth(0)
                                .expect("split did not have any elements?");

                            entries.push(Entry::new(e.clone(),
                                                    "Field",
                                                    Some(format!("{}::{}", context, entry_name)),
                                                    false))
                        }
                    }

                    ("h4", "type") => {
                        if let Some(entry_name) = extract_entry_name(e, "type") {
                            entries.push(Entry::new(e.clone(),
                                                    "Type",
                                                    Some(format!("{}::{}", context, entry_name)),
                                                    false))
                        }
                    }

                    ("section", "content") => {
                        if extract_entry_name(e, "constant").is_some() {
                            entries.push(Entry::new(e.clone(),
                                                    "Constant",
                                                    extract_entry_name(e, "in-band"),
                                                    false))
                        } else if extract_entry_name(e, "enum").is_some() {
                            current_context = extract_entry_name(e, "in-band");
                            entries.push(Entry::new(e.clone(), "Enum", current_context.clone(), false))
                        } else if extract_entry_name(e, "macro").is_some() {
                            entries.push(Entry::new(e.clone(),
                                                    "Macro",
                                                    extract_entry_name(e, "in-band"),
                                                    false))
                        } else if extract_entry_name(e, "trait").is_some() {
                            current_context = extract_entry_name(e, "in-band");
                            entries.push(Entry::new(e.clone(),
                                                    "Trait",
                                                    current_context.clone(),
                                                    false))
                        } else if extract_entry_name(e, "mod").is_some() {
                            entries.push(Entry::new(e.clone(),
                                                    "Module",
                                                    extract_entry_name(e, "in-band"),
                                                    false))
                        } else if extract_entry_name(e, "struct").is_some() {
                            current_context = extract_entry_name(e, "in-band");
                            entries.push(Entry::new(e.clone(),
                                                    "Struct",
                                                    current_context.clone(),
                                                    false))
                        } else if extract_entry_name(e, "fn").is_some() {
                            entries.push(Entry::new(e.clone(),
                                                    "Function",
                                                    extract_entry_name(e, "in-band"),
                                                    false))
                        }
                    }

                    (_, _) => {}
                }
            }
        }

        walk_tree(e,
                  current_context.clone().unwrap_or(context.clone()),
                  entries);
    }
}

pub fn extract_entry_name(e: &Handle, element_class: &str) -> Option<String> {
    let ignore_first_text_element =
        !["invisible", "type", "fnname"].iter().any(|&x| x == element_class);
    find_element_with_class(e, element_class).and_then(|x| get_text(&x, ignore_first_text_element))
}

fn find_element_with_class(h: &Handle, class_value: &str) -> Option<Handle> {
    for e in &h.borrow().children {
        if let Element(_, _, ref attrs) = e.borrow().node {
            if attrs.iter()
                .find(|attr| attr.name == qualname!("", "class"))
                .and_then(|attr| Some(attr.value.to_string() == class_value))
                .unwrap_or(false) {
                return Some(e.clone());
            }
        }

        // recurse down and check children
        if let Some(x) = find_element_with_class(e, class_value) {
            return Some(x);
        }
    }
    None
}

fn get_text(h: &Handle, ignore_first_text_element: bool) -> Option<String> {
    let mut text_tokens = Vec::new();
    let node = h.borrow();

    // get all text from current and all descendent elements: <span>hallo <a href="http://heise.de">heise</a>!</span> -> hallo heise !
    for e in &node.children {
        match e.borrow().node {
            Text(ref t) => text_tokens.push(t.to_string()),

            // if node is not a text node, recurse down
            _ => {
                if let Some(s) = get_text(e, false) {
                    text_tokens.push(s)
                }
            }
        }
    }

    // adjust extracted text: mainly remove white space and some unicode characters
    if text_tokens.is_empty() {
        None
    } else {
        if ignore_first_text_element {
            text_tokens.remove(0);
        }
        Some(String::from(text_tokens.join(" ").trim())
            .replace("\u{a0}", "")
            .replace(" >", ">")
            .replace(" <", "<")
            .replace(" ::", "::")
            .replace(":: ", "::")
            .replace("  ", " "))
    }
}

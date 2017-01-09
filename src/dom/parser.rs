use html5ever::rcdom::NodeEnum::{Element, Text};
use html5ever::rcdom::{Handle, RcDom};
use super::Entry;


pub fn find_entry_elements(dom: &mut RcDom) -> Vec<Option<Entry>> {
    let mut entries = vec![];
    walk_tree(&dom.document, String::from(""), &mut entries);
    entries
}


pub fn walk_tree(h: &Handle, context: String, entries: &mut Vec<Option<Entry>>) {
    let mut current_context: Option<String> = None;

    for e in h.borrow().children.iter() {
        if let Element(ref name, _, ref attrs) = e.borrow().node {
            let tag = &(*name.local.to_ascii_lowercase());
            if let Some(class_attr) = attrs.iter()
                                           .find(|ref x| x.name == qualname!("", "class"))
                                           .and_then(|c| Some(c.clone().value.to_string())) {

                match (tag, class_attr.as_str()) {
                    ("h3", "impl") =>
                        entries.push(Entry::new(e.clone(),
                                                "Method",
                                                extract_entry_name(&e, Some("in-band")),
                                                true)),

                    ("h4", "method") => entries.push(Entry::new(e.clone(),
                                                                "Method",
                                                                Some(format!("{}::{}",
																			 context,
                                                             extract_entry_name(&e,
                                                                                Some("fnname"))
                                                                 .unwrap_or(String::from("")))),
                                                                false)),

                    ("h3", "method stab") => entries.push(Entry::new(e.clone(),
                                                                     "Method",
                                                                     Some(format!("{}::{}",
																			 context,
                                                             extract_entry_name(&e,
                                                                                Some("fnname"))
                                                                 .unwrap_or(String::from("")))),
                                                                     false)),

                    ("h4", "type") =>
                        entries.push(Entry::new(e.clone(),
                                                "Type",
                                                extract_entry_name(&e, None).and_then(|s| {
                                                    Some(s.replace("type ", "::")
                                                          .replace(" = ", "=")
                                                          .rsplit('=')
                                                          .collect())
                                                }),
                                                false)),

                    ("section", "content constant") =>
                        entries.push(Entry::new(e.clone(),
                                                "Constant",
                                                extract_entry_name(&e, Some("in-band")),
                                                false)),

                    ("section", "content enum") => {
                        current_context = extract_entry_name(&e, Some("in-band"))
                                              .and_then(|s| Some(s.replace("Enum ", "")));
                        entries.push(Entry::new(e.clone(), "Enum", current_context.clone(), false))
                    }

                    ("section", "content fn") =>
                        entries.push(Entry::new(e.clone(),
                                                "Function",
                                                extract_entry_name(&e, Some("in-band"))
                                                    .and_then(|s| {
                                                        Some(s.replace("Function ", ""))
                                                    }),
                                                false)),

                    ("section", "content macro") =>
                        entries.push(Entry::new(e.clone(),
                                                "Macro",
                                                extract_entry_name(&e, Some("in-band")),
                                                false)),

                    ("section", "content mod") =>
                        entries.push(Entry::new(e.clone(),
                                                "Module",
                                                extract_entry_name(&e, Some("in-band"))
                                                    .and_then(|s| Some(s.replace("Crate ", ""))),
                                                false)),

                    ("section", "content struct") => {
                        current_context = extract_entry_name(&e, Some("in-band"))
                                              .and_then(|s| Some(s.replace("Struct ", "")));
                        entries.push(Entry::new(e.clone(),
                                                "Struct",
                                                current_context.clone(),
                                                false))
                    }

                    ("section", "content trait") => {
                        current_context = extract_entry_name(&e, Some("in-band"))
                                              .and_then(|s| Some(s.replace("Trait ", "")));
                        entries.push(Entry::new(e.clone(), "Trait", current_context.clone(), false))
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

pub fn extract_entry_name(e: &Handle, element_class: Option<&str>) -> Option<String> {
    find_element_with_class(e, element_class).and_then(|x| get_text(&x))
}

fn find_element_with_class(h: &Handle, class_value: Option<&str>) -> Option<Handle> {
    if class_value.is_none() {
        return Some(h.clone());
    }

    for e in h.borrow().children.iter() {
        if let Element(_, _, ref attrs) = e.borrow().node {
            if attrs.iter()
                    .find(|ref attr| attr.name == qualname!("", "class"))
                    .and_then(|attr| Some(attr.value.to_string() == class_value.unwrap()))
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

fn get_text(h: &Handle) -> Option<String> {
    let mut text_tokens = Vec::new();
    let node = h.borrow();

    // get all text from current and all descendent elements: <span>hallo <a href="http://heise.de">heise</a>!</span> -> hallo heise !
    for e in node.children.iter() {
        match e.borrow().node {
            Text(ref t) => text_tokens.push(t.to_string()),

            // if node is not a text node, recurse down
            _ => if let Some(s) = get_text(e) {
                text_tokens.push(s)
            },
        }
    }

    // adjust extracted text: mainly remove white space and some unicode characters
    if text_tokens.len() > 0 {
        Some(String::from(text_tokens.join(" ").trim())
                 .replace("\u{a0}", "")
                 .replace(" >", ">")
                 .replace(" <", "<")
                 .replace(" ::", "::")
                 .replace(":: ", "::")
                 .replace("  ", " "))
    } else {
        None
    }
}

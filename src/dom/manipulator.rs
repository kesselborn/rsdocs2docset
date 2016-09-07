use html5ever;
use html5ever::rcdom::{Handle, RcDom};
use html5ever::tree_builder::{NodeOrText, TreeSink};
use super::Entry;
use super::parser;

pub fn add_dash_links(mut dom: &mut RcDom, entries: &Vec<Entry>) {
    let class_attr = html5ever::Attribute {
        name: qualname!("", "class"),
        value: format_tendril!("dashAnchor"),
    };

    let mut i = 0;

    // https://kapeli.com/docsets#tableofcontents
    // https://kapeli.com/docsets#supportedentrytypes
    for entry in entries {
        if let Some(entryname) = parser::extract_entry_name(&entry) {
            let (handle, entrytype) = match *entry {
                Entry::Const(ref c) => (c, "const"),
                Entry::Enum(ref e) => (e, "enum"),
                Entry::Function(ref f) => (f, "function"),
                Entry::Macro(ref m) => (m, "macro"),
                Entry::Method(ref m) => (m, "method"),
                Entry::Module(ref m) => (m, "module"),
                Entry::Struct(ref s) => (s, "struct"),
                Entry::Trait(ref t) => (t, "trait"),
                Entry::Type(ref t) => (t, "type"),
            };
            let name_attr = html5ever::Attribute {
                name: qualname!("", "name"),
                // TODO: percent escape entryname
                // //dash_ref_SOMEID/TYPE/NAME/IS_SECTION
                value: format_tendril!("//dash_ref_{}/{}/{}/{}", i, entrytype, entryname, "0"),
            };
            let dash_link = dom.create_element(qualname!(html, "a"),
                                               vec![name_attr, class_attr.clone()]);
            let _ = dom.append_before_sibling(handle.clone() as Handle,
                                              NodeOrText::AppendNode(dash_link));

            i = i + 1;
        }
    }

}

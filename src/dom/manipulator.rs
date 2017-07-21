use html5ever;
use html5ever::rcdom::RcDom;
use html5ever::tree_builder::{NodeOrText, TreeSink};
use super::Entry;

pub fn add_dash_links(mut dom: &mut RcDom, entries: &[Option<Entry>]) {
    let class_attr = html5ever::Attribute {
        name: qualname!("", "class"),
        value: format_tendril!("dashAnchor"),
    };

    for entry in entries.iter().filter_map(|x| x.as_ref()) {
        let name_attr = html5ever::Attribute {
            name: qualname!("", "name"),
            value: format_tendril!("{}", entry.anchor_name),
        };
        let dash_link =
            dom.create_element(qualname!(html, "a"), vec![name_attr, class_attr.clone()]);

        if dom.append_before_sibling(entry.handle.clone(), NodeOrText::AppendNode(dash_link))
            .is_err() {
            println!("error inserting dash link")
        }
    }
}

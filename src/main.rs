extern crate html5ever;
#[macro_use(qualname,ns,atom)]
extern crate string_cache;
#[macro_use(format_tendril)]
extern crate tendril;

use html5ever::rcdom::NodeEnum::Element;
use html5ever::rcdom::{RcDom, Handle};
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{TreeBuilderOpts, TreeSink, NodeOrText};
use html5ever::{ParseOpts, parse_document};
use std::string::String;
use std::io;

enum Entry {
    Method(Handle),
}

fn main() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
        ..Default::default()
    };

    let stdin = io::stdin();
    let mut dom = parse_document(RcDom::default(), opts)
                      .from_utf8()
                      .read_from(&mut stdin.lock())
                      .unwrap();

    let entries = find_entry_elements(&mut dom);

    // https://kapeli.com/docsets#tableofcontents
    // https://kapeli.com/docsets#supportedentrytypes
    for entry in entries {
        match entry {
            Entry::Method(e) => {
                add_dash_link_before_entry(&mut dom, &e, "method", "42")
            }
        }
    }

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

fn add_dash_link_before_entry(dom: &mut RcDom, p: &Handle, entrytype: &str, entryname: &str) {
    let class_attr = html5ever::Attribute {
        name: qualname!("", "class"),
        value: format_tendril!("dashAnchor"),
    };

    let name_attr = html5ever::Attribute {
        name: qualname!("", "name"),
        value: format_tendril!("//apple_ref/cpp/{}/{}", entrytype, entryname),
    };

    let dash_link = dom.create_element(qualname!(html, "a"), vec![name_attr, class_attr.clone()]);
    let _ = dom.append_before_sibling(p.clone(), NodeOrText::AppendNode(dash_link));
}

fn find_entry_elements(dom: &mut RcDom) -> Vec<Entry> {
    let mut entries = vec![];
    walk_tree(&dom.document, &mut entries);
    entries
}

fn walk_tree(h: &Handle, entries: &mut Vec<Entry>) {
    let node = h.borrow();
    for e in node.children.iter() {
        if let Element(_, _, ref attrs) = e.borrow().node {
            if let Some(attr) = attrs.iter().find(|ref x| x.name == qualname!("", "class")) {
                match attr.clone().value.to_string().as_str() {
                    "method" => entries.push(Entry::Method(e.clone())),
                    _ => {}
                }
            }
        }
        walk_tree(e, entries);
    }
}

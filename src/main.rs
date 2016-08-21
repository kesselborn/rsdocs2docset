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
use std::io;

enum Section {
    Package(Handle),
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

    let mut sections = vec![];
    walk(&dom.document, &mut sections);

    for section in sections {
        match section {
            Section::Package(p) => {
                let attr = html5ever::Attribute {
                    name: qualname!("", "name"),
                    value: format_tendril!("xxx"),
                };
                let dash_link = dom.create_element(qualname!(html, "a"), vec![attr]);
                dom.append_before_sibling(p, NodeOrText::AppendNode(dash_link));
            }
        }
    }

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

fn walk(h: &Handle, sections: &mut Vec<Section>) {
    let node = h.borrow();
    for e in node.children.iter() {
        walk(e, sections);
        if let Element(ref qualname, _, _) = e.borrow().node {
            if qualname.local == string_cache::Atom::from("script") {
                sections.push(Section::Package(e.clone()));
            }
        }
    }
}

extern crate html5ever;
#[macro_use(qualname,ns,atom)]
extern crate string_cache;
#[macro_use(format_tendril)] extern crate tendril;

use html5ever::{ParseOpts, parse_document};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::rcdom::{RcDom, Handle, Node};
use html5ever::rcdom::NodeEnum::Element;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use std::io;
use html5ever::tree_builder::TreeSink;
use html5ever::tree_builder::interface::NodeOrText;
use tendril::Tendril;

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
    walkx(&dom.document, &mut sections);

    for section in sections {
        let attr = html5ever::Attribute{name: qualname!("", "name"), value: format_tendril!("xxx")};
        let myElement = dom.create_element(qualname!(html, "a"), vec![attr]);
        dom.append(section, NodeOrText::AppendNode(myElement));
    }

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

fn walkx(h :&Handle, sections :&mut Vec<Handle>) {
    let document = h.borrow();
    let html = document.children[0].borrow();
    for e in html.children.iter() {
        if let Element(ref qualname, _, _) = e.borrow().node {
            sections.push(e.clone());
        }
    }
    ;
}

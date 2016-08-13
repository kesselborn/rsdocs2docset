extern crate html5ever;

use html5ever::{ParseOpts, parse_document};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::rcdom::{RcDom, Node};
use html5ever::rcdom::NodeEnum::Element;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use std::io;

fn main() {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let stdin = io::stdin();
    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut stdin.lock())
        .unwrap();

    let document = dom.document.borrow();
    let html = document.children[0].borrow();
    for e in html.children.iter() {
        if let Element(ref qualname, _, _) = e.borrow().node {
            if qualname.local.eq_str_ignore_ascii_case("body") {
                for e in e.borrow().children.iter() {
                    if let Element(_, _, ref mut attributes) = e.borrow_mut().node {
                        if attributes.len() > 0 {
                            attributes[0].value.push_tendril(&From::from("#anchor"));
                        }
                    }
                }
            }
        }
    }

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

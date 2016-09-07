extern crate docsrs2docset;
extern crate html5ever;

use html5ever::driver::{ParseOpts, parse_document};
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::rcdom::RcDom;
use html5ever::tendril::TendrilSink;

use std::io;
use std::string::String;

use docsrs2docset::dom_manipulator;

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

    let entries = dom_manipulator::find_entry_elements(&mut dom);

    dom_manipulator::add_dash_links(&mut dom, &entries);

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

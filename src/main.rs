extern crate docsrs2docset;
extern crate html5ever;

use html5ever::driver::{ParseOpts, parse_document};
use html5ever::rcdom::RcDom;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;

use std::fs::File;
use std::io::Write;
use std::string::String;

use docsrs2docset::dom::{manipulator, parser};

// #### read https://thesquareplanet.com/blog/rust-tips-and-tricks/ Cloning iterators
// #### read https://thesquareplanet.com/blog/rust-tips-and-tricks/ Partial matching

fn main() {
    docset_file_from_rs_doc("index.html".to_string(), "index-out.html".to_string())
}

fn docset_file_from_rs_doc(input: String, output: String) {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
        ..Default::default()
    };

    let mut in_file = match File::open(&input) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", input, e);
            return;
        }
    };

    let mut dom = parse_document(RcDom::default(), opts)
                      .from_utf8()
                      .read_from(&mut in_file)
                      .unwrap();

    let entries = parser::find_entry_elements(&mut dom);
    manipulator::add_dash_links(&mut dom, &entries);

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();

    let mut out_file = match File::create(&output) {
        Ok(f) => f,
        Err(e) => {
            println!("Error creating file {}: {}", output, e);
            return;
        }
    };

    match out_file.write_all(result.as_ref()) {
        Ok(_) => println!("Successfully added docset links from {} -> {}",
                          input,
                          output),
        Err(e) => println!("Error writing {}: {}", output, e),
    }
}

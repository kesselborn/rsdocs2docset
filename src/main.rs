extern crate docsrs2docset;
extern crate html5ever;

use html5ever::driver::{ParseOpts, parse_document};
use html5ever::rcdom::RcDom;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;

use std::fs::{self, DirBuilder, DirEntry, File, ReadDir};
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;
use std::string::String;

use docsrs2docset::dom::{manipulator, parser};

// #### read https://thesquareplanet.com/blog/rust-tips-and-tricks/ Cloning iterators
// #### read https://thesquareplanet.com/blog/rust-tips-and-tricks/ Partial matching

fn main() {
    let out_dir = "out-dir".to_string();

    if let Err(e) = docset_tree_from_rs_doc_tree(Path::new("target/doc"),
                                                 &out_dir,
                                                 &docset_file_from_rs_doc) {
        println!{"error: {}", e}
    }
}

fn docset_tree_from_rs_doc_tree(source_dir: &Path, out_dir: &String, cb: &Fn(&DirEntry, &String))
                                -> io::Result<()> {
    if source_dir.is_dir() {
        for entry in try!(fs::read_dir(source_dir)) {
            let entry = try!(entry);
            let path = entry.path();
            if path.is_dir() {
                try!(docset_tree_from_rs_doc_tree(&path, &out_dir, cb));
            } else {
                cb(&entry, &out_dir);
            }
        }
    }
    Ok(())
}

fn docset_file_from_rs_doc(input: &DirEntry, output_prefix: &String) {
    let out_dir = Path::new(output_prefix).join(input.path());
    let output = out_dir.join(input.file_name());

    DirBuilder::new()
        .recursive(true)
        .create(out_dir)
        .unwrap();


    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
        ..Default::default()
    };

    let mut in_file = match File::open(&input.path()) {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening {}: {}", input.path().display(), e);
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
            println!("Error creating file {}: {}", output.display(), e);
            return;
        }
    };

    match out_file.write_all(result.as_ref()) {
        Ok(_) => println!("Successfully added docset links from {} -> {}",
                          input.path().display(),
                          output.display()),
        Err(e) => println!("Error writing {}: {}", output.display(), e),
    }
}

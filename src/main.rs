// extern crate sqlite;
extern crate html5ever;

#[macro_use(qualname,ns,atom)] extern crate string_cache;
#[macro_use(format_tendril)] extern crate tendril;

use std::io::{self, Write};
use std::default::Default;
use std::iter::{repeat};

use tendril::TendrilSink;

use html5ever::driver::ParseOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{parse_document, serialize};
use html5ever::rcdom::{Document, Doctype, Text, Comment, Element, RcDom, Handle};
use html5ever::tree_builder::TreeSink;


fn main() {
    // let connection = sqlite::open("docSet.dsidx").unwrap();

    // connection.execute("CREATE TABLE searchIndex(id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT);").unwrap();
    // connection.execute("CREATE UNIQUE INDEX anchor ON searchIndex (name, type, path);").unwrap();
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts { drop_doctype: false, ..Default::default() },
        ..Default::default()
    };
    let stdin = io::stdin();
    let mut dom = parse_document(RcDom::default(), opts)
                  .from_utf8()
                  .read_from(&mut stdin.lock())
                  .unwrap();

    walk(0, dom.document);

    let foo = dom.create_element(qualname!(html, "script"), vec!());
    let mut bar = dom.create_comment(format_tendril!("foooooooooooooooooooo"));
    //dom.append(dom.document, bar);

     //The validator.nu HTML2HTML always prints a doctype at the very beginning.
    serialize(&mut io::stdout(), &dom.document, Default::default())
        .ok()
        .expect("serialization failed");
}

fn walk(indent: usize, handle: Handle) {
    let node = handle.borrow();
    //handle.create_comment(
    // FIXME: don't allocate
    //print!("{}", repeat(" ").take(indent).collect::<String>());
    match node.node {
        //Document
        //    => println!("#Document"),

        //Doctype(ref name, ref public, ref system)
        //    => println!("<!DOCTYPE {} \"{}\" \"{}\">", *name, *public, *system),

        //Text(ref text)
        //    => println!("#text: {}", escape_default(text)),

        //Comment(ref text)
        //    => println!("<!-- {} -->", escape_default(text)),

        Element(ref name, ref e, ref attrs) => {
            //name = name;
            //assert!(name.ns == ns!(html));
            //serialize(&mut io::stdout(), &dom.document, Default::default()).ok()
            //print!("<{}", name.local);
            //for attr in attrs.iter() {
            //    //assert!(attr.name.ns == ns!());
            //    print!(" {}=\"{}\"", attr.name.local, attr.value);
            //}
            //println!(">");
        }

        _ => {
            serialize(&mut io::stdout(), &handle, Default::default()).ok();
        },
    }


    for child in node.children.iter() {
        walk(indent+4, child.clone());
    }
}

// FIXME: Copy of str::escape_default from std, which is currently unstable
pub fn escape_default(s: &str) -> String {
    s.chars().flat_map(|c| c.escape_default()).collect()
}

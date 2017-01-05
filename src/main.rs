extern crate rsdocs2docset;
extern crate html5ever;
#[macro_use]
extern crate quick_error;
extern crate clap;
extern crate url;
extern crate sqlite;

use clap::{Arg, App};
use html5ever::driver::{ParseOpts, parse_document};
use html5ever::rcdom::RcDom;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;

use std::ffi::OsStr;
use std::fs::{self, DirBuilder, DirEntry, File};
use std::io::{self,Write};
use std::path::Path;
use std::string::String;

use rsdocs2docset::dom::{manipulator, parser};

type Result<T> = std::result::Result<T, RsDoc2DocsetError>;

static DB_PATH: &'static str = "/tmp/foo.dsidx";

// #### read https://thesquareplanet.com/blog/rust-tips-and-tricks/ Cloning iterators
// #### read https://thesquareplanet.com/blog/rust-tips-and-tricks/ Partial matching

quick_error! {
    #[derive(Debug)]
    pub enum RsDoc2DocsetError {
        Io(err: std::io::Error) {
            from()
                cause(err)
                description(err.description())
        }
        Utf8(err: std::string::FromUtf8Error) {
            from()
                cause(err)
                description(err.description())
        }
    }
}

fn main() {
    let args = App::new("RsDoc2Docset")
        .version("0.0.1")
        .about("A tool that converts rust docs to Dash docset files")
        .arg(Arg::with_name("indir")
             .short("i")
             .long("rsdoc")
             .value_name("INDIR")
             .help("directory that contains rustdoc files")
             .required(true)
             .takes_value(true))
        .arg(Arg::with_name("name")
             .short("n")
             .long("name")
             .value_name("NAME")
             .help("name of the docset")
             .required(true)
             .takes_value(true))
        .get_matches();

    create_db();

    if let Err(e) = docset_from_rs_doc_tree(Path::new(args.value_of("indir").unwrap()),
    format!("{}.docset/Contents/Resources/Documents/", &args.value_of("name").unwrap()).as_str(),
	&annotate_file) {
		println!{"error: {}", e}
	}

	let info_plist = format!(include_str!("Info.plist.tmpl"), name = args.value_of("name").unwrap(), identifier = String::from(args.value_of("name").unwrap()).replace(" ", "-").to_lowercase());

	let plist_path_str = format!("{}.docset/Contents/Info.plist", &args.value_of("name").unwrap());
    let plist_path = Path::new(&plist_path_str);
	if let Err(e) = File::create(plist_path).and_then(|mut x| x.write_all(info_plist.as_ref())) {
		println!{"error: {}", e}
	}

    let icon = include_bytes!("icon.png");
	let icon_path_str = format!("{}.docset/icon.png", &args.value_of("name").unwrap());
    let icon_path = Path::new(&icon_path_str);
	if let Err(e) = File::create(icon_path).and_then(|mut x| x.write_all(icon)) {
		println!{"error: {}", e}
	}

    let icon_2x = include_bytes!("icon@2x.png");
	let icon_2x_path_str = format!("{}.docset/icon_2x_.png", &args.value_of("name").unwrap());
    let icon_2x_path = Path::new(&icon_2x_path_str);
	if let Err(e) = File::create(icon_2x_path).and_then(|mut x| x.write_all(icon_2x)) {
		println!{"error: {}", e}
	}

	let db_path_str = format!("{}.docset/Contents/Resources/docSet.dsidx", &args.value_of("name").unwrap());
    if let Err(e) = fs::rename(DB_PATH, db_path_str) {
        panic!("error moving db: {}", e)
    }
}

fn create_db() {
    if let Err(e) = sqlite::open(Path::new(DB_PATH))
        .and_then(|c| c.execute("CREATE TABLE searchIndex(id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT);")) {
        panic!("error creating db: {}", e);
    }
}

fn docset_from_rs_doc_tree(source_dir: &Path, out_dir: &str,
                           annotate_file: &Fn(&DirEntry, &str) -> Result<()>)
    -> Result<()> {
        if !source_dir.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("{} does not exist", source_dir.to_str().unwrap())).into());
        }

        if source_dir.is_dir() {
            for entry in fs::read_dir(source_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    try!(docset_from_rs_doc_tree(&entry.path(), &out_dir, annotate_file));
                } else {
                    try!(annotate_file(&entry, &out_dir));
                }
            }
        }
        Ok(())
    }

fn annotate_file(in_file: &DirEntry, output_prefix: &str) -> Result<()> {
    let out_dir = Path::new(output_prefix).join(in_file.path().parent().unwrap());
    let out_file = out_dir.join(in_file.file_name());

    try!(DirBuilder::new()
         .recursive(true)
         .create(out_dir));

    if in_file.path().extension() != Some(OsStr::new("html")) {
        try!(fs::copy(in_file.path(), &out_file));
        println!("{:70} | ", in_file.path().display());
        //print!(".")
    } else {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
            ..Default::default()
        };

        let mut dom = parse_document(RcDom::default(), opts)
            .from_utf8()
            .read_from( &mut File::open(&in_file.path())? )?;

        let entries = parser::find_entry_elements(&mut dom);
        for entry in entries.iter().filter_map(|x| x.as_ref()) {
            print!("{:70} | ", in_file.path().display());
            println!("{}", entry);

            let sql_cmd = format!("INSERT OR IGNORE INTO searchIndex(name, type, path) VALUES (\"{}\", \"{}\", \"{}#{}\");", entry.entry_name, entry.entry_type, in_file.path().to_str().unwrap(), entry.anchor_name);
            if let Err(e) = sqlite::open(Path::new(DB_PATH))
                .and_then(|c| c.execute(&sql_cmd)) {
                    panic!("error executing {}: {}", &sql_cmd, e);
                }

        }
        manipulator::add_dash_links(&mut dom, &entries);

        let mut bytes = vec![];
        try!(serialize(&mut bytes, &dom.document, SerializeOpts::default()));

        let result = String::from_utf8(bytes)?;

        File::create(&out_file)?.write_all(result.as_ref())?;

        //print!(".")
    }

    Ok(())
}

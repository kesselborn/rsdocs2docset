#[macro_use] extern crate quick_error;

extern crate clap;
extern crate html5ever;
extern crate rsdocs2docset;
extern crate sqlite;
extern crate url;

use clap::{Arg, App};

use html5ever::driver::{ParseOpts, parse_document};
use html5ever::rcdom::RcDom;
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;

use std::ffi::OsStr;
use std::fs::{self, DirBuilder, DirEntry, File};
use std::io::{self,Write};
use std::path::Path;

use rsdocs2docset::dom::{manipulator, parser};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

type Result = std::result::Result<(), RsDoc2DocsetError>;

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
        Sqlite(err: sqlite::Error) {
            from()
                cause(err)
                description(err.description())
        }
    }
}

fn main() {
    let args = App::new("RsDoc2Docset")
        .version(VERSION)
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

    create_docset(args.value_of("indir").unwrap(), args.value_of("name").unwrap()).expect("error creating docset");
    println!("\n{}.docset successfully created!", args.value_of("name").unwrap())
}

fn create_docset(indir: &str, name: &str) -> Result {
    let db_filename = format!("{}.docset/Contents/Resources/docSet.dsidx", name);
    let db_path = Path::new(&db_filename);

    try!(write_file(Path::new(format!("{}.docset/Contents/Info.plist", name).as_str()), format!(include_str!("Info.plist.tmpl"), name = name, identifier = name.replace(" ", "-").to_lowercase()).as_ref()));
    try!(write_file(Path::new(format!("{}.docset/icon.png", name).as_str()), include_bytes!("icon.png")));
    try!(write_file(Path::new(format!("{}.docset/icon@2x.png", name).as_str()), include_bytes!("icon@2x.png")));

    try!(mkdir_parent_p(db_path));
    try!(sqlite::open(db_path).and_then(|c| c.execute("CREATE TABLE searchIndex(id INTEGER PRIMARY KEY, name TEXT, type TEXT, path TEXT);")));

    try!(docset_from_rs_doc_tree(&Path::new(indir), format!("{}.docset/Contents/Resources/Documents/", name).as_str(), Path::new(db_path)));

    Ok(())
}

// recursivly creates the parent dir of `path`
fn mkdir_parent_p(path: &Path) -> Result {
    let dir = path.parent().unwrap();

    try!(DirBuilder::new()
         .recursive(true)
         .create(dir));
    Ok(())
}

fn write_file(path: &Path, data: &[u8]) -> Result {
    try!(mkdir_parent_p(path));
    try!(File::create(path).and_then(|mut f| f.write_all(data)));
    Ok(())
}

fn docset_from_rs_doc_tree(source_dir: &Path, out_dir: &str, db_path: &Path) -> Result {
        if !source_dir.exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("{} does not exist", source_dir.to_str().unwrap())).into());
        }

        if source_dir.is_dir() {
            for entry in fs::read_dir(source_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    try!(docset_from_rs_doc_tree(&entry.path(), &out_dir, &db_path));
                } else {
                    try!(annotate_file(&entry, &out_dir, &db_path));
                }
            }
        }
        Ok(())
    }

fn annotate_file(in_file: &DirEntry, output_prefix: &str, db_path: &Path) -> Result {
    let out_file = Path::new(output_prefix).join(in_file.path());

    if in_file.path().extension() != Some(OsStr::new("html")) {
        try!(mkdir_parent_p(&out_file));
        try!(fs::copy(in_file.path(), &out_file));
        //println!("{:70} | ", in_file.path().display());
        print!("."); io::stdout().flush().unwrap();
    } else {
        let mut dom = parse_document(RcDom::default(), ParseOpts::default())
            .from_utf8()
            .read_from( &mut File::open(&in_file.path())? )?;

        let entries = parser::find_entry_elements(&mut dom);
        for entry in entries.iter().filter_map(|x| x.as_ref()).filter(|x| x.is_section == false) {
            //println!("{:70} | {}", in_file.path().display(), entry);

            let sql_cmd = format!("INSERT OR IGNORE INTO searchIndex(name, type, path) VALUES (\"{}\", \"{}\", \"{}#{}\");", entry.entry_name, entry.entry_type, in_file.path().to_str().unwrap(), entry.anchor_name);
            try!(sqlite::open(db_path).and_then(|c| c.execute(&sql_cmd)))
        }
        manipulator::add_dash_links(&mut dom, &entries);

        let mut bytes = vec![];
        try!(serialize(&mut bytes, &dom.document, SerializeOpts::default()));
        try!(write_file(&out_file, bytes.as_ref()));

        print!("o"); io::stdout().flush().unwrap();
    }

    Ok(())
}

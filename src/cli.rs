use clap::{App, Arg};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn build_cli() -> App<'static, 'static> {
    App::new("RsDoc2Docset")
        .version(VERSION)
        .about("A tool that converts rust docs to Dash docset files")
        .arg(Arg::with_name("bash-completion-code")
                 .long("bash-completion-code")
                 .help("create bash completion code"))
        .arg(Arg::with_name("indir")
                 .short("i")
                 .long("rsdoc")
                 .value_name("INDIR")
                 .help("directory that contains rustdoc files")
                 .takes_value(true))
        .arg(Arg::with_name("name")
                 .short("n")
                 .long("name")
                 .value_name("NAME")
                 .help("name of the docset")
                 .takes_value(true))
}

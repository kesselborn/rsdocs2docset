extern crate html5ever;
#[macro_use(qualname,ns,atom)]
extern crate string_cache;
#[macro_use(format_tendril)]
extern crate tendril;

use html5ever::rcdom::NodeEnum::{Element, Text};
use html5ever::rcdom::{Handle, RcDom};
use html5ever::serialize::{SerializeOpts, serialize};
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::{NodeOrText, TreeBuilderOpts, TreeSink};
use html5ever::{ParseOpts, parse_document};

use std::io;
use std::string::String;

enum Entry {
    Const(Handle),
    Enum(Handle),
    Function(Handle),
    Macro(Handle),
    Method(Handle),
    Module(Handle),
    Struct(Handle),
    Trait(Handle),
    Type(Handle),
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

    let entries = find_entry_elements(&mut dom);

    // https://kapeli.com/docsets#tableofcontents
    // https://kapeli.com/docsets#supportedentrytypes
    for entry in entries {
        match entry {
            Entry::Method(e) => {
                add_dash_link_before_entry(&mut dom, &e, "method", "42")
            }
            _ => {}
        }
    }

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

fn add_dash_link_before_entry(dom: &mut RcDom, p: &Handle, entrytype: &str, entryname: &str) {
    let class_attr = html5ever::Attribute {
        name: qualname!("", "class"),
        value: format_tendril!("dashAnchor"),
    };

    let name_attr = html5ever::Attribute {
        name: qualname!("", "name"),
        // TODO: percent escape entryname
        value: format_tendril!("//apple_ref/cpp/{}/{}", entrytype, entryname),
    };

    let dash_link = dom.create_element(qualname!(html, "a"), vec![name_attr, class_attr.clone()]);
    let _ = dom.append_before_sibling(p.clone(), NodeOrText::AppendNode(dash_link));
}

fn find_entry_elements(dom: &mut RcDom) -> Vec<Entry> {
    let mut entries = vec![];
    walk_tree(&dom.document, &mut entries);
    entries
}

fn walk_tree(h: &Handle, entries: &mut Vec<Entry>) {
    let node = h.borrow();
    for e in node.children.iter() {
        if let Element(ref name, _, ref attrs) = e.borrow().node {
            let tag = &(*name.local.to_ascii_lowercase());
            if let Some(attr) = attrs.iter().find(|ref x| x.name == qualname!("", "class")) {
                match (tag, attr.clone().value.to_string().as_str()) {
                    ("h4", "method") => entries.push(Entry::Method(e.clone())),
                    ("section", "content constant") => entries.push(Entry::Const(e.clone())),
                    ("section", "content enum") => entries.push(Entry::Enum(e.clone())),
                    ("section", "content fn") => entries.push(Entry::Function(e.clone())),
                    ("section", "content macro") => entries.push(Entry::Macro(e.clone())),
                    ("section", "content mod") => entries.push(Entry::Module(e.clone())),
                    ("section", "content struct") => entries.push(Entry::Struct(e.clone())),
                    ("section", "content trait") => entries.push(Entry::Trait(e.clone())),
                    ("section", "type") => entries.push(Entry::Type(e.clone())),
                    (_, _) => {}
                }
            }
        }
        walk_tree(e, entries);
    }
}

fn find_element_with_class(h: &Handle, c: &str) -> Option<Handle> {
    let node = h.borrow();
    for e in node.children.iter() {
        if let Element(_, _, ref attrs) = e.borrow().node {
            if let Some(attr) = attrs.iter().find(|ref x| x.name == qualname!("", "class")) {
                if attr.clone().value.to_string() == c {
                    return Some(e.clone());
                }
            }
        }
        if let Some(e) = find_element_with_class(e, c) {
            return Some(e);
        }
    }
    None
}

fn get_text(h: &Handle) -> Option<String> {
    let node = h.borrow();
    for e in node.children.iter() {
        if let Text(ref t) = e.borrow().node {
            return Some(t.to_string());
        }
    }

    None
}

fn extract_entry_name(e: &Entry) -> Option<String> {
    match *e {
        Entry::Const(ref c) => {
            if let Some(e) = find_element_with_class(c, "constant") {
                return get_text(&e);
            }
        }
        Entry::Enum(ref e) => {
            if let Some(e) = find_element_with_class(e, "enum") {
                return get_text(&e);
            }
        }
        Entry::Function(ref e) => {
            if let Some(e) = find_element_with_class(e, "fn") {
                return get_text(&e);
            }
        }
        Entry::Method(ref e) => {
            if let Some(e) = find_element_with_class(e, "fnname") {
                return get_text(&e);
            }
        }
        Entry::Macro(ref e) => {
            if let Some(e) = find_element_with_class(e, "macro") {
                return get_text(&e);
            }
        }
        Entry::Module(ref e) => {
            if let Some(e) = find_element_with_class(e, "mod") {
                return get_text(&e);
            }
        }
        _ => (),
    }
    None
}

#[cfg(test)]
mod tests {
    use html5ever::{ParseOpts, parse_document};
    use html5ever::rcdom::{Handle, RcDom};
    use tendril::TendrilSink;
    use html5ever::tree_builder::TreeBuilderOpts;

    fn dom_from_snippet(s: &str) -> Handle {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
            ..Default::default()
        };

        let dom = parse_document(RcDom::default(), opts)
                      .from_utf8()
                      .read_from(&mut format!("<html><head></head><body>{}</body></html>", s)
                                          .as_bytes())
                      .unwrap();

        dom.document
    }

    #[test]
    fn it_extracts_name_for_const_correctly() {
        // alloc::boxed::HEAP constant, entry name: HEAP
        let document_with_constant_section = dom_from_snippet(r##"
          <section id="main" class="content constant">
            <h1 class="fqn">
              <span class="in-band">
                <a href="../index.html">alloc</a>::<wbr><a href="index.html">boxed</a>::<wbr><a class="constant" href="#">HEAP</a>
              </span>
              <span class="out-of-band">
                <span class="since" title="Stable since Rust version "></span>
                <span id="render-detail">
                  <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a>
                </span>
                <a id="src-86" class="srclink" href="../../src/alloc/up/src/liballoc/boxed.rs.html#91" title="goto source code">[src]</a>
              </span>
            </h1>
          </section>
        "##);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&document_with_constant_section, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Const(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"HEAP".to_string()),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_extracts_name_for_enum_correctly() {
        // collections::borrow::Cow, entry name: Cow
        let document_with_enum_section = dom_from_snippet(r##"
          <section id="main" class="content enum">
            <h1 class="fqn">
              <span class="in-band">Enum <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="enum" href="#">Cow</a></span>
              <span class="out-of-band">
                <span class="since" title="Stable since Rust version 1.0.0">1.0.0</span>
                <span id="render-detail">
                  <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a> 
                </span>
                <a id="src-2309" class="srclink" href="../../src/collections/up/src/libcollections/borrow.rs.html#106-118" title="goto source code">[src]</a>
              </span>
            </h1>
          </section>
        "##);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&document_with_enum_section, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Enum(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"Cow".to_string()),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_extracts_name_for_function_correctly() {
        // Function std::fs::metadata, entry name: metadata
        let document_with_function_section = dom_from_snippet(r##"
          <section id="main" class="content fn">
            <h1 class="fqn">
              <span class="in-band">Function <a href="../index.html">std</a>::<wbr><a href="index-2.html">fs</a>::<wbr><a class="fn" href="#">metadata</a></span>
              <span class="out-of-band">
                <span class="since" title="Stable since Rust version 1.0.0">1.0.0</span>
                <span id="render-detail">
                  <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a>
                </span>
                <a id="src-3833" class="srclink" href="../../src/std/up/src/libstd/fs.rs.html#933-935" title="goto source code">[src]</a>
              </span>
            </h1>
          </section>
        "##);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&document_with_function_section, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Function(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"metadata".to_string()),
            None => assert_eq!(true, false),
        }
    }


    #[test]
    fn it_extracts_name_for_macro_correctly() {
        // Macro std::println!, entry name: println!
        let document_with_macro_section = dom_from_snippet(r##"
          <section id="main" class="content macro">
            <h1 class="fqn">
              <span class="in-band"><a href="index-2.html">std</a>::<wbr><a class="macro" href="#">println!</a></span>
              <span class="out-of-band">
                <span class="since" title="Stable since Rust version 1.0.0">1.0.0</span>
                <span id="render-detail"> <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a> </span>
                <a id="src-15389" class="srclink" href="../src/std/up/src/libstd/macros.rs.html#118-121" title="goto source code">[src]</a>
              </span>
            </h1>
          </section>
        "##);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&document_with_macro_section, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Macro(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"println!".to_string()),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_extracts_name_for_method_correctly() {
        // Method Hash::hash, entry name: hash
        let document_with_method_section = dom_from_snippet(r##"
          <h4 id="method.hash" class="method">
            <code>fn <a href="../../core/hash/trait.Hash.html#tymethod.hash" class="fnname">hash</a>&lt;H:&nbsp;<a class="trait" href="../../core/hash/trait.Hasher.html" title="core::hash::Hasher">Hasher</a>&gt;(&amp;self, state: &amp;mut H)</code>
            <a href="javascript:void(0)" class="collapse-toggle">[<span class="inner">−</span>]</a>
          </h4>
        "##);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&document_with_method_section, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Method(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"hash".to_string()),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_extracts_name_for_module_correctly() {
        // Module/Crate Collections, entry name: collections
        let document_with_module_section = dom_from_snippet(r##"
          <section id="main" class="content mod">
            <h1 class="fqn">
              <span class="in-band">Crate <a class="mod" href="#">collections</a></span>
              <span class="out-of-band">
                <span class="since" title="Stable since Rust version "></span>
                <span id="render-detail"> <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a> </span>
                <a id="src-0" class="srclink" href="../src/collections/up/src/libcollections/lib.rs.html#11-140" title="goto source code">[src]</a>
              </span>
            </h1>
          </section>
        "##);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&document_with_module_section, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Module(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"collections".to_string()),
            None => assert_eq!(true, false),
        }
    }




// struct: <section id="main" class="content struct"> <h1 class="fqn"><span class="in-band">Struct <a href="../index.html">std</a>::<wbr><a href="index-2.html">io</a>::<wbr><a class="struct" href="#">Bytes</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail"> <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a> </span><a id="src-5013" class="srclink" href="../../src/std/up/src/libstd/io/mod.rs.html#1532-1534" title="goto source code">[src]</a></span></h1></section>

// trait: <section id="main" class="content trait"> <h1 class="fqn"><span class="in-band">Trait <a href="../index.html">collections</a>::<wbr><a href="index-2.html">fmt</a>::<wbr><a class="trait" href="#">Binary</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail"> <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a> </span><a id="src-38600" class="srclink" href="../../core/fmt/trait.Binary74db.html?gotosrc=38600" title="goto source code">[src]</a></span></h1></section>

// type: <h4 id="associatedtype.Output" class="type"><code>type <a href="../std/ops/trait.Not.html#associatedtype.Output" class="type">Output</a> = <a class="primitive" href="primitive.bool.html">bool</a></code></h4>
}

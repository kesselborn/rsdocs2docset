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

    add_dash_links(&mut dom, &entries);

    let mut bytes = vec![];
    serialize(&mut bytes, &dom.document, SerializeOpts::default()).unwrap();
    let result = String::from_utf8(bytes).unwrap();
    println!("{}", result);
}

fn add_dash_links(mut dom: &mut RcDom, entries: &Vec<Entry>) {
    // https://kapeli.com/docsets#tableofcontents
    // https://kapeli.com/docsets#supportedentrytypes
    for entry in entries {
        match *entry {
            Entry::Method(ref e) => {
                add_dash_link_before_entry(&mut dom, &e, "method", "42")
            }
            _ => {}
        }
    }
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
    for e in h.borrow().children.iter() {
        if let Element(ref name, _, ref attrs) = e.borrow().node {
            let tag = &(*name.local.to_ascii_lowercase());
            if let Some(class_attr) = attrs.iter()
                                           .find(|ref x| x.name == qualname!("", "class"))
                                           .and_then(|c| Some(c.clone().value.to_string())) {

                match (tag, class_attr.as_str()) {
                    ("h4", "method") => entries.push(Entry::Method(e.clone())),
                    ("h4", "type") => entries.push(Entry::Type(e.clone())),
                    ("section", "content constant") => entries.push(Entry::Const(e.clone())),
                    ("section", "content enum") => entries.push(Entry::Enum(e.clone())),
                    ("section", "content fn") => entries.push(Entry::Function(e.clone())),
                    ("section", "content macro") => entries.push(Entry::Macro(e.clone())),
                    ("section", "content mod") => entries.push(Entry::Module(e.clone())),
                    ("section", "content struct") => entries.push(Entry::Struct(e.clone())),
                    ("section", "content trait") => entries.push(Entry::Trait(e.clone())),
                    (_, _) => {}
                }
            }
        }
        walk_tree(e, entries);
    }
}

fn find_element_with_class(h: &Handle, class_value: &str) -> Option<Handle> {
    for e in h.borrow().children.iter() {
        if let Element(_, _, ref attrs) = e.borrow().node {
            if attrs.iter()
                    .find(|ref attr| attr.name == qualname!("", "class"))
                    .and_then(|attr| Some(attr.value.to_string() == class_value))
                    .unwrap_or(false) {
                return Some(e.clone());
            }
        }
        if let Some(e) = find_element_with_class(e, class_value) {
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
    let name_element = match *e {
        Entry::Const(ref c) => find_element_with_class(c, "constant"),
        Entry::Enum(ref e) => find_element_with_class(e, "enum"),
        Entry::Function(ref e) => find_element_with_class(e, "fn"),
        Entry::Method(ref e) => find_element_with_class(e, "fnname"),
        Entry::Macro(ref e) => find_element_with_class(e, "macro"),
        Entry::Module(ref e) => find_element_with_class(e, "mod"),
        Entry::Struct(ref e) => find_element_with_class(e, "struct"),
        Entry::Trait(ref e) => find_element_with_class(e, "trait"),
        Entry::Type(ref e) => find_element_with_class(e, "type"),
    };

    match name_element {
        Some(element) => get_text(&element),
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use html5ever::{ParseOpts, parse_document};
    use html5ever::rcdom::RcDom;
    use tendril::TendrilSink;
    use html5ever::tree_builder::TreeBuilderOpts;

    fn dom_from_snippet(s: &str) -> RcDom {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
            ..Default::default()
        };

        let dom = parse_document(RcDom::default(), opts)
                      .from_utf8()
                      .read_from(&mut format!("<html><head></head><body>{}</body></html>", s)
                                          .as_bytes())
                      .unwrap();

        dom
    }

    #[test]
    fn it_extracts_name_for_const_correctly() {
        let dom_with_constant_section = dom_from_snippet(CONST_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_constant_section.document, &mut entries);

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
    fn it_inserts_dash_anchor_befor_const_entry() {
        let dom_with_constant_section = dom_from_snippet(CONST_SNIPPET);
        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_constant_section.document, &mut entries);

    }

    #[test]
    fn it_extracts_name_for_enum_correctly() {
        let dom_with_enum_section = dom_from_snippet(ENUM_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_enum_section.document, &mut entries);

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
        let dom_with_function_section = dom_from_snippet(FUNCTION_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_function_section.document, &mut entries);

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
        let dom_with_macro_section = dom_from_snippet(MACRO_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_macro_section.document, &mut entries);

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
        let dom_with_method_section = dom_from_snippet(METHOD_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_method_section.document, &mut entries);

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
        let dom_with_module_section = dom_from_snippet(MODULE_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_module_section.document, &mut entries);

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

    #[test]
    fn it_extracts_name_for_struct_correctly() {
        let dom_with_struct_section = dom_from_snippet(STRUCT_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_struct_section.document, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Struct(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"Bytes".to_string()),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_extracts_name_for_trait_correctly() {
        let dom_with_trait_section = dom_from_snippet(TRAIT_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_trait_section.document, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Trait(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"Binary".to_string()),
            None => assert_eq!(true, false),
        }
    }

    #[test]
    fn it_extracts_name_for_type_correctly() {
        let dom_with_type_section = dom_from_snippet(TYPE_SNIPPET);

        let mut entries: Vec<super::Entry> = Vec::new();
        super::walk_tree(&dom_with_type_section.document, &mut entries);

        assert_eq!(entries.len(), 1);
        match entries[0] {
            super::Entry::Type(_) => assert!(true),
            _ => assert!(false),
        }

        match super::extract_entry_name(&entries[0]) {
            Some(s) => assert_eq!(s, *"Output".to_string()),
            None => assert_eq!(true, false),
        }
    }

    /// /////////////////////// fixtures
    /// alloc::boxed::HEAP constant, entry name: HEAP
    static CONST_SNIPPET: &str = r##"
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
    "##;

    // collections::borrow::Cow, entry name: Cow
    static ENUM_SNIPPET: &str = r##"
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
    "##;

    // Function std::fs::metadata, entry name: metadata
    static FUNCTION_SNIPPET: &str = r##"
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
    "##;

    // Macro std::println!, entry name: println!
    static MACRO_SNIPPET: &str = r##"
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
    "##;

    // Method Hash::hash, entry name: hash
    static METHOD_SNIPPET: &str = r##"
      <h4 id="method.hash" class="method">
        <code>fn <a href="../../core/hash/trait.Hash.html#tymethod.hash" class="fnname">hash</a>&lt;H:&nbsp;<a class="trait" href="../../core/hash/trait.Hasher.html" title="core::hash::Hasher">Hasher</a>&gt;(&amp;self, state: &amp;mut H)</code>
        <a href="javascript:void(0)" class="collapse-toggle">[<span class="inner">−</span>]</a>
      </h4>
    "##;

    // Module/Crate Collections, entry name: collections
    static MODULE_SNIPPET: &str = r##"
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
    "##;

    // Struct collections::str::Bytes,  entry name: Bytes
    static STRUCT_SNIPPET: &str = r##"
      <section id="main" class="content struct">
        <h1 class="fqn">
          <span class="in-band">Struct <a href="../index.html">std</a>::<wbr><a href="index-2.html">io</a>::<wbr><a class="struct" href="#">Bytes</a></span>
          <span class="out-of-band">
            <span class="since" title="Stable since Rust version 1.0.0">1.0.0</span>
            <span id="render-detail">
              <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a>
            </span>
            <a id="src-5013" class="srclink" href="../../src/std/up/src/libstd/io/mod.rs.html#1532-1534" title="goto source code">[src]</a>
          </span>
        </h1>
      </section>
    "##;

    // Trait collections::fmt::Binary,  entry name: Binary
    static TRAIT_SNIPPET: &str = r##"
      <section id="main" class="content trait">
        <h1 class="fqn">
          <span class="in-band">Trait <a href="../index.html">collections</a>::<wbr><a href="index-2.html">fmt</a>::<wbr><a class="trait" href="#">Binary</a></span>
          <span class="out-of-band">
            <span class="since" title="Stable since Rust version 1.0.0">1.0.0</span>
            <span id="render-detail"> <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a> </span>
            <a id="src-38600" class="srclink" href="../../core/fmt/trait.Binary74db.html?gotosrc=38600" title="goto source code">[src]</a>
          </span>
        </h1>
      </section>
    "##;

    // Type Trait collections::fmt::Binary / Output, entry name: Output
    static TYPE_SNIPPET: &str = r##"
      <h4 id="associatedtype.Output" class="type">
        <code>type <a href="../std/ops/trait.Not.html#associatedtype.Output" class="type">Output</a> = <a class="primitive" href="primitive.bool.html">bool</a></code>
      </h4>
    "##;
}

use html5ever::rcdom::RcDom;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::{ParseOpts, parse_document};
use tendril::TendrilSink;

pub fn dom_from_snippet(s: &str) -> RcDom {
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


/// alloc::boxed::HEAP constant, entry name: HEAP
pub static CONST_SNIPPET: &str = r##"
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
pub static ENUM_SNIPPET: &str = r##"
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
pub static FUNCTION_SNIPPET: &str = r##"
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
pub static MACRO_SNIPPET: &str = r##"
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
pub static METHOD_SNIPPET: &str = r##"
  <h4 id="method.hash" class="method">
    <code>fn <a href="../../core/hash/trait.Hash.html#tymethod.hash" class="fnname">hash</a>&lt;H:&nbsp;<a class="trait" href="../../core/hash/trait.Hasher.html" title="core::hash::Hasher">Hasher</a>&gt;(&amp;self, state: &amp;mut H)</code>
    <a href="javascript:void(0)" class="collapse-toggle">[<span class="inner">−</span>]</a>
  </h4>
"##;

// Module/Crate Collections, entry name: collections
pub static MODULE_SNIPPET: &str = r##"
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
pub static STRUCT_SNIPPET: &str = r##"
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
pub static TRAIT_SNIPPET: &str = r##"
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
pub static TYPE_SNIPPET: &str = r##"
  <h4 id="associatedtype.Output" class="type">
    <code>type <a href="../std/ops/trait.Not.html#associatedtype.Output" class="type">Output</a> = <a class="primitive" href="primitive.bool.html">bool</a></code>
  </h4>
"##;

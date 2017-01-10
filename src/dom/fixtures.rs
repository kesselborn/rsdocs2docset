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

/// alloc::arc::Arc impl Arc, entry name: impl<T> Arc<T>
pub static IMPL_SNIPPET: &'static str = r##"
<h3 class="impl">
    <span class="in-band">
        <code>impl&lt;T:&nbsp;<a class="trait" href="../../core/clone/trait.Clone.html" title="core::clone::Clone">Clone</a>&gt; <a class="struct" href="../../alloc/arc/struct.Arc.html" title="alloc::arc::Arc">Arc</a>&lt;T&gt;</code>
    </span>
    <span class="out-of-band">
        <div class="ghost"></div>
        <a id="src-413" class="srclink" href="../../src/alloc/up/src/liballoc/arc.rs.html#487-571" title="goto source code">[src]</a>
    </span>
</h3>
"##;


/// alloc::boxed::HEAP constant
pub static CONST_SNIPPET: &'static str = r##"
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

// collections::borrow::Cow
pub static ENUM_SNIPPET: &'static str = r##"
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

// Function std::fs::metadata
pub static FUNCTION_SNIPPET: &'static str = r##"
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

// Macro std::println
pub static MACRO_SNIPPET: &'static str = r##"
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

pub static TRAIT_METHOD_SNIPPET: &'static str = r##"
<html>
<body class="rustdoc">
  <section class="content trait" id="main">
    <h1 class="fqn"><span class="in-band">Trait <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="trait" href=
    "#">Borrow</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span> <span id="render-detail"><a href=
    "javascript:void(0)" id="toggle-all-docs" title="collapse all docs">[<span class="inner">−</span>]</a></span> <a class="srclink" href=
    "../../core/borrow/trait.Borrowb39b.html?gotosrc=2749" id="src-2749" title="goto source code">[src]</a></span></h1>
    <pre class="rust trait">pub trait Borrow&lt;Borrowed&gt; <span class="where">where Borrowed: ?<a class="trait" href="../../core/marker/trait.Sized.html"
    title="core::marker::Sized">Sized</a></span> {
    fn <a class="fnname" href="#tymethod.borrow">borrow</a>(&amp;self) -&gt; &amp;Borrowed;
}</pre><a class="dashAnchor" id="//dash_ref_589/Method/Required%20Methods/1" name="//dash_ref_589/Method/Required%20Methods/1"></a>
    <h2 id="required-methods">Required Methods</h2>
    <div class="methods">
      <a class="dashAnchor" id="//dash_ref_588/Method/borrow/0" name="//dash_ref_588/Method/borrow/0"></a>
      <h3 class="method stab" id="tymethod.borrow"><span class="invisible" id="borrow.v"><code>fn <a class="fnname" href=
      "#tymethod.borrow">borrow</a>(&amp;self) -&gt; &amp;Borrowed</code></span><a class="collapse-toggle" href="javascript:void(0)">[<span class=
      "inner">−</span>]</a></h3>
    </div>
  </section>
</body>
</html>
"##;

pub static ENUM_METHOD_SNIPPET: &'static str = r##"
<!DOCTYPE html>
<html>
<head>
  <title></title>
</head>
<body class="rustdoc">
  <nav class="sub">
    <form class="search-form">
      <div class="search-container">
        <input autocomplete="off" class="search-input" name="search" placeholder="Click or press ‘S’ to search, ‘?’ for more options…" type="search">
      </div>
    </form>
  </nav>
  <section class="content enum" id="main">
    <h1 class="fqn"><span class="in-band">Enum <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="enum" href=
    "#">Cow</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span> <span id="render-detail"><a href=
    "javascript:void(0)" id="toggle-all-docs" title="collapse all docs">[<span class="inner">−</span>]</a></span> <a class="srclink" href=
    "../../src/collections/up/src/libcollections/borrow.rs.html#115-127" id="src-2279" title="goto source code">[src]</a></span></h1>
    <h2 id="methods">Methods</h2>
    <h3 class="impl" id="dash_impl_1"><span class="in-band"><code>impl&lt;'a, B:&nbsp;?<a class="trait" href="../../core/marker/trait.Sized.html" title=
    "core::marker::Sized">Sized</a>&gt; <a class="enum" href="enum.Cow.html" title="collections::borrow::Cow">Cow</a>&lt;'a, B&gt; <span class="where">where B:
    <a class="trait" href="trait.ToOwned.html" title="collections::borrow::ToOwned">ToOwned</a></span></code></span><span class="out-of-band"></span></h3>
    <div class="ghost">
      <h3 class="impl" id="dash_impl_1"><a class="srclink" href="../../src/collections/up/src/libcollections/borrow.rs.html#142-194" id="src-2294" title=
      "goto source code">[src]</a></h3>
    </div>
    <div class="impl-items">
      <span class="out-of-band"><a class="dashAnchor" id="//dash_ref_511/Method/to%5Fmut/0" name="//dash_ref_511/Method/to%5Fmut/0"></a><a id="dash_method_1"
      name="dash_method_1"></a></span>
      <h4 class="method" id="method.to_mut" style=
      ";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span class=
      "out-of-band"><span class="invisible" id="to_mut.v"><code>fn <a class="fnname" href="#method.to_mut">to_mut</a>(&amp;mut self) -&gt; &amp;mut
      B::Owned</code></span><a class="collapse-toggle" href="javascript:void(0)">[<span class="inner">−</span>]</a></span></h4>
      <h4 class="method" id="method.into_owned"><span class="invisible" id="into_owned.v"><code>fn <a class="fnname" href=
      "#method.into_owned">into_owned</a>(self) -&gt; B::Owned</code></span><a class="collapse-toggle" href="javascript:void(0)">[<span class=
      "inner">−</span>]</a></h4>
    </div>
  </section>
</body>
</html>
"##;

pub static STRUCT_METHOD_SNIPPET: &'static str = r##"
<html>
<body class="rustdoc">
  <section class="content struct" id="main">
    <h1 class="fqn"><span class="in-band">Struct <a href="../index.html">core</a>::<wbr><a href="index-2.html">any</a>::<wbr><a class="struct" href=
    "#">TypeId</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span> <span id="render-detail"><a href=
    "javascript:void(0)" id="toggle-all-docs" title="collapse all docs">[<span class="inner">−</span>]</a></span> <a class="srclink" href=
    "../../src/core/up/src/libcore/any.rs.html#343-345" id="src-33815" title="goto source code">[src]</a></span></h1>
    <h2 id="methods">Methods</h2>
    <h3 class="impl" id="dash_impl_1"><span class="in-band"><code>impl <a class="struct" href="struct.TypeId.html" title=
    "core::any::TypeId">TypeId</a></code></span><span class="out-of-band"></span></h3>
    <div class="ghost">
      <h3 class="impl" id="dash_impl_1"><a class="srclink" href="../../src/core/up/src/libcore/any.rs.html#347-371" id="src-2820" title=
      "goto source code">[src]</a></h3>
    </div>
    <div class="impl-items">
      <span class="out-of-band"><a class="dashAnchor" id="//dash_ref_4950/Method/of/0" name="//dash_ref_4950/Method/of/0"></a><a id="dash_method_1" name=
      "dash_method_1"></a></span>
      <h4 class="method" id="method.of"><span class="out-of-band"><span class="invisible" id="of.v"><code>fn <a class="fnname" href=
      "#method.of">of</a>&lt;T:&nbsp;?<a class="trait" href="../marker/trait.Sized.html" title="core::marker::Sized">Sized</a> + 'static&gt;() -&gt; <a class=
      "struct" href="struct.TypeId.html" title="core::any::TypeId">TypeId</a></code></span><a class="collapse-toggle" href="javascript:void(0)">[<span class=
      "inner">−</span>]</a></span></h4>
    </div>
  </section>
</body>
</html>
"##;

// Module/Crate Collections, entry name: collections
pub static MODULE_SNIPPET: &'static str = r##"
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

// Struct collections::str::Bytes
pub static STRUCT_SNIPPET: &'static str = r##"
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

// Trait collections::fmt::Binary
pub static TRAIT_SNIPPET: &'static str = r##"
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

// Type core::iter::Chain::Item
pub static TYPE_SNIPPET: &'static str = r##"
<!DOCTYPE html>
<html>
<body class="rustdoc">
  <section class="content struct" id="main">
    <h1 class="fqn"><span class="in-band">Struct <a href=
    "../index.html">core</a>::<wbr><a href=
    "index-2.html">iter</a>::<wbr><a class="struct" href=
    "#">Chain</a></span><span class="out-of-band"><span class="since" title=
    "Stable since Rust version 1.0.0">1.0.0</span> <span id=
    "render-detail"><a href="javascript:void(0)" id="toggle-all-docs" title=
    "collapse all docs">[<span class="inner"></span>]</a></span> <a class=
    "srclink" href="../../src/core/up/src/libcore/iter/mod.rs.html#504-508" id=
    "src-41701" title="goto source code">[src]</a></span></h1>
    <div class="impl-items">
      <a class="dashAnchor" id="//dash_ref_5608/Type/Item/0" name=
      "//dash_ref_5608/Type/Item/0"></a><a id="dash_method_4" name=
      "dash_method_4"></a>
      <h4 class="type" id="associatedtype.Item" style=
      ";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;">
      <span class="invisible" id="Item.t"><code>type <a class="type" href=
      "trait.Iterator.html#associatedtype.Item">Item</a> =
      A::Item</code></span></h4>
    </div>
  </section>
</body>
</html>
"##;

// Struct core::ops::Range / Fields, field name: core::ops::Range::start
pub static FIELD_SNIPPET:  &'static str = r##"
<!DOCTYPE html>
<html>
<body class="rustdoc">
  <section class="content struct" id="main">
    <h1 class="fqn">
		<span class="in-band">Struct <a href= "../index.html">core</a>::<wbr><a href="index-2.html">ops</a>::<wbr><a class="struct" href="#">Range</a></span>
		<span class="out-of-band">
          <span class="since" title="Stable since Rust version 1.0.0">1.0.0</span>
		  <span id="render-detail">
            <a href="javascript:void(0)" id="toggle-all-docs" title="collapse all docs">[<span class="inner"></span>]</a>
          </span>
          <a class="srclink" href="../../src/core/up/src/libcore/ops.rs.html#2039-2046" id="src-33148" title="goto source code">[src]</a>
        </span>
     </h1>
     <span class="structfield" id="structfield.start">
       <span class="invisible" id="start.v">
         <code>start: Idx</code>
       </span>
     </span>
     <span class="stab"></span>
  </section>
</body>
</html>
"##;

// collections::borrow::Cow::Borrowed
pub static VARIANT_SNIPPET: &'static str = r##"
<html>
<body class="rustdoc">
  <section class="content enum" id="main">
    <h1 class="fqn"><span class="in-band">Enum <a href=
    "../index.html">collections</a>::<wbr><a href=
    "index-2.html">borrow</a>::<wbr><a class="enum" href=
    "#">Cow</a></span><span class="out-of-band"><span class="since" title=
    "Stable since Rust version 1.0.0">1.0.0</span> <span id=
    "render-detail"><a href="javascript:void(0)" id="toggle-all-docs" title=
    "collapse all docs">[<span class="inner">−</span>]</a></span> <a class=
    "srclink" href=
    "../../src/collections/up/src/libcollections/borrow.rs.html#115-127" id=
    "src-2279" title="goto source code">[src]</a></span></h1><span class=
    "variant" id="variant.Borrowed" style=
    ";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span class="invisible"
    id="Borrowed.v"><code>Borrowed(&amp;'a B)</code></span></span>
  </section>
</body>
</html>
"##;

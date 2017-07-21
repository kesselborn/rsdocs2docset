use html5ever::{ParseOpts, parse_document};
use html5ever::rcdom::RcDom;
use html5ever::tree_builder::TreeBuilderOpts;
use tendril::TendrilSink;

/// ///////////////// prettifier: https://www.freeformatter.com/html-formatter.html

pub fn dom_from_snippet(s: &str) -> RcDom {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts { drop_doctype: true, ..Default::default() },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut format!("<html><head></head><body>{}</body></html>", s).as_bytes())
        .unwrap();

    dom
}

/// alloc::arc::Arc impl Arc, entry name: impl<T> Arc<T>
pub static IMPL_SNIPPET: &'static str = r##"
<h3 class="impl">
  <span class="in-band">
    <code>impl&lt;T&gt; <a class="struct" href="struct.Arc.html" title="struct alloc::arc::Arc">Arc</a>&lt;T&gt;</code>
  </span>
  <span class="out-of-band">
    <div class="ghost"></div>
    <a class="srclink" href="../../src/alloc/arc.rs.html#225-351" title="goto source code">[src]</a>
  </span>
</h3>
"##;


/// alloc::boxed::HEAP constant
pub static CONST_SNIPPET: &'static str = r##"
<section id="main" class="content">
  <h1 class="fqn">
    <span class="in-band">Constant 
      <a href="../index.html">alloc</a>::<wbr><a href="index-2.html">boxed</a>::<wbr><a class="constant" href="#">HEAP</a>
    </span>
    <span class="out-of-band">
      <span class="since" title="Stable since Rust version "></span>
      <span id="render-detail">
        <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs"> [<span class="inner">−</span>] </a>
      </span>
      <a class="srclink" href="../../src/alloc/boxed.rs.html#91" title="goto source code">[src]</a>
    </span>
  </h1>
<section>
"##;

// collections::borrow::Cow
pub static ENUM_SNIPPET: &'static str = r##"
<section id="main" class="content">
   <h1 class="fqn"><span class="in-band">Enum <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="enum" href="#">Cow</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
      <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
      [<span class="inner">−</span>]
      </a>
      </span><a class="srclink" href="../../src/collections/borrow.rs.html#144-156" title="goto source code">[src]</a></span>
   </h1>
</section>
"##;

// Function std::fs::metadata
pub static FUNCTION_SNIPPET: &'static str = r##"
<section id="main" class="content">
   <h1 class="fqn">
     <span class="in-band">Function <a href="../index.html">std</a>::<wbr><a href="index-2.html">fs</a>::<wbr><a class="fn" href="#">metadata</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
      <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
      [<span class="inner">−</span>]
      </a>
      </span><a class="srclink" href="../../src/std/fs.rs.html#1239-1241" title="goto source code">[src]</a></span>
   </h1>
</section>
"##;

// Macro std::println
pub static MACRO_SNIPPET: &'static str = r##"
<section id="main" class="content">
   <h1 class="fqn"><span class="in-band">Macro <a href="index-2.html">std</a>::<wbr><a class="macro" href="#">println</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
      <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
      [<span class="inner">−</span>]
      </a>
      </span><a class="srclink" href="../src/std/macros.rs.html#121-125" title="goto source code">[src]</a></span>
   </h1>
</section>
"##;

// Trait method collections::borrow::Borrow::borrow
pub static TRAIT_METHOD_SNIPPET: &'static str = r##"
<html>
   <body class="rustdoc trait">
      <section id="main" class="content">
         <h1 class="fqn"><span class="in-band">Trait <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="trait" href="#">Borrow</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
            <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
            [<span class="inner">−</span>]
            </a>
            </span><a class="srclink" href="../../src/core/borrow.rs.html#36-58" title="goto source code">[src]</a></span>
         </h1>
         <pre class="rust trait">pub trait Borrow&lt;Borrowed&gt; <span class="where fmt-newline">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Borrowed: ?<a class="trait" href="../../core/marker/trait.Sized.html" title="trait core::marker::Sized">Sized</a>,&nbsp;</span>{
    fn <a href="#tymethod.borrow" class="fnname">borrow</a>(&amp;self) -&gt; &amp;Borrowed;
}</pre>
         <div class="toggle-wrapper"><a href="javascript:void(0)" class="collapse-toggle">[<span class="inner">−</span>]<span class="toggle-label" style="display: none;">&nbsp;Expand&nbsp;description</span></a></div>
         <h2 id="required-methods">Required Methods</h2>
         <div class="methods">
            <h3 id="tymethod.borrow" class="method" style=";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span id="borrow.v" class="invisible"><code>fn <a href="#tymethod.borrow" class="fnname">borrow</a>(&amp;self) -&gt; &amp;Borrowed</code></span><a href="javascript:void(0)" class="collapse-toggle">[<span class="inner">−</span>]</a></h3>
         </div>
      </section>
   </body>
</html>
"##;

// Enum method: collections::borrow::Cow::to_mut
pub static ENUM_METHOD_SNIPPET: &'static str = r##"
<html>
   <body class="rustdoc enum">
      <section id="main" class="content">
         <h1 class="fqn"><span class="in-band">Enum <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="enum" href="#">Cow</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
            <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
            [<span class="inner">−</span>]
            </a>
            </span><a class="srclink" href="../../src/collections/borrow.rs.html#144-156" title="goto source code">[src]</a></span>
         </h1>
         <div class="impl-items">
            <h4 id="method.to_mut" class="method" style=";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span id="to_mut.v" class="invisible"><code>fn <a href="#method.to_mut" class="fnname">to_mut</a>(&amp;mut self) -&gt; &amp;mut &lt;B as <a class="trait" href="trait.ToOwned.html" title="trait collections::borrow::ToOwned">ToOwned</a>&gt;::<a class="trait" href="trait.ToOwned.html" title="trait collections::borrow::ToOwned">Owned</a></code></span><a href="javascript:void(0)" class="collapse-toggle">[<span class="inner">−</span>]</a></h4>
         </div>
      </section>
   </body>
</html>
"##;

// core::any::TypeId::of
pub static STRUCT_METHOD_SNIPPET: &'static str = r##"
<html>
   <body class="rustdoc struct">
      <section id="main" class="content">
         <h1 class="fqn"><span class="in-band">Struct <a href="../index.html">core</a>::<wbr><a href="index-2.html">any</a>::<wbr><a class="struct" href="#">TypeId</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
            <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
            [<span class="inner">−</span>]
            </a>
            </span><a class="srclink" href="../../src/core/any.rs.html#347-349" title="goto source code">[src]</a></span>
         </h1>
         <pre class="rust struct">pub struct TypeId { /* fields omitted */ }</pre>
         <h2 id="methods">Methods</h2>
         <div class="impl-items">
            <a class="dashAnchor" name="//dash_ref_5458/Method/of/0"></a><a name="dash_method_1"></a>
            <h4 id="method.of" class="method" style=";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span id="of.v" class="invisible"><code>fn <a href="#method.of" class="fnname">of</a>&lt;T:&nbsp;?<a class="trait" href="../marker/trait.Sized.html" title="trait core::marker::Sized">Sized</a> + 'static&gt;() -&gt; <a class="struct" href="struct.TypeId.html" title="struct core::any::TypeId">TypeId</a></code></span></h4>
         </div>
      </section>
   </body>
</html>
"##;

// Module/Crate Collections, entry name: collections
pub static MODULE_SNIPPET: &'static str = r##"
<section id="main" class="content">
   <h1 class="fqn"><span class="in-band">Crate <a class="mod" href="#">collections</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version "></span><span id="render-detail">
      <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
      [<span class="inner">−</span>]
      </a>
      </span><a class="srclink" href="../src/collections/lib.rs.html#11-191" title="goto source code">[src]</a></span>
   </h1>
</section>
"##;

// Struct collections::str::Bytes
pub static STRUCT_SNIPPET: &'static str = r##"
<section id="main" class="content">
   <h1 class="fqn"><span class="in-band">Struct <a href="../index.html">collections</a>::<wbr><a href="index-2.html">str</a>::<wbr><a class="struct" href="#">Bytes</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
      <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
      [<span class="inner">−</span>]
      </a>
      </span><a class="srclink" href="../../src/core/str/mod.rs.html#680" title="goto source code">[src]</a></span>
   </h1>
</section>
"##;

// Trait collections::fmt::Binary
pub static TRAIT_SNIPPET: &'static str = r##"
<section id="main" class="content">
   <h1 class="fqn"><span class="in-band">Trait <a href="../index.html">collections</a>::<wbr><a href="index-2.html">fmt</a>::<wbr><a class="trait" href="#">Binary</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
      <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
      [<span class="inner">−</span>]
      </a>
      </span><a class="srclink" href="../../src/core/fmt/mod.rs.html#667-671" title="goto source code">[src]</a></span>
   </h1>
</section>
"##;

// Type core::iter::Chain::Item
pub static TYPE_SNIPPET: &'static str = r##"
<html>
   <body class="rustdoc struct">
      <section id="main" class="content">
         <h1 class="fqn"><span class="in-band">Struct <a href="../index.html">core</a>::<wbr><a href="index-2.html">iter</a>::<wbr><a class="struct" href="#">Chain</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
            <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
            [<span class="inner">−</span>]
            </a>
            </span><a class="srclink" href="../../src/core/iter/mod.rs.html#533-537" title="goto source code">[src]</a></span>
         </h1>
         <div class="impl-items">
            <h4 id="associatedtype.Item" class="type" style=";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span id="Item.t" class="invisible"><code>type <a href="trait.Iterator.html#associatedtype.Item" class="type"><span class="DHSpanWrap"><span class="DHHighlightSpan">Item</span></span></a> = A::<a class="trait" href="trait.Iterator.html" title="trait core::iter::Iterator"><span class="DHSpanWrap"><span class="DHHighlightSpan">Item</span></span></a></code></span></h4>
         </div>
      </section>
   </body>
</html>
"##;

// Struct core::ops::Range / Fields, field name: core::ops::Range::start
pub static FIELD_SNIPPET:  &'static str = r##"
<html>
   <body class="rustdoc struct">
      <section id="main" class="content">
         <h1 class="fqn"><span class="in-band">Struct <a href="../index.html">core</a>::<wbr><a href="index-2.html">ops</a>::<wbr><a class="struct" href="#">Range</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
            <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
            [<span class="inner">−</span>]
            </a>
            </span><a class="srclink" href="../../src/core/ops.rs.html#2068-2075" title="goto source code">[src]</a></span>
         </h1>
         <pre class="rust struct">pub struct Range&lt;Idx&gt; {
    pub start: Idx,
    pub end: Idx,
}</pre>
         <h2 id="fields" class="fields">Fields</h2>
         <span id="structfield.start" class="structfield" style=";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;">
         <span id="start.v" class="invisible">
         <code>start: Idx</code>
         </span></span>
      </section>
   </body>
</html>
"##;

// collections::borrow::Cow::Borrowed
pub static VARIANT_SNIPPET: &'static str = r##"
<html>
   <body class="rustdoc enum">
      <section id="main" class="content">
         <h1 class="fqn"><span class="in-band">Enum <a href="../index.html">collections</a>::<wbr><a href="index-2.html">borrow</a>::<wbr><a class="enum" href="#">Cow</a></span><span class="out-of-band"><span class="since" title="Stable since Rust version 1.0.0">1.0.0</span><span id="render-detail">
            <a id="toggle-all-docs" href="javascript:void(0)" title="collapse all docs">
            [<span class="inner">−</span>]
            </a>
            </span><a class="srclink" href="../../src/collections/borrow.rs.html#144-156" title="goto source code">[src]</a></span>
         </h1>
         <pre class="rust enum">pub enum Cow&lt;'a, B:&nbsp;?<a class="trait" href="../../core/marker/trait.Sized.html" title="trait core::marker::Sized">Sized</a> + 'a&gt; <span class="where fmt-newline">where<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class="trait" href="trait.ToOwned.html" title="trait collections::borrow::ToOwned">ToOwned</a>,&nbsp;</span> {
    Borrowed(&amp;'a B),
    Owned(&lt;B as <a class="trait" href="trait.ToOwned.html" title="trait collections::borrow::ToOwned">ToOwned</a>&gt;::<a class="trait" href="trait.ToOwned.html" title="trait collections::borrow::ToOwned">Owned</a>),
}</pre>
         <h2 id="variants" class="variants">Variants</h2>
         <span id="variant.Borrowed" class="variant" style=";-webkit-transition-property:background-color;-webkit-transition-duration:0.4s;-webkit-transition-timing-function:ease-in;"><span id="Borrowed.v" class="invisible"><code>Borrowed(&amp;'a B)</code></span></span>
      </section>
   </body>
</html>
"##;

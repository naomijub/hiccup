# Hiccup 
[![Build Status](https://travis-ci.org/naomijub/hiccup.svg?branch=master)](https://travis-ci.org/naomijub/hiccup)

A Clojure's [Hiccup](https://github.com/weavejester/hiccup) inspired macro. At the moment **support for inline code execution is not guaranteed**.
**The main objective of this lib is to prevent unclosed html tags.**

## Basic elements: 

The macro `hiccup! receives a mutable string as the first argument and mutates the string to emit the HTML.
The order of the elements is: 
1. `tag` as the first element.
2. Optional attribute inside the tag should follow the tag name as `{attribute1=>"value1 value2 ... valuen", attr=>"value"}`. Also, the attributes should be inside `{...}` and separate each key value pair by `,`.
The element should be written as `key=>"value"`, where key is a symbol, followed by an arrow (`=>`), and then the value as a string `"value"`.
3. After (Optional) the tag name or the attributes `{...}` you could include `[...]` that can have other tags, such as `p["text"]` or regular string values.
4. Inside the `[...]` you also can substitute your string for some simple rust code inside a `(...)`. This can bem something like `p[format!("{:?}", 3 + 4)]` or `div[(x)]` where x was defined in the outside.

### Differences between Clojure and Rust Hiccup: 
* [Clojure](https://github.com/weavejester/hiccup/wiki/Syntax): `[:a {:href "http://github.com"} "GitHub"]`
* Rust: `a{href=>"http://github.com"}["GitHub"]`

## Usage

Add dependency to `Cargo.toml`:
 
```toml
[dependencies]
hiccup = "0.2.5"
```

Code example with `hiccup!` macro:

```rust
use hiccup::hiccup;

fn main() {
    let mut html = String::new();

    let _ = hiccup!(&mut html,
        html[
            head[meta{name=>"author", content=>"Julia Naomi"}
                title["Hiccup guide"]]
            body{class=>"amazing hiccup guide"}[
                h1{font=>"bold", color=>"red"}["Hiccup is the best!"]
                p["please lookup clojure's hiccup for better ideas on this macro"]]
        ]);

    assert_eq!(html,"<html><head><meta name=\"author\" content=\"Julia Naomi\"/>\
    <title>Hiccup guide</title></head><body class=\"amazing hiccup guide\">\
    <h1 font=\"bold\" color=\"red\">Hiccup is the best!</h1>\
    <p>please lookup clojure\'s hiccup for better ideas on this macro</p></body></html>");
}
```

With remote code execution:

```rust
let mut html_inner = String::new();
let mut html_outer = String::new();
let x = "inner my str";
let y = "my str2";
let z = "my str3";

let _ = hiccup!(&mut html_inner,
    div[
        div{hello=>"inner world"}[(x)]
    ]
);

let _ = hiccup!(&mut html_outer,
    html[
        body{class=>"amazing hiccup guide"}[
            p["please lookup clojure's hiccup for better ideas on this macro"]
            div[
                div{hello=>"world"}[(html_inner)]
                div[(y.to_owned() + " " + z)]
                p["bye"]
            ]
        ]
    ]);

assert_eq!(html_outer,"<html><body class=\"amazing hiccup guide\">\
<p>please lookup clojure\'s hiccup for better ideas on this macro</p>\
<div><div hello=\"world\"><div><div hello=\"inner world\">inner my str</div></div></div>\
<div>my str2 my str3</div><p>bye</p></div></body></html>");
```

## FAQs

1. Is it possible tu use this lib as an XML templating?
> Yes, I added a more generic XML case to the tests recently

```rust
fn main() {
    let mut out = String::new();

    let _ = hiccup!(&mut out,
        xml{metas=>"I forgot them all", version=>"any version"}[
            family[name{name=>"Rubiechiov", origin=>"Kazakhstan"}]
            members{class=>"close family"}[
                member{age=>"oldest", color=>"yellow"}["some name"]
                member{age=>"mid-age", color=>"yellow"}["some other name"]
                member{age=>"yougest", color=>"brown"}["Julia"]]
        ]);

    assert_eq!(out,"<xml metas=\"I forgot them all\" version=\"any version\"><family>\
    <name name=\"Rubiechiov\" origin=\"Kazakhstan\"/></family><members class=\"close family\">\
    <member age=\"oldest\" color=\"yellow\">some name</member>\
    <member age=\"mid-age\" color=\"yellow\">some other name</member>\
    <member age=\"yougest\" color=\"brown\">Julia</member></members></xml>");
}
```

## Contribution
Add the feature you included in the macro as the name of the PR **with tests**

### Thanks to
[@otaviopace](https://github.com/otaviopace)
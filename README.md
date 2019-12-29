# Hiccup

A Clojure's [Hiccup](https://github.com/weavejester/hiccup) inspired macro. At the moment **support for inline code execution is not guaranteed**.
**The main objective of this lib is to prevent unclosed html tags.**

## Basic elements: 

The macro `hiccup! receives a mutable string as the first argument and mutates the string to emit the HTML.
The order of the elements is: 
1. `tag` as the first element.
2. Optional attribute inside the tag should follow the tag name as `{attribute1=>"value1 value2 ... valuen", attr=>"value"}`. Also, the attributes should be inside `{...}` and separate each key value pair by `,`.
The element should be written as `key=>"value"`, where key is a symbol, followed by an arrow (`=>`), and then the value as a string `"value"`.
3. After (Optional) the tag name or the attributes `{...}` tou should include `[...]` that can have other tags, such as `p["text"]` or regular string values.

### Differences between Clojure and Rust Hiccup: 
* [Clojure](https://github.com/weavejester/hiccup/wiki/Syntax): `[:a {:href "http://github.com"} "GitHub"]`
* Rust: `a{href=>"http://github.com"}["GitHub"]`

## Usage

Add dependency to `Cargo.toml`:
 
```toml
[dependencies]
hiccup = "0.1.6"
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

## Contribution
Add the feature you included in the macro as the name of the PR **with tests*
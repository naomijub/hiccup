# Hiccup

A Clojure's Hiccup inspired macro. At the moment **support for inline code execution is not guaranteed**.
**The main objective of this lib is to prevent unclosed html tags.**

## Usage

Add dependency to `cargo.toml`:
 
```toml
[dependencies]
hiccup = "0.1.1"
```

Code example with `hiccup!` macro:

```rust
extern crate hiccup;

use hiccup::hiccup;

fn main() {
    let mut html = String::new();

    let _ = hiccup!(&mut html,
        html[
            head[meta{name=>"author", content=>"Julia Naomi"}[]
                title["Hiccup guide"]]
            body{class=>"amazing hiccup guide"}[
                h1{font=>"bold"}["Hiccup is the best!"]
                p["please lookup clojure's hiccup for better ideas on this macro"]]
        ]);

    assert_eq!(html,"<html><head><meta name=\"author\" content=\"Julia Naomi\"></meta>\
    <title>Hiccup guide</title></head><body class=\"amazing hiccup guide\">\
    <h1 font=\"bold\">Hiccup is the best!</h1>\
    <p>please lookup clojure\'s hiccup for better ideas on this macro</p></body></html>");
}
```
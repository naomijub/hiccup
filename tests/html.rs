extern crate hiccup;

use hiccup::hiccup;

#[test]
fn html_templating() {
    let mut out = String::new();

    let _ = hiccup!(&mut out,
        html[
            head[meta{name=>"author", content=>"Julia Naomi"}
                title["Hiccup guide"]]
            body{class=>"amazing hiccup guide"}[
                h1{font=>"bold", color=>"red"}["Hiccup is the best!"]
                p["please lookup clojure's hiccup for better ideas on this macro"]]
        ]);

    assert_eq!(out,"<html><head><meta name=\"author\" content=\"Julia Naomi\"/>\
    <title>Hiccup guide</title></head><body class=\"amazing hiccup guide\">\
    <h1 font=\"bold\" color=\"red\">Hiccup is the best!</h1>\
    <p>please lookup clojure\'s hiccup for better ideas on this macro</p></body></html>");
}
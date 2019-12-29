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


#[test]
fn inner_html_templating() {
    let mut out = String::new();

    let _ = hiccup!(&mut out,
        html[
            body{class=>"amazing hiccup guide"}[
                h1{font=>"bold", color=>"red"}[bold["Hiccup"] "is the best!"]
                p["please lookup clojure's hiccup for better ideas on this macro"]
                p[(format!("{:?}", 3 + 5))]]
        ]);

    assert_eq!(out,"<html><body class=\"amazing hiccup guide\"><h1 font=\"bold\" color=\"red\">\
    <bold>Hiccup</bold>is the best!</h1><p>please lookup clojure\'s hiccup for better ideas on this macro</p>\
    <p>8</p></body></html>");
}

#[test]
fn remote_code_templating() {
    let mut out = String::new();
    let x = "my str";
    let y = "my str2";
    let z = "my str3";

    let _ = hiccup!(&mut out,
        html[
            body{class=>"amazing hiccup guide"}[
                h1{font=>"bold", color=>"red"}[bold["Hiccup"] "is the best!"]
                p["please lookup clojure's hiccup for better ideas on this macro"]
                div[
                    div{hello=>"world"}[(x)]
                    div[(y.to_owned() + " " + z)]
                ]
            ]
        ]);

    assert_eq!(out,"<html><body class=\"amazing hiccup guide\"><h1 font=\"bold\" color=\"red\">\
    <bold>Hiccup</bold>is the best!</h1><p>please lookup clojure\'s hiccup for better ideas on this macro</p>\
    <div><div hello=\"world\">my str</div><div>my str2 my str3</div></div></body></html>");
}

#[test]
fn remote_code_with_extra_tag_templating() {
    let mut out = String::new();
    let x = "my str";
    let y = "my str2";
    let z = "my str3";

    let _ = hiccup!(&mut out,
        html[
            body{class=>"amazing hiccup guide"}[
                h1{font=>"bold", color=>"red"}[bold["Hiccup"] "is the best!"]
                p["please lookup clojure's hiccup for better ideas on this macro"]
                div[
                    div{hello=>"world"}[(x)]
                    div[(y.to_owned() + " " + z)]
                    p["bye"]
                ]
            ]
        ]);

    assert_eq!(out,"<html><body class=\"amazing hiccup guide\"><h1 font=\"bold\" color=\"red\">\
    <bold>Hiccup</bold>is the best!</h1><p>please lookup clojure\'s hiccup for better ideas on this macro</p>\
    <div><div hello=\"world\">my str</div><div>my str2 my str3</div><p>bye</p></div></body></html>");
}

#[test]
fn remote_code_with_inner_tags_templating() {
    let mut out_inner = String::new();
    let mut out_outer = String::new();
    let x = "inner my str";
    let y = "my str2";
    let z = "my str3";

    let _ = hiccup!(&mut out_inner,
        div[
            div{hello=>"inner world"}[(x)]
        ]
    );

    let _ = hiccup!(&mut out_outer,
        html[
            body{class=>"amazing hiccup guide"}[
                p["please lookup clojure's hiccup for better ideas on this macro"]
                div[
                    div{hello=>"world"}[(out_inner)]
                    div[(y.to_owned() + " " + z)]
                    p["bye"]
                ]
            ]
        ]);

    assert_eq!(out_outer,"<html><body class=\"amazing hiccup guide\">\
    <p>please lookup clojure\'s hiccup for better ideas on this macro</p>\
    <div><div hello=\"world\"><div><div hello=\"inner world\">inner my str</div></div></div>\
    <div>my str2 my str3</div><p>bye</p></div></body></html>");
}
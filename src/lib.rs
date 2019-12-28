/// `hiccup!`: 
/// * The main objective of this lib is to prevent unclosed html tags.
/// 
/// ```rust
/// extern crate hiccup;
///
/// use hiccup::hiccup;
///
/// fn main() {
///     let mut html = String::new();
///
///     let _ = hiccup!(&mut html,
///         html[
///             head[meta{name=>"author", content=>"Julia Naomi"}[]
///                 title["Hiccup guide"]]
///             body{class=>"amazing hiccup guide"}[
///                 h1{font=>"bold"}["Hiccup is the best!"]
///                 p["please lookup clojure's hiccup for better ideas on this macro"]]
///         ]);
///
///     assert_eq!(html,"<html><head><meta name=\"author\" content=\"Julia Naomi\"></meta>\
///     <title>Hiccup guide</title></head><body class=\"amazing hiccup guide\">\
///     <h1 font=\"bold\">Hiccup is the best!</h1>\
///     <p>please lookup clojure\'s hiccup for better ideas on this macro</p></body></html>");
/// }
/// ```
/// 
#[macro_export]
macro_rules! hiccup {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => {{
        use std::fmt::Write;
        let _ = write!($w, "{}", $e);
    }};

    ($w:expr, $tag:ident {$($key:expr => $value:expr),*}[$($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}", stringify!($tag));
        $(
            let _ = write!($w, " {}=", stringify!($key));
            let _ = write!($w, "{}", stringify!($value));
        )*
        let _ = write!($w, ">");

        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
    }};

    ($w:expr, $tag:ident [$($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}>", stringify!($tag));
        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_html() {
        let mut out = String::new();

        let _ = hiccup!(&mut out,
            html[
                head[title["Hiccup guide"]]
                body[h1["Hiccup is the best!"]]
            ]);

            assert_eq!(out, "<html><head><title>Hiccup guide</title></head>\
            <body><h1>Hiccup is the best!</h1></body></html>");
    }

    #[test]
    fn attr_block() {
        let mut out = String::new();

        let _ = hiccup!(&mut out,
            html[
                head[title["Hiccup guide"]]
                body[h1{class=>"value", c=>"v"}["Hiccup is the best!"]]
            ]);

        assert_eq!(out, "<html><head><title>Hiccup guide</title></head><body>\
        <h1 class=\"value\" c=\"v\">Hiccup is the best!</h1></body></html>");
    }
}

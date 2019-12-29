/// # `hiccup!`: 
/// * The main objective of this lib is to prevent unclosed html tags.
/// This macro is inspired by Clojures [hiccup](https://github.com/weavejester/hiccup)
/// 
/// ## Basic usage: 
/// 
/// The macro `hiccup! receives a mutable string as the first argument and mutates the string to emit the HTML.
/// The order of the elements is: 
/// 1. `tag` as the first element.
/// 2. Optional attribute inside the tag should follow the tag name as `{attribute1=>"value1 value2 ... valuen", attr=>"value"}`. Also, the attributes should be inside `{...}` and separate each key value pair by `,`.
/// The element should be written as `key=>"value"`, where key is a symbol, followed by an arrow (`=>`), and then the value as a string `"value"`.
/// 3. After (Optional) the tag name or the attributes `{...}` you could include `[...]` that can have other tags, such as `p["text"]` or regular string values.
/// 4. Inside the `[...]` you also can substitute your string for some simple rust code inside a `(...)`. This can bem something like `p[format!("{:?}", 3 + 4)]` or `div[(x)]` where x was defined in the outside. 
/// 
/// ### Differences between Clojure and Rust Hiccup: 
/// * [Clojure](https://github.com/weavejester/hiccup/wiki/Syntax): `[:a {:href "http://github.com"} "GitHub"]`
/// * Rust: `a{href=>"http://github.com"}["GitHub"]`
/// 
/// ### Syntax with code inside: 
/// * `a{href=>"http://github.com"}[(format!("{:?}", 3 + 5))]`
/// * `p[(x)]`, where `x` can be another `html` or simply a value;
/// 
/// ## Example
/// 
/// ### Basic syntax
/// ```rust
/// use hiccup::hiccup;
///
/// fn main() {
///     let mut html = String::new();
///
///     let _ = hiccup!(&mut html,
///         html[
///             head[meta{name=>"author", content=>"Julia Naomi"}
///                 title["Hiccup guide"]]
///             body{class=>"amazing hiccup guide"}[
///                 h1{font=>"bold", color=>"red"}["Hiccup is the best!"]
///                 p["please lookup clojure's hiccup for better ideas on this macro"]]
///         ]);
///
///     assert_eq!(html,"<html><head><meta name=\"author\" content=\"Julia Naomi\"/>\
///     <title>Hiccup guide</title></head><body class=\"amazing hiccup guide\">\
///     <h1 font=\"bold\" color=\"red\">Hiccup is the best!</h1>\
///     <p>please lookup clojure\'s hiccup for better ideas on this macro</p></body></html>");
/// }
/// ```
/// 
/// ### With remote code execution
/// ```rust
/// use hiccup::hiccup;
///
/// fn main() {
///     let mut html_inner = String::new();
///     let mut html_outer = String::new();
///     let x = "inner my str";
///     let y = "my str2";
///     let z = "my str3";
///
///     let _ = hiccup!(&mut html_inner,
///         div[
///             div{hello=>"inner world"}[(x)]
///         ]
///     );
///
///     let _ = hiccup!(&mut html_outer,
///         html[
///             body{class=>"amazing hiccup guide"}[
///                 p["please lookup clojure's hiccup for better ideas on this macro"]
///                 div[
///                     div{hello=>"world"}[(html_inner)]
///                     div[(y.to_owned() + " " + z)]
///                     p["bye"]
///                 ]
///             ]
///         ]);
///
///     assert_eq!(html_outer,"<html><body class=\"amazing hiccup guide\">\
///     <p>please lookup clojure\'s hiccup for better ideas on this macro</p>\
///     <div><div hello=\"world\"><div><div hello=\"inner world\">inner my str</div></div></div>\
///     <div>my str2 my str3</div><p>bye</p></div></body></html>");
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

    ($w:expr, $tag:ident {$($key:expr => $value:expr),*}[($($code:block)*) $($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}", stringify!($tag));
        $(
            let _ = write!($w, " {}=", stringify!($key));
            let _ = write!($w, "{}", stringify!($value));
        )*
        let _ = write!($w, ">");
        $($code)*
        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
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

    ($w:expr, $tag:ident [($($code:block)*) $($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}>", stringify!($tag));
        $($code)*
        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
    }};

    ($w:expr, $tag:ident {$($key:expr => $value:expr),*} $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}", stringify!($tag));
        $(
            let _ = write!($w, " {}=", stringify!($key));
            let _ = write!($w, "{}", stringify!($value));
        )*
        let _ = write!($w, "/>");
        hiccup!($w, $($rest)*);
    }};

    ($w:expr, $tag:ident [$($inner:tt)*] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}>", stringify!($tag));
        hiccup!($w, $($inner)*);
        let _ = write!($w, "</{}>", stringify!($tag));
        hiccup!($w, $($rest)*);
    }};

    ($w:expr, $tag:ident [($($code:block)*)] $($rest:tt)*) => {{
        use std::fmt::Write;
        
        let _ = write!($w, "<{}>", stringify!($tag));
        $($code)*
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

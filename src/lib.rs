
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
            write!($w, " {}=", stringify!($key));
            write!($w, "{}", stringify!($value));
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


#[macro_export]
macro_rules! hiccup {
    ($w:expr, ) => (());

    ($w:expr, $e:tt) => {{
        use std::fmt::Write;
        let _ = write!($w, "{}", $e);
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

        hiccup!(&mut out,
            html[
                head[title["Macros guide"]]
                body[h1["Macros are the best!"]]
            ]);

            assert_eq!(out, "<html><head><title>Macros guide</title></head>\
            <body><h1>Macros are the best!</h1></body></html>");
    }
}

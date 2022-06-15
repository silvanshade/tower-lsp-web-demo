#![deny(clippy::all)]
#![deny(unsafe_code)]

mod core;
pub mod handler;
mod server;

pub use server::*;

pub(crate) fn format_sexp(sexp: impl AsRef<str>) -> String {
    format_sexp_indented(sexp, 0)
}

fn format_sexp_indented(sexp: impl AsRef<str>, initial_indent_level: u32) -> String {
    use std::fmt::Write;

    let sexp = sexp.as_ref();
    let mut formatted = String::new();
    let mut indent_level = initial_indent_level;
    let mut has_field = false;
    let mut s_iter = sexp.split(|c| c == ' ' || c == ')');
    while let Some(s) = s_iter.next() {
        if s.is_empty() {
            // ")"
            indent_level -= 1;
            write!(formatted, ")").unwrap();
        } else if s.starts_with('(') {
            if has_field {
                has_field = false;
            } else {
                if indent_level > 0 {
                    writeln!(formatted).unwrap();
                    for _ in 0 .. indent_level {
                        write!(formatted, "  ").unwrap();
                    }
                }
                indent_level += 1;
            }

            // "(node_name"
            write!(formatted, "{}", s).unwrap();

            let mut c_iter = s.chars();
            c_iter.next();
            match c_iter.next() {
                Some('M') | Some('U') => {
                    // "(MISSING node_name" or "(UNEXPECTED 'x'"
                    let s = s_iter.next().unwrap();
                    write!(formatted, " {}", s).unwrap();
                },
                Some(_) | None => {},
            }
        } else if s.ends_with(':') {
            // "field:"
            writeln!(formatted).unwrap();
            for _ in 0 .. indent_level {
                write!(formatted, "  ").unwrap();
            }
            write!(formatted, "{} ", s).unwrap();
            has_field = true;
            indent_level += 1;
        }
    }

    formatted
}

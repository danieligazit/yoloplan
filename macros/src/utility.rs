use {
    inflector::Inflector,
    schema::Schema,
    proc_macro2::{Span, TokenStream},
};


pub fn replace_invalid_identifier_chars(s: &str) -> String {
    s.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
}

pub fn str_to_ident(s: &str) -> syn::Ident {
    let s = replace_invalid_identifier_chars(s);
    let keywords = [
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn",
        "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref",
        "return", "self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use",
        "where", "while", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
        "typeof", "unsized", "virtual", "yield", "async", "await", "try",
    ];

    if keywords.iter().any(|&keyword| keyword == s) {
        syn::Ident::new(&format!("{}_", s), Span::call_site())
    } else {
        syn::Ident::new(&format!("{}", s), Span::call_site())
    }
}

pub fn rename_keyword(prefix: &str, s: &str) -> Option<TokenStream> {
    let n = str_to_ident(s);

    if n != s {
        let prefix = syn::Ident::new(prefix, Span::call_site());
        Some(quote! {
            #[serde(rename = #s)]
            #prefix #n
        })
    } else {
        None
    }
}

pub fn field(s: &str) -> TokenStream {
    if let Some(t) = rename_keyword("pub", s) {
        t
    } else {
        let snake = s.to_snake_case();
        if snake != s || snake.contains(|c: char| c == '$' || c == '#') {
            let field = if snake == "ref" {
                syn::Ident::new("ref_".into(), Span::call_site())
            } else {
                syn::Ident::new(&snake.replace('$', "").replace('#', ""), Span::call_site())
            };

            quote! {
                #[serde(rename = #s)]
                pub #field
            }
        } else {
            let field = syn::Ident::new(s, Span::call_site());
            quote!( pub #field )
        }
    }
}

pub fn merge_option<T, F>(mut result: &mut Option<T>, r: &Option<T>, f: F)
where
    F: FnOnce(&mut T, &T),
    T: Clone,
{
    *result = match (&mut result, r) {
        (&mut &mut Some(ref mut result), &Some(ref r)) => return f(result, r),
        (&mut &mut None, &Some(ref r)) => Some(r.clone()),
        _ => return (),
    };
}

pub fn merge_all_of(result: &mut Schema, r: &Schema) {
    use std::collections::btree_map::Entry;

    for (k, v) in &r.properties {
        match result.properties.entry(k.clone()) {
            Entry::Vacant(entry) => {
                entry.insert(v.clone());
            }
            Entry::Occupied(mut entry) => merge_all_of(entry.get_mut(), v),
        }
    }

    if let Some(ref ref_) = r.ref_ {
        result.ref_ = Some(ref_.clone());
    }

    if let Some(ref description) = r.description {
        result.description = Some(description.clone());
    }

    merge_option(&mut result.required, &r.required, |required, r_required| {
        required.extend(r_required.iter().cloned());
    });

    result.type_.retain(|e| r.type_.contains(e));
}

pub fn make_doc_comment(mut comment: &str, remaining_line: usize) -> TokenStream {
    let mut out_comment = String::new();
    out_comment.push_str("/// ");
    let mut length = 4;
    while let Some(word) = comment.split(char::is_whitespace).next() {
        if comment.is_empty() {
            break;
        }
        comment = &comment[word.len()..];
        if length + word.len() >= remaining_line {
            out_comment.push_str("\n/// ");
            length = 4;
        }
        out_comment.push_str(word);
        length += word.len();
        let mut n = comment.chars();
        match n.next() {
            Some('\n') => {
                out_comment.push_str("\n");
                out_comment.push_str("/// ");
                length = 4;
            }
            Some(_) => {
                out_comment.push_str(" ");
                length += 1;
            }
            None => (),
        }
        comment = n.as_str();
    }
    if out_comment.ends_with(' ') {
        out_comment.pop();
    }
    out_comment.push_str("\n");
    out_comment.parse().unwrap()
}
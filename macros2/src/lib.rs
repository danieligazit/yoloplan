use std::path::PathBuf;

mod expander;
mod schema;
use expander::Expander;

/// A configurable builder for generating Rust types from a JSON
/// schema.
///
/// The default options are usually fine. In that case, you can use
/// the [`generate()`](fn.generate.html) convenience method instead.
struct GenerateBuilder<'a> {
    /// The module path to this crate. Some generated code may make
    /// use of types defined in this crate. Unless you have
    /// re-exported this crate or imported it under a different name,
    /// the default should be fine.
    pub schemafy_path: &'a str,
}

impl<'a> Default for GenerateBuilder<'a> {
    fn default() -> Self {
        GenerateBuilder {
            schemafy_path: "::schemafy_core::",
        }
    }
}

impl<'a> GenerateBuilder<'a> {
    fn build_tokens(mut self, tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
        struct Def {
            input_file: syn::LitStr,
        }

        impl syn::parse::Parse for Def {
            fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
                Ok(Def {
                    input_file: input.parse()?,
                })
            }
        }

        let def = syn::parse_macro_input!(tokens as Def);

        let input_file = PathBuf::from(def.input_file.value());
        let crate_root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());

        let input_path = if input_file.is_relative() {
            crate_root.join(input_file)
        } else {
            input_file
        };

        let json = std::fs::read_to_string(&input_path)
            .unwrap_or_else(|err| panic!("Unable to read `{}`: {}", input_path.to_string_lossy(), err));

        let schema = serde_json::from_str(&json).unwrap_or_else(|err| panic!("{}", err));
        let mut expander = Expander::new(
            self.schemafy_path,
            &schema,
        );
        expander.expand(&schema).into()
    }
}

/// Generate Rust types from a JSON schema.
///
/// If the `root` parameter is supplied, then a type will be
/// generated from the root of the schema.
///
/// ```rust
/// extern crate serde;
/// extern crate schemafy_core;
/// extern crate serde_json;
///
/// use serde::{Serialize, Deserialize};
///
/// schemafy::schemafy!(
///     root: MyRoot // Optional name for the root type (if one exists)
///     "tests/nested.json"
/// );
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let nested: Defnested = serde_json::from_str(r#"{ "append": "abc" }"#)?;
///     assert_eq!(nested.append, Some("abc".to_string()));
///     Ok(())
/// }
/// ```
#[proc_macro]
pub fn schemafy(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    GenerateBuilder {
        ..GenerateBuilder::default()
    }
    .build_tokens(tokens)
}

#[doc(hidden)]
#[proc_macro]
pub fn regenerate(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    use std::process::Command;

    let tokens = GenerateBuilder {
        ..GenerateBuilder::default()
    }
    .build_tokens(tokens);

    {
        let out = tokens.to_string();
        std::fs::write("src/schema.rs", &out).unwrap();
        Command::new("rustfmt")
            .arg("schemafy_lib/src/schema.rs")
            .output()
            .unwrap();
    }

    tokens
}
use std::path::PathBuf;
extern crate proc_macro2;
extern crate serde_json;
extern crate inflector;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate quote;

mod expander;
mod schema;
mod one_or_many;
mod utility;
use expander::Expander;

#[proc_macro]
pub fn schemafy(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
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
        &schema,
    );
    expander.expand(&schema).into()
}

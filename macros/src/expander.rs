
/// Types from the JSON Schema meta-schema (draft 4).
///
/// This module is itself generated from a JSON schema.
// use schema;

use {
    std::borrow::Cow,
    inflector::Inflector,
    serde_json::Value,
    schema::{Schema, SimpleTypes},
    proc_macro2::{Span, TokenStream},
    utility::*,
};



const LINE_LENGTH: usize = 100;
const INDENT_LENGTH: usize = 4;


struct FieldExpander<'a, 'r: 'a> {
    default: bool,
    expander: &'a mut Expander<'r>,
}

impl<'a, 'r> FieldExpander<'a, 'r> {
    fn expand_fields(&mut self, type_name: &str, schema: &Schema) -> Vec<TokenStream> {
        let schema = self.expander.schema(schema);
        schema
            .properties
            .iter()
            .map(|(field_name, value)| {
                self.expander.current_field.clone_from(field_name);
                let key = field(field_name);
                let required = schema
                    .required
                    .iter()
                    .flat_map(|a| a.iter())
                    .any(|req| req == field_name);
                let field_type = self.expander.expand_type(type_name, required, value);
                if !field_type.typ.starts_with("Option<") {
                    self.default = false;
                }
                let typ = field_type.typ.parse::<TokenStream>().unwrap();

                let default = if field_type.default {
                    Some(quote! { #[serde(default)] })
                } else {
                    None
                };
                let attributes = if field_type.attributes.is_empty() {
                    None
                } else {
                    let attributes = field_type
                        .attributes
                        .iter()
                        .map(|attr| attr.parse::<TokenStream>().unwrap());
                    Some(quote! {
                        #[serde( #(#attributes),* )]
                    })
                };
                let comment = value
                    .description
                    .as_ref()
                    .map(|comment| make_doc_comment(comment, LINE_LENGTH - INDENT_LENGTH));
                quote! {
                    #comment
                    #default
                    #attributes
                    #key : #typ
                }
            })
            .collect()
    }
}


pub struct Expander<'r> {
    current_type: String,
    current_field: String,
    types: Vec<(String, TokenStream)>,
    root: &'r Schema,
}

struct FieldType {
    typ: String,
    attributes: Vec<String>,
    default: bool,
}

impl<S> From<S> for FieldType
where
    S: Into<String>,
{
    fn from(s: S) -> FieldType {
        FieldType {
            typ: s.into(),
            attributes: Vec::new(),
            default: false,
        }
    }
}

impl<'r> Expander<'r> {
    pub fn new(root: &'r Schema) -> Expander<'r> {
        Expander {
            current_field: "".into(),
            current_type: "".into(),
            types: Vec::new(),
            root,
        }
    }

    fn type_ref(&self, s: &str) -> String {
        let s = s.split('/').last().expect("Component");
        replace_invalid_identifier_chars(&s.to_pascal_case())
    }

    fn schema(&self, schema: &'r Schema) -> Cow<'r, Schema> {
        let schema = match schema.ref_ {
            Some(ref ref_) => self.schema_ref(ref_),
            None => schema,
        };
        match schema.all_of {
            Some(ref all_of) if !all_of.is_empty() => {
                all_of
                    .iter()
                    .skip(1)
                    .fold(self.schema(&all_of[0]).clone(), |mut result, def| {
                        merge_all_of(result.to_mut(), &self.schema(def));
                        result
                    })
            }
            _ => Cow::Borrowed(schema),
        }
    }

    fn schema_ref(&self, s: &str) -> &'r Schema {
        s.split('/').fold(self.root, |schema, comp| {
            if comp == "#" {
                self.root
            } else if comp == "definitions" {
                schema
            } else {
                schema
                    .definitions
                    .get(comp)
                    .unwrap_or_else(|| panic!("Expected definition: `{}` {}", s, comp))
            }
        })
    }

    fn expand_type(&mut self, type_name: &str, required: bool, typ: &Schema) -> FieldType {
        let mut result = self.expand_type_(typ);
        if type_name == result.typ {
            result.typ = format!("Box<{}>", result.typ)
        }
        if !required && !result.default {
            result.typ = format!("Option<{}>", result.typ)
        }
        result
    }

    fn expand_type_(&mut self, typ: &Schema) -> FieldType {
        if let Some(ref ref_) = typ.ref_ {
            self.type_ref(ref_).into()
        } else if typ.any_of.as_ref().map_or(false, |a| a.len() == 2) {
            let any_of = typ.any_of.as_ref().unwrap();
            let simple = self.schema(&any_of[0]);
            let array = self.schema(&any_of[1]);
            if !array.type_.is_empty() {
                if let SimpleTypes::Array = array.type_[0] {
                    if simple == self.schema(&array.items[0]) {
                        return FieldType {
                            typ: format!("Vec<{}>", self.expand_type_(&any_of[0]).typ),
                            attributes: vec![
                                r#"with="::one_or_many""#.to_string(),
                            ],
                            default: true,
                        };
                    }
                }
            }
            return "serde_json::Value".into();
        } else if typ.type_.len() == 2 {
            if typ.type_[0] == SimpleTypes::Null || typ.type_[1] == SimpleTypes::Null {
                let mut ty = typ.clone();
                ty.type_.retain(|x| *x != SimpleTypes::Null);

                FieldType {
                    typ: format!("Option<{}>", self.expand_type_(&ty).typ),
                    attributes: vec![],
                    default: true,
                }
            } else {
                "serde_json::Value".into()
            }
        } else if typ.type_.len() == 1 {
            match typ.type_[0] {
                SimpleTypes::String => {
                    if typ.enum_.as_ref().map_or(false, |e| e.is_empty()) {
                        "serde_json::Value".into()
                    } else {
                        "String".into()
                    }
                }
                SimpleTypes::Integer => "i64".into(),
                SimpleTypes::Boolean => "bool".into(),
                SimpleTypes::Number => "f64".into(),
                // Handle objects defined inline
                SimpleTypes::Object
                    if !typ.properties.is_empty()
                        || typ.additional_properties == Some(Value::Bool(false)) =>
                {
                    let name = format!(
                        "{}{}",
                        self.current_type.to_pascal_case(),
                        self.current_field.to_pascal_case()
                    );
                    let tokens = self.expand_schema(&name, typ);
                    self.types.push((name.clone(), tokens));
                    name.into()
                }
                SimpleTypes::Object => {
                    let prop = match typ.additional_properties {
                        Some(ref props) if props.is_object() => {
                            let prop = serde_json::from_value(props.clone()).unwrap();
                            self.expand_type_(&prop).typ
                        }
                        _ => "serde_json::Value".into(),
                    };
                    let result = format!("::std::collections::BTreeMap<String, {}>", prop);
                    FieldType {
                        typ: result,
                        attributes: Vec::new(),
                        default: typ.default == Some(Value::Object(Default::default())),
                    }
                }
                SimpleTypes::Array => {
                    let item_type = typ.items.get(0).map_or("serde_json::Value".into(), |item| {
                        self.current_type = format!("{}Item", self.current_type);
                        self.expand_type_(item).typ
                    });
                    format!("Vec<{}>", item_type).into()
                }
                _ => "serde_json::Value".into(),
            }
        } else {
            "serde_json::Value".into()
        }
    }

    fn expand_definitions(&mut self, schema: &Schema) {
        for (name, def) in &schema.definitions {
            
            let type_decl = self.expand_schema(name, def);
            let definition_tokens = match def.description {
                Some(ref comment) => {
                    let t = make_doc_comment(comment, LINE_LENGTH);
                    quote! {
                        #t
                        #type_decl
                    }
                }
                None => type_decl,
            };
            self.types.push((name.to_string(), definition_tokens));
        }
    }

    fn get_identifier_values_logic(&self) -> TokenStream{

        let self_ident = syn::Ident::new("self", Span::call_site());

        let mut logic = quote!{
            let mut identifier_values = Vec::new();
        };

        for (field_name, field_schema) in &self.root.definitions["event"].properties{
            let identification_method = match &field_schema.identify {
                None => continue,
                Some(idetification_method) => idetification_method,
            };

            let field = syn::Ident::new(field_name, Span::call_site());
            
            logic = quote!{
                #logic
                identifier_values.push((#field_name.to_owned(), serde_json::to_value(#self_ident.#field)?, #identification_method.to_owned()));
            };
        }

        quote! {
            #logic
            Ok(identifier_values)
        }
    }

    fn expand_schema(&mut self, original_name: &str, schema: &Schema) -> TokenStream {
        self.expand_definitions(schema);

        let pascal_case_name = replace_invalid_identifier_chars(&original_name.to_pascal_case());
        self.current_type.clone_from(&pascal_case_name);
        let (fields, default) = {
            let mut field_expander = FieldExpander {
                default: true,
                expander: self,
            };
            let fields = field_expander.expand_fields(original_name, schema);
            (fields, field_expander.default)
        };
        let name = syn::Ident::new(&pascal_case_name, Span::call_site());
        
        let is_struct =
            !fields.is_empty() || schema.additional_properties == Some(Value::Bool(false));


        let type_decl = if is_struct {
            
            let struct_definition = if default {
                quote! {
                    #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
                    pub struct #name {
                        #(#fields),*
                    }
                }
            } else {
                quote! {
                    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                    pub struct #name {
                        #(#fields),*
                    }
                }
            };

            let identifier_values_logic = self.get_identifier_values_logic();

            quote! {
                #struct_definition
                
                impl #name {
                    fn get_identifier_values(self) -> Result<Vec<(String, serde_json::Value, String)>, Box<dyn std::error::Error>>{
                        #identifier_values_logic
                    }
                }
            }
        } else if schema.enum_.as_ref().map_or(false, |e| !e.is_empty()) {
            let mut optional = false;
            let mut repr_i64 = false;
            let variants = if schema.enum_names.as_ref().map_or(false, |e| !e.is_empty()) {
                let values = schema.enum_.as_ref().map_or(&[][..], |v| v);
                let names = schema.enum_names.as_ref().map_or(&[][..], |v| v);
                if names.len() != values.len() {
                    panic!("enumNames(length {}) and enum(length {}) have different length", names.len(), values.len())
                }
                names.iter()
                    .enumerate()
                    .map(|(idx, name)| (&values[idx], name))
                    .flat_map(|(value, name)| {
                        let pascal_case_variant = name.to_pascal_case();
                        let variant_name =
                            rename_keyword("", &pascal_case_variant).unwrap_or_else(|| {
                                let v = syn::Ident::new(&pascal_case_variant, Span::call_site());
                                quote!(#v)
                            });
                        match value {
                            Value::String(ref s) => Some(quote! {
                                #[serde(rename = #s)]
                                #variant_name
                            }),
                            Value::Number(ref n) => {
                                repr_i64 = true;
                                let num = syn::LitInt::new(&n.to_string(), Span::call_site());
                                Some(quote! {
                                    #variant_name = #num
                                })
                            },
                            Value::Null => {
                                optional = true;
                                None
                            },
                            _ => panic!("Expected string,bool or number for enum got `{}`", value),
                        }
                    })
                    .collect::<Vec<_>>()
            } else {
                schema
                    .enum_
                    .as_ref()
                    .map_or(&[][..], |v| v)
                    .iter()
                    .flat_map(|v| match *v {
                        Value::String(ref v) => {
                            let pascal_case_variant = v.to_pascal_case();
                            let variant_name =
                                rename_keyword("", &pascal_case_variant).unwrap_or_else(|| {
                                    let v = syn::Ident::new(&pascal_case_variant, Span::call_site());
                                    quote!(#v)
                                });
                            Some(if pascal_case_variant == *v {
                                variant_name
                            } else {
                                quote! {
                                    #[serde(rename = #v)]
                                    #variant_name
                                }
                            })
                        }
                        Value::Null => {
                            optional = true;
                            None
                        }
                        _ => panic!("Expected string for enum got `{}`", v),
                    })
                    .collect::<Vec<_>>()
            };
            if optional {
                let enum_name = syn::Ident::new(&format!("{}_", name), Span::call_site());
                if repr_i64 {
                    quote! {
                        pub type #name = Option<#enum_name>;
                        #[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
                        #[repr(i64)]
                        pub enum #enum_name {
                            #(#variants),*
                        }
                    }
                } else {
                    quote! {
                        pub type #name = Option<#enum_name>;
                        #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                        pub enum #enum_name {
                            #(#variants),*
                        }
                    }
                }
            } else {
                if repr_i64 {
                    quote! {
                        #[derive(Clone, PartialEq, Debug, Serialize_repr, Deserialize_repr)]
                        #[repr(i64)]
                        pub enum #name {
                            #(#variants),*
                        }
                    }
                } else {
                    quote! {
                        #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                        pub enum #name {
                            #(#variants),*
                        }
                    }
                }
            }
        } else {
            let typ = self
                .expand_type("", true, schema)
                .typ
                .parse::<TokenStream>()
                .unwrap();
            return quote! {
                pub type #name = #typ;
            };
        };
        if name == original_name {
            type_decl
        } else {
            quote! {
                #[serde(rename = #original_name)]
                #type_decl
            }
        }
    }

    pub fn expand(&mut self, schema: &Schema) -> TokenStream {
        self.expand_definitions(schema);

        let types = self.types.iter().map(|t| &t.1);
        quote! {
            #( #types )*
        }
    }

    pub fn expand_root(&mut self) -> TokenStream {
        self.expand(self.root)
    }
}
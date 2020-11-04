
/// Types from the JSON Schema meta-schema (draft 4).
///
/// This module is itself generated from a JSON schema.
// use schema;


use {
    std::borrow::Cow,
    inflector::Inflector,
    serde_json::Value,
    schema::Schema,
    proc_macro2::{Span, TokenStream},
    utility::*,
    common,
    common::SimpleTypes,
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
                quote! {
                    #default
                    #attributes
                    #key : #typ
                }
            })
            .collect()
    }
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

pub struct Expander<'r> {
    current_type: String,
    current_field: String,
    types: Vec<(String, TokenStream)>,
    root: &'r Schema,
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
                if let SimpleTypes::Vec = array.type_[0] {
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
            return "common::Value".into();
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
                "common::Value".into()
            }
        } else if typ.type_.len() == 1 {

            match typ.type_[0] {
                SimpleTypes::String =>"String".into(),
                SimpleTypes::Boolean => "bool".into(),
                SimpleTypes::Char => "char".into(),
                
                SimpleTypes::I64 => "i64".into(),
                SimpleTypes::I32 =>"i32".into(),
                SimpleTypes::I16 => "i16".into(),
                SimpleTypes::I8 => "i8".into(),

                SimpleTypes::F64 => "f64".into(),
                SimpleTypes::F32 =>"f32".into(),

                SimpleTypes::U64 => "u64".into(),
                SimpleTypes::U32 =>"u32".into(),
                SimpleTypes::U16 => "u16".into(),
                SimpleTypes::U8 => "u8".into(),
                
                SimpleTypes::USize => "usize".into(),
                SimpleTypes::ISize => "isize".into(),
                
                SimpleTypes::IP4 => "String".into(),
                SimpleTypes::Object
                    if !typ.properties.is_empty() => {
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
                    let result = "::std::collections::HashMap<String, common::Value>";
                    FieldType {
                        typ: result.to_owned(),
                        attributes: Vec::new(),
                        default: typ.default == Some(common::Value::Object(Default::default())),
                    }
                }

                SimpleTypes::Vec => {
                    let item_type = typ.items.get(0).map_or("common::Value".into(), |item| {
                        self.current_type = format!("{}Item", self.current_type);
                        self.expand_type_(item).typ
                    });
                    format!("Vec<{}>", item_type).into()
                }
                _ => "common::Value".into(),
            }
        } else {
            "serde_json::Value".into()
        }
    }

    fn get_fields_logic(&mut self) -> TokenStream {
        let mut logic = quote!{
            let mut fields = Vec::new();
        };

        for (field_name, field_schema) in &self.root.definitions[&self.current_type].properties{
            let field_type = self.expand_type_(field_schema).typ;
  
            logic = quote!{
                #logic
                fields.push((#field_name.to_owned(), #field_type.to_owned()));
            };
        }

        quote! {
            #logic
            Ok(fields)
        }
    }

    fn get_values_logic(&mut self) -> TokenStream {
        let self_ident = syn::Ident::new("self", Span::call_site());

        let mut logic = quote!{
            let mut values = std::collections::HashMap::new();
        };

        for (field_name, field_schema) in &self.root.definitions[&self.current_type].properties{
            
            let field = syn::Ident::new(field_name, Span::call_site());
            logic = quote!{
                #logic
                values.insert(#field_name, common::Value::from(#self_ident.#field.clone()));
            };
        }

        quote! {
            #logic
            Ok(values)
        }
    }


    fn get_identifier_values_logic(&self) -> TokenStream{
        let self_ident = syn::Ident::new("self", Span::call_site());

        let mut logic = quote!{
            let mut identifier_values = Vec::new();
        };

        for (field_name, field_schema) in &self.root.definitions[&self.current_type].properties{
            let identification_method = match &field_schema.identify {
                None => continue,
                Some(idetification_method) => idetification_method,
            };

            let field = syn::Ident::new(field_name, Span::call_site());
            
            logic = quote!{
                #logic
                identifier_values.push((#field_name, common::Value::from(#self_ident.#field.clone()), #identification_method));
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
        
        let is_struct = !fields.is_empty();

        let type_decl = if is_struct {
            
            // fn get_struct_decleration(&self, name: &str, default: bool, fields: Vec<TokenStream>)
            let struct_definition = if default {
                quote! {
                    #[derive(Clone, PartialEq, Debug, Default, Deserialize, Serialize)]
                    pub struct #name {
                        #(#fields),*
                        
                        #[serde(default)]
                        _metadata: Metadata
                    }
                }
            } else {
                quote! {
                    #[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
                    pub struct #name {
                        #(#fields),*,

                        #[serde(default)]
                        _metadata: common::Metadata
                    }
                }
            };

            let fields_logic = self.get_fields_logic();
            let identifier_values_logic = self.get_identifier_values_logic();
            let values_logic = self.get_values_logic();

            quote! {
                #struct_definition
                
                impl #name {
                    fn get_fields() -> anyhow::Result<Vec<(String, String)>>{
                        #fields_logic
                    }
                    
                    fn get_values(&self) -> anyhow::Result<(std::collections::HashMap<&str, common::Value>)>{
                        #values_logic
                    }

                    fn get_identifier_values(&self) -> anyhow::Result<Vec<(&str, common::Value, &str)>>{
                        #identifier_values_logic
                    }

                    fn get_name() -> String{
                        return #pascal_case_name.to_owned()
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

    ///
    /// After the Expander types vector is filled by calling expand_definitons, the method
    /// formats all the newly created structs to a single token stream
    pub fn expand(&mut self, schema: &Schema) -> TokenStream {
        self.expand_definitions(schema);

        let types = self.types.iter().map(|t| &t.1);
        quote! {
            #( #types )*
        }
    }

    ///
    /// Iterates over the definitions in a given schema and calls the definition
    ///  creation process for each of them.
    fn expand_definitions(&mut self, schema: &Schema) {
        for (name, def) in &schema.definitions {
            let type_decl = self.expand_schema(name, def);
            println!("completed {}", &type_decl.to_string());
            self.types.push((name.to_string(), type_decl));
            
        }
    }

    pub fn expand_root(&mut self) -> TokenStream {
        self.expand(self.root)
    }
}
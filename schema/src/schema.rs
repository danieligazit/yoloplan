use {
    common::SimpleTypes,
};
pub type PositiveInteger = i64;
pub type PositiveIntegerDefault0 = serde_json::Value;
pub type SchemaArray = Vec<Schema>;


pub type StringArray = Vec<String>;
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub ref_: Option<String>,
    
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    
    #[serde(rename = "allOf")]
    pub all_of: Option<SchemaArray>,
    
    #[serde(rename = "anyOf")]
    pub any_of: Option<SchemaArray>,
    
    pub default: Option<common::Value>,
    
    #[serde(default)]
    pub definitions: ::std::collections::BTreeMap<String, Schema>,
    
    pub description: Option<String>,
    
    pub id: Option<String>,
    
    #[serde(default)]
    #[serde(with = "::one_or_many")]
    pub items: Vec<Schema>,
    
    #[serde(rename = "multipleOf")]
    pub multiple_of: Option<f64>,
    
    pub not: Option<Box<Schema>>,
    
    #[serde(rename = "oneOf")]
    pub one_of: Option<SchemaArray>,
    
    #[serde(default)]
    pub properties: ::std::collections::BTreeMap<String, Schema>,
    
    pub required: Option<StringArray>,
    
    pub title: Option<String>,
    
    #[serde(default)]
    #[serde(with = "::one_or_many")]
    #[serde(rename = "type")]
    pub type_: Vec<SimpleTypes>,
    
    #[serde(rename = "uniqueItems")]
    pub unique_items: Option<bool>,
    
    pub identify: Option<String>,
    pub aggreagte: Option<String>,
}
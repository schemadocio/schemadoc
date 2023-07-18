use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use indexmap::IndexMap;
use serde_json::Value;

use crate::core::{Either, MayBeRefCore, ReferenceDescriptor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Swagger2Ref {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl ReferenceDescriptor for Swagger2Ref {
    fn reference(&self) -> &str {
        &self.reference
    }
}

pub type MayBeRef200<T> = MayBeRefCore<T, Swagger2Ref>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDoc {
    pub url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    name: String,
    description: Option<String>,
    external_doc: Option<ExternalDoc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub r#in: String,
    pub description: Option<String>,
    pub required: Option<bool>,

    pub schema: Option<MayBeRef200<Schema>>,

    pub r#type: Option<Either<String, Vec<String>>>,
    pub format: Option<String>,
    pub allow_empty_value: Option<bool>,
    pub items: Option<MayBeRef200<Schema>>,
    pub collection_format: Option<String>,
    pub default: Option<Value>,
    pub maximum: Option<f32>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f32>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
    pub max_items: Option<usize>,
    pub min_items: Option<usize>,
    pub unique_items: Option<bool>,
    pub r#enum: Option<Vec<Value>>,
    pub multiple_of: Option<f32>,

    // Not in schema
    pub all_of: Option<Vec<MayBeRef200<Schema>>>,
    pub any_of: Option<Vec<MayBeRef200<Schema>>>,
    pub one_of: Option<Vec<MayBeRef200<Schema>>>,
    pub not: Option<Vec<MayBeRef200<Schema>>>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub format: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub default: Option<Value>,
    pub multiple_of: Option<f32>,

    pub maximum: Option<f32>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f32>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
    pub max_items: Option<usize>,
    pub min_items: Option<usize>,
    pub unique_items: Option<bool>,
    pub max_properties: Option<usize>,
    pub min_properties: Option<usize>,
    pub required: Option<Vec<String>>,
    pub r#enum: Option<Vec<Value>>,

    pub r#type: Option<Either<String, Vec<String>>>,
    pub items: Box<Option<MayBeRef200<Schema>>>,

    pub all_of: Option<Vec<MayBeRef200<Schema>>>,

    pub properties: Option<IndexMap<String, MayBeRef200<Schema>>>,
    pub additional_properties: Option<Either<bool, MayBeRef200<Schema>>>,

    pub discriminator: Option<String>,
    pub read_only: Option<bool>,
    // xml: Option<Xml>,
    pub external_docs: Option<ExternalDoc>,
    pub example: Option<Value>,

    // Not in schema
    pub one_of: Option<Vec<MayBeRef200<Schema>>>,
    pub any_of: Option<Vec<MayBeRef200<Schema>>>,
    pub not: Option<Vec<MayBeRef200<Schema>>>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub description: Option<String>,
    pub r#type: String,
    pub format: Option<String>,
    pub items: Option<MayBeRef200<Schema>>,

    pub collection_format: Option<String>,
    pub default: Option<Value>,
    pub maximum: Option<f32>,
    pub exclusive_maximum: Option<bool>,
    pub minimum: Option<f32>,
    pub exclusive_minimum: Option<bool>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
    pub max_items: Option<usize>,
    pub min_items: Option<usize>,
    pub unique_items: Option<bool>,
    pub r#enum: Option<Vec<Value>>,
    pub multiple_of: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    content_type: String,
    schema: MayBeRef200<Schema>,
    example: Option<String>,
    // examples
    // encoding: Vec<>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: Option<String>,
    pub schema: Option<MayBeRef200<Schema>>,
    pub headers: Option<IndexMap<String, Header>>,
    pub example: Option<Example>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Example {
    #[serde(flatten)]
    pub examples: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub description: Option<String>,

    pub external_docs: Option<ExternalDoc>,

    pub operation_id: Option<String>,

    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,

    pub parameters: Option<Vec<MayBeRef200<Parameter>>>,
    pub responses: IndexMap<String, MayBeRef200<Response>>,

    pub schemes: Option<Vec<String>>,
    pub deprecated: Option<bool>,
    // pub security: Option<Security>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Path {
    pub get: Option<Operation>,
    pub put: Option<Operation>,
    pub post: Option<Operation>,
    pub delete: Option<Operation>,
    pub options: Option<Operation>,
    pub head: Option<Operation>,
    pub patch: Option<Operation>,
    pub parameters: Option<Vec<MayBeRef200<Parameter>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct License {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: Option<String>,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,

    pub contact: Option<Contact>,
    pub license: Option<License>,

    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerV2 {
    pub swagger: String,
    pub info: Option<Info>,

    pub consumes: Option<Vec<String>>,
    pub produces: Option<Vec<String>>,

    pub paths: Option<IndexMap<String, Path>>,
    pub definitions: Option<IndexMap<String, Schema>>,
    pub parameters: Option<IndexMap<String, Parameter>>,
    pub responses: Option<IndexMap<String, Response>>,
}

impl SwaggerV2 {
    pub const fn id() -> &'static str {
        "SwaggerV2"
    }
}

#[cfg(test)]
mod tests {
    use crate::schema::Schema;

    #[test]
    fn test_schema_example_is_generic_value() {
        let s = r#"
        {
            "example": {
              "grant_type": "password",
              "password": "admin",
              "username": "admin"
            }
        }
"#;

        let _: Schema = serde_json::from_str(s).unwrap();
    }
}

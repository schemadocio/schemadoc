use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use indexmap::IndexMap;
use serde_json::Value;

use crate::core::{Either, Keyed, MayBeRefCore, ReferenceDescriptor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpSchemaRef {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl ReferenceDescriptor for HttpSchemaRef {
    fn reference(&self) -> &str {
        &self.reference
    }
}

pub type MayBeRef<T> = MayBeRefCore<T, HttpSchemaRef>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpSchema {
    pub version: String,

    pub schema_version: String,
    pub schema_source: String,
    pub schema_source_version: String,

    pub info: Option<Info>,
    pub servers: Option<Vec<Server>>,
    pub paths: Option<IndexMap<String, MayBeRef<Path>>>,
    pub components: Option<Components>,
    // TODO:
    // pub security:
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDoc>,
}

impl HttpSchema {
    pub fn schema_version() -> &'static str {
        "0.4.1"
    }
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
pub struct Server {
    pub url: Option<String>,
    pub description: Option<String>,
    pub variables: Option<IndexMap<String, ServerVariable>>,
}

impl Keyed<usize> for Server {
    fn key(&self, key: usize) -> String {
        self.url.clone().unwrap_or_else(|| key.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    pub r#enum: Option<Vec<String>>,
    pub default: Option<Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: Option<IndexMap<String, MayBeRef<Schema>>>,
    pub responses: Option<IndexMap<String, MayBeRef<Response>>>,
    pub parameters: Option<IndexMap<String, MayBeRef<Parameter>>>,
    pub examples: Option<IndexMap<String, MayBeRef<Example>>>,
    pub request_bodies: Option<IndexMap<String, MayBeRef<RequestBody>>>,
    pub headers: Option<IndexMap<String, MayBeRef<Header>>>,
    pub security_schemes: Option<IndexMap<String, MayBeRef<SecurityScheme>>>,
    pub links: Option<IndexMap<String, MayBeRef<Link>>>,
    // pub callbacks: Option<IndexMap<String, MayBeRef<T>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDoc {
    pub url: Option<String>,
    pub description: Option<String>,
}

impl Keyed<usize> for ExternalDoc {
    fn key(&self, _: usize) -> String {
        format!("{:?}", self.url)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    pub r#in: String,

    pub description: Option<String>,

    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,

    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,

    pub schema: Option<MayBeRef<Schema>>,

    pub examples: Option<IndexMap<String, MayBeRef<Value>>>,

    pub content: Option<IndexMap<String, MediaType>>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
}

impl Keyed<usize> for Parameter {
    fn key(&self, _: usize) -> String {
        format!("{}.{}", self.name, self.r#in)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: Option<IndexMap<String, MediaType>>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    pub schema: Option<MayBeRef<Schema>>,
    pub examples: Option<IndexMap<String, MayBeRef<Example>>>,
    pub encoding: Option<IndexMap<String, Encoding>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    pub content_type: Option<String>,
    pub headers: Option<IndexMap<String, MayBeRef<Header>>>,
    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub operation_ref: Option<String>,
    pub operation_id: Option<String>,
    pub parameters: Option<IndexMap<String, Value>>,
    pub request_body: Option<Value>,
    pub description: Option<String>,
    pub server: Option<Server>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub description: Option<String>,
    pub content: Option<IndexMap<String, MediaType>>,
    pub links: Option<IndexMap<String, MayBeRef<Link>>>,
    pub headers: Option<IndexMap<String, MayBeRef<Header>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Example {
    pub summary: Option<String>,
    pub description: Option<String>,
    pub value: Option<Value>,
    pub external_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Discriminator {
    pub property_name: Option<String>,
    pub mapping: Option<IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Xml {
    pub name: Option<String>,
    pub namespace: Option<String>,
    pub prefix: Option<String>,
    pub attribute: Option<bool>,
    pub wrapped: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SecurityScheme {
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub r#in: Option<String>,
    pub scheme: Option<String>,
    pub bearer_format: Option<String>,
    pub flows: Option<OAuthFlows>,
    pub open_id_connect_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlows {
    pub implicit: Option<OAuthFlow>,
    pub password: Option<OAuthFlow>,
    pub client_credentials: Option<OAuthFlow>,
    pub authorization_code: Option<OAuthFlow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlow {
    pub authorization_url: Option<String>,
    pub token_url: Option<String>,
    pub refresh_url: Option<String>,
    pub scopes: Option<IndexMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: Option<String>,
    pub description: Option<String>,
    pub external_doc: Option<ExternalDoc>,
}

impl Keyed<usize> for Tag {
    fn key(&self, _: usize) -> String {
        format!("{:?}", self.name)
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    // https://json-schema.org/draft/2020-12/json-schema-core.html

    // #[serde(rename = "$schema")]
    // schema_: Option<String>,

    // #[serde(rename = "id")]
    // id_: Option<String>,
    // #[serde(rename = "$anchor")]
    // anchor_: Option<String>,
    //
    // #[serde(rename = "$dynamicAnchor")]
    // dynamic_anchor_: Option<String>,
    //
    // #[serde(rename = "$dynamicRef")]
    // dynamic_ref_: Option<String>,
    //
    // #[serde(rename = "$def")]
    // defs_: Option<IndexMap<String, Schema>>,
    //
    // #[serde(rename = "$comment")]
    // comment_: Option<IndexMap<String, Schema>>,

    // if
    // then
    // else
    // dependentSchemas

    // contains
    pub title: Option<String>,
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

    // r#const: Option<Vec>,

    // pub contains: Box<Option<MayBeRef<Schema>>>
    // pub min_contains: Option<usize>,
    // pub max_contains: Option<bool>,

    // dependentRequired

    // https://json-schema.org/draft/2020-12/json-schema-validation.html#:~:text=to%20these%20keywords.-,8.3.,-contentEncoding
    // content_encoding
    // contentMediaType
    // contentSchema
    pub r#type: Option<Either<String, Vec<String>>>,
    pub all_of: Option<Vec<MayBeRef<Schema>>>,
    pub one_of: Option<Vec<MayBeRef<Schema>>>,
    pub any_of: Option<Vec<MayBeRef<Schema>>>,
    pub not: Option<Vec<MayBeRef<Schema>>>,

    pub items: Box<Option<MayBeRef<Schema>>>,

    // prefix_items: Option<Vec<MayBeRef<Schema>>>,

    // unevaluated_items: Box<Option<MayBeRef<Schema>>>
    pub properties: Option<IndexMap<String, MayBeRef<Schema>>>,
    pub additional_properties: Option<Either<bool, MayBeRef<Schema>>>, // TODO: Can be Bool in 3.1??
    // pattern_properties: Option<IndexMap<String, MayBeRef<Schema>>>,

    // property_names: Box<Option<MayBeRef<Schema>>>,

    // unevaluated_properties: Box<Option<MayBeRef<Schema>>>
    pub description: Option<String>,
    pub format: Option<String>,
    pub default: Option<Value>,

    pub discriminator: Option<Discriminator>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub xml: Option<Xml>,
    pub external_docs: Option<ExternalDoc>,
    pub example: Option<Value>,

    // examples: Option<Vec<Value>>,
    pub deprecated: Option<bool>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
}

impl Keyed<usize> for Schema {
    fn key(&self, idx: usize) -> String {
        if let Some(kind) = &self.r#type {
            if let Some(title) = &self.title {
                return format!("{kind:?}{title:?}");
            }

            // if let Some(items) = &*self.items {
            //     return items.key(idx);
            // }

            // return kind.to_string();
        }

        idx.to_string()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub description: Option<String>,

    pub required: Option<bool>,
    pub deprecated: Option<bool>,
    pub allow_empty_value: Option<bool>,

    pub style: Option<String>,
    pub explode: Option<bool>,
    pub allow_reserved: Option<bool>,

    pub schema: Option<MayBeRef<Schema>>,

    pub examples: Option<IndexMap<String, MayBeRef<Value>>>,

    pub content: Option<IndexMap<String, MediaType>>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub description: Option<String>,

    pub external_docs: Option<ExternalDoc>,

    pub operation_id: Option<String>,

    pub responses: Option<IndexMap<String, MayBeRef<Response>>>,

    pub request_body: Option<MayBeRef<RequestBody>>,

    pub servers: Option<Vec<Server>>,
    pub parameters: Option<Vec<MayBeRef<Parameter>>>,

    pub security: Option<Vec<IndexMap<String, Vec<String>>>>,

    // TODO:
    // pub callbacks
    pub deprecated: Option<bool>,
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
    pub trace: Option<Operation>,

    pub servers: Option<Vec<Server>>,

    pub summary: Option<String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::schema::*;

    #[test]
    fn check_operation() {
        let op_def = r#"{
      "post": {
        "tags": ["Nodes"],
        "summary": "Export Xlsx Template",
        "description": "Generate XLSX-template for aggregated node data editing",
        "operationId": "gen_xlsx_aggr_node",
        "parameters": [
          {
            "required": true,
            "schema": { "title": "Path", "type": "string" },
            "name": "path",
            "in": "path"
          },
          {
            "required": false,
            "schema": { "title": "Update Sender", "type": "string" },
            "name": "update_sender",
            "in": "query"
          },
          {
            "required": false,
            "schema": { "title": "Force", "type": "boolean", "default": false },
            "name": "force",
            "in": "query"
          },
          {
            "required": false,
            "schema": { "title": "Compound Amount", "type": "integer" },
            "name": "compound_amount",
            "in": "query"
          },
          {
            "required": false,
            "schema": {
              "allOf": [{ "$ref": "/components/schemas/ExportFmt" }],
              "default": "xlsx"
            },
            "name": "export_format",
            "in": "query"
          }
        ],
        "requestBody": {
          "content": {
            "application/json": {
              "schema": {
                "$ref": "/components/schemas/Body_export_xlsx_template_api_v2_nodes__path__template_generate__post"
              }
            }
          }
        },
        "responses": {
          "200": {
            "description": "Successful Response",
            "content": {
              "application/json": { "schema": {} },
              "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet": {}
            }
          },
          "422": {
            "description": "Validation Error",
            "content": {
              "application/json": {
                "schema": { "$ref": "/components/schemas/HTTPValidationError" }
              }
            }
          }
        },
        "security": [{ "OAuth2PasswordBearer": [] }]
      }
    }"#;

        let _: Path = serde_json::from_str(op_def).unwrap();
    }

    #[test]
    fn check_schema_additional_properties() {
        let op_def = r#"{
            "title": "AdditionalProperties",
            "type": "object",
            "additionalProperties": {
              "$ref": "/components/schemas/AdditionalProperties"
            }
          }"#;

        let op: Schema = serde_json::from_str(op_def).unwrap();
        assert!(matches!(op.additional_properties, Some(Either::Right(_))));

        let op_def = r#"{
            "title": "AdditionalProperties",
            "type": "object",
            "additionalProperties": false
          }"#;

        let op: Schema = serde_json::from_str(op_def).unwrap();
        assert!(matches!(op.additional_properties, Some(Either::Left(_))));

        let sc_def = r#"
        {
        "type": "object",
        "discriminator": { "propertyName": "type" },
        "properties": {
          "type": {
            "type": "string",
            "description": "The type of context being attached to the entity.",
            "enum": ["link", "image"]
          }
        },
        "required": ["type"]
      }
        "#;
        let op: Schema = serde_json::from_str(sc_def).unwrap();
        assert!(matches!(op.discriminator, Some(_)))
    }
}

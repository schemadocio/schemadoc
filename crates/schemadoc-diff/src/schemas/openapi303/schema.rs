use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use indexmap::IndexMap;
use serde_json::Value;

use crate::core::{DiffResult, Either, MayBeRefCore, ReferenceDescriptor};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi303Ref {
    #[serde(rename = "$ref")]
    pub reference: String,
}

impl ReferenceDescriptor for OpenApi303Ref {
    fn reference(&self) -> &str {
        &self.reference
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApi303RefDiff {
    #[serde(rename = "$ref")]
    pub reference: DiffResult<String>,
}

impl ReferenceDescriptor for OpenApi303RefDiff {
    fn reference(&self) -> &str {
        self.reference.get().expect("Reference diff cannot be null")
    }
}

pub type MayBeRef303<T> = MayBeRefCore<T, OpenApi303Ref>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApi303 {
    pub openapi: String,
    pub info: Option<Info>,
    pub servers: Option<Vec<Server>>,
    pub paths: Option<IndexMap<String, MayBeRef303<Path>>>,
    pub components: Option<Components>,
    // TODO:
    // pub security:
    pub tags: Option<Vec<Tag>>,
    pub external_docs: Option<ExternalDoc>,
}

impl OpenApi303 {
    pub const fn id() -> &'static str {
        "OpenApi303"
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerVariable {
    pub r#enum: Option<Vec<String>>,
    pub default: Option<Value>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Components {
    pub schemas: Option<IndexMap<String, MayBeRef303<Schema>>>,
    pub responses: Option<IndexMap<String, MayBeRef303<Response>>>,
    pub parameters: Option<IndexMap<String, MayBeRef303<Parameter>>>,
    pub examples: Option<IndexMap<String, MayBeRef303<Example>>>,
    pub request_bodies: Option<IndexMap<String, MayBeRef303<RequestBody>>>,
    pub headers: Option<IndexMap<String, MayBeRef303<Header>>>,
    pub security_schemes:
        Option<IndexMap<String, MayBeRef303<SecurityScheme>>>,
    pub links: Option<IndexMap<String, MayBeRef303<Link>>>,
    // pub callbacks: Option<HashMap<String, MayBeRef303<Header>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalDoc {
    pub url: Option<String>,
    pub description: Option<String>,
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

    pub schema: Option<MayBeRef303<Schema>>,

    pub example: Option<Value>,
    pub examples: Option<IndexMap<String, MayBeRef303<Value>>>,

    pub content: Option<IndexMap<String, MediaType>>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: Option<IndexMap<String, MediaType>>,
    pub required: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaType {
    pub schema: Option<MayBeRef303<Schema>>,
    pub example: Option<Value>,

    pub examples: Option<IndexMap<String, MayBeRef303<Example>>>,
    pub encoding: Option<IndexMap<String, Encoding>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encoding {
    pub content_type: Option<String>,
    pub headers: Option<IndexMap<String, MayBeRef303<Header>>>,
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
    pub headers: Option<IndexMap<String, MayBeRef303<Header>>>,
    pub content: Option<IndexMap<String, MediaType>>,

    pub links: Option<IndexMap<String, MayBeRef303<Link>>>,
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
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

    // Allow array of types, which is not corresponds to the schema version
    pub r#type: Option<Either<String, Vec<String>>>,
    pub all_of: Option<Vec<MayBeRef303<Schema>>>,
    pub one_of: Option<Vec<MayBeRef303<Schema>>>,
    pub any_of: Option<Vec<MayBeRef303<Schema>>>,
    pub not: Option<Vec<MayBeRef303<Schema>>>,

    pub items: Box<Option<MayBeRef303<Schema>>>,
    pub properties: Option<IndexMap<String, MayBeRef303<Schema>>>,
    pub additional_properties: Option<Either<bool, MayBeRef303<Schema>>>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub default: Option<Value>,

    pub nullable: Option<bool>,
    pub discriminator: Option<Discriminator>,
    pub read_only: Option<bool>,
    pub write_only: Option<bool>,
    pub xml: Option<Xml>,
    pub external_docs: Option<ExternalDoc>,
    pub example: Option<Value>,
    pub deprecated: Option<bool>,

    #[serde(flatten)]
    pub custom_fields: IndexMap<String, Value>,
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

    pub schema: Option<MayBeRef303<Schema>>,

    pub example: Option<Value>,
    pub examples: Option<IndexMap<String, MayBeRef303<Value>>>,

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

    pub parameters: Option<Vec<MayBeRef303<Parameter>>>,

    pub responses: Option<IndexMap<String, MayBeRef303<Response>>>,

    pub request_body: Option<MayBeRef303<RequestBody>>,
    pub servers: Option<Vec<Server>>,

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
    pub parameters: Option<Vec<MayBeRef303<Parameter>>>,

    pub summary: Option<String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::core::Either;
    use crate::schemas::openapi310::schema::*;

    #[test]
    fn check_operation() {
        let op_def = r#"{
      "post": {
        "tags": ["Nodes"],
        "summary": "Export Xlsx Template",
        "description": "Generate XLSX-template for aggregated node data editing",
        "operationId": "export_xlsx_template_api_v2_nodes__path__template_generate__post",
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

    #[test]
    fn check_strange_thing() {
        let op_def = r#"
 {
                "description": "Response",
                "content": {
                  "application/json": {
                    "schema": {
                      "allOf": [
                        {
                          "type": "object",
                          "properties": {
                            "client_id": {
                              "type": "string"
                            },
                            "client_secret": {
                              "type": "string"
                            },
                            "webhook_secret": {
                              "type": [
                                "string",
                                "null"
                              ]
                            },
                            "pem": {
                              "type": "string"
                            }
                          },
                          "required": [
                            "client_id",
                            "client_secret",
                            "webhook_secret",
                            "pem"
                          ],
                          "additionalProperties": true
                        }
                      ]
                    },
                    "examples": {
                    }
                  }
                }
              }
        "#;

        let source_de = &mut serde_json::Deserializer::from_str(op_def);
        let result: Result<Response, _> =
            serde_path_to_error::deserialize(source_de);
        let _ = result.map_err(|err| {
            let path = err.path().to_string();
            dbg!(path, err)
        });
    }

    #[test]
    fn check_additional_props() {
        let schema_def = r#"
        {
            "type": "array",
            "items": {
              "maxItems": 4,
              "minItems": 4,
              "type": "array",
              "items": {
                  "type": "string"
                }
            }
          }
        "#;

        let _: Schema = serde_json::from_str(schema_def).unwrap();
    }
}

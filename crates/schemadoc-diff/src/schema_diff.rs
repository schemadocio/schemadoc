use serde::{Deserialize, Serialize};
use std::fmt::Debug;

use schemadoc_diff_derive::{Diff, DiffOwnChanges, Empty};
use serde_json::Value;

use crate::core::{
    Diff, DiffContext, DiffResult, EitherDiff, Empty, MapDiff,
    MayBeRefCoreDiff, Referencable, VecDiff,
};

use crate::schema::HttpSchemaRef;
use crate::schema_diff_utils::{PathsMapPathResolver, TypeVecDiffSorter};

pub type MayBeRefDiff<T> = MayBeRefCoreDiff<T, HttpSchemaRef>;

fn check_custom_fields(fields: &DiffResult<MapDiff<Value>>) -> bool {
    if fields.is_same() {
        let fields = fields.get().expect("Must always be set");
        fields.is_empty()
    } else {
        fields.is_none()
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct HttpSchemaDiff {
    pub version: String,

    pub schema_version: String,
    pub schema_source: String,
    pub schema_source_version: String,

    pub info: DiffResult<InfoDiff>,
    pub servers: DiffResult<VecDiff<ServerDiff>>,
    pub paths:
        DiffResult<MapDiff<MayBeRefDiff<PathDiff>, PathsMapPathResolver>>,
    pub components: DiffResult<ComponentsDiff>,
    pub tags: DiffResult<VecDiff<TagDiff>>,
    pub external_docs: DiffResult<ExternalDocDiff>,
}

impl HttpSchemaDiff {
    pub fn get_diff_version(&self) -> String {
        format!(
            "{}-{}-{}",
            self.schema_version,
            self.schema_source,
            self.schema_source_version
        )
    }
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct InfoDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub title: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub terms_of_service: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub contact: DiffResult<ContactDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub license: DiffResult<LicenseDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub version: DiffResult<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
pub struct ContactDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub name: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub url: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub email: DiffResult<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
pub struct LicenseDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub name: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub url: DiffResult<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
pub struct ServerDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub url: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub variables: DiffResult<MapDiff<ServerVariableDiff>>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
pub struct ServerVariableDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub r#enum: DiffResult<VecDiff<String>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub default: DiffResult<Value>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct ComponentsDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub schemas: DiffResult<MapDiff<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub responses: DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub parameters: DiffResult<MapDiff<MayBeRefDiff<ParameterDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub examples: DiffResult<MapDiff<MayBeRefDiff<ExampleDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub request_bodies: DiffResult<MapDiff<MayBeRefDiff<RequestBodyDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub headers: DiffResult<MapDiff<MayBeRefDiff<HeaderDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub security_schemes:
        DiffResult<MapDiff<MayBeRefDiff<SecuritySchemeDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub links: DiffResult<MapDiff<MayBeRefDiff<LinkDiff>>>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub url: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct ParameterDiff {
    pub name: String,
    pub r#in: String,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub required: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub deprecated: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub allow_empty_value: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub style: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub explode: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub allow_reserved: DiffResult<bool>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub schema: DiffResult<MayBeRefDiff<SchemaDiff>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub examples: DiffResult<MapDiff<MayBeRefDiff<Value>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub content: DiffResult<MapDiff<MediaTypeDiff>>,

    #[serde(skip_serializing_if = "check_custom_fields")]
    pub custom_fields: DiffResult<MapDiff<Value>>,
}

impl Referencable for ParameterDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct RequestBodyDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub content: DiffResult<MapDiff<MediaTypeDiff>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub required: DiffResult<bool>,
}

impl Referencable for RequestBodyDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct MediaTypeDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub schema: DiffResult<MayBeRefDiff<SchemaDiff>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub examples: DiffResult<MapDiff<MayBeRefDiff<ExampleDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub encoding: DiffResult<MapDiff<EncodingDiff>>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct EncodingDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub content_type: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub headers: DiffResult<MapDiff<MayBeRefDiff<HeaderDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub style: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub explode: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub allow_reserved: DiffResult<bool>,
}

impl Referencable for EncodingDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct LinkDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub operation_ref: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub operation_id: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub parameters: DiffResult<MapDiff<Value>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub request_body: DiffResult<Value>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub server: DiffResult<ServerDiff>,
}

impl Referencable for LinkDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
pub struct ResponseDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub content: DiffResult<MapDiff<MediaTypeDiff>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub links: DiffResult<MapDiff<MayBeRefDiff<LinkDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub headers: DiffResult<MapDiff<MayBeRefDiff<HeaderDiff>>>,
}

impl Referencable for ResponseDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct ExampleDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub summary: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub value: DiffResult<Value>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub external_value: DiffResult<String>,
}

impl Referencable for ExampleDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct DiscriminatorDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub property_name: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub mapping: DiffResult<MapDiff<String>>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct XmlDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub name: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub namespace: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub prefix: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub attribute: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub wrapped: DiffResult<bool>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySchemeDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub r#type: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub name: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub r#in: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub scheme: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub bearer_format: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub flows: DiffResult<OAuthFlowsDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub open_id_connect_url: DiffResult<String>,
}

impl Referencable for SecuritySchemeDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowsDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub implicit: DiffResult<OAuthFlowDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub password: DiffResult<OAuthFlowDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub client_credentials: DiffResult<OAuthFlowDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub authorization_code: DiffResult<OAuthFlowDiff>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub authorization_url: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub token_url: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub refresh_url: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub scopes: DiffResult<MapDiff<String>>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct TagDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub name: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub external_doc: DiffResult<ExternalDocDiff>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct SchemaDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub title: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub multiple_of: DiffResult<f32>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub maximum: DiffResult<f32>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub exclusive_maximum: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub minimum: DiffResult<f32>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub exclusive_minimum: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub max_length: DiffResult<usize>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub min_length: DiffResult<usize>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub pattern: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub max_items: DiffResult<usize>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub min_items: DiffResult<usize>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub unique_items: DiffResult<bool>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub max_properties: DiffResult<usize>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub min_properties: DiffResult<usize>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub required: DiffResult<VecDiff<String>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub r#enum: DiffResult<VecDiff<Value>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub r#type:
        DiffResult<EitherDiff<String, VecDiff<String, TypeVecDiffSorter>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub all_of: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub one_of: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub any_of: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub not: DiffResult<VecDiff<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub items: Box<DiffResult<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub properties: DiffResult<MapDiff<MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub additional_properties:
        DiffResult<EitherDiff<bool, MayBeRefDiff<SchemaDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub format: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub default: DiffResult<Value>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub discriminator: DiffResult<DiscriminatorDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub read_only: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub write_only: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub xml: DiffResult<XmlDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub external_docs: DiffResult<ExternalDocDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub example: DiffResult<Value>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub deprecated: DiffResult<bool>,

    #[serde(skip_serializing_if = "check_custom_fields")]
    pub custom_fields: DiffResult<MapDiff<Value>>,
}

impl Referencable for SchemaDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct HeaderDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub required: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub deprecated: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub allow_empty_value: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub style: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub explode: DiffResult<bool>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub allow_reserved: DiffResult<bool>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub schema: DiffResult<MayBeRefDiff<SchemaDiff>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub examples: DiffResult<MapDiff<MayBeRefDiff<Value>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub content: DiffResult<MapDiff<MediaTypeDiff>>,

    #[serde(skip_serializing_if = "check_custom_fields")]
    pub custom_fields: DiffResult<MapDiff<Value>>,
}

impl Referencable for HeaderDiff {}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
#[serde(rename_all = "camelCase")]
pub struct OperationDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub tags: DiffResult<VecDiff<String>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub summary: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub external_docs: DiffResult<ExternalDocDiff>,

    pub operation_id: DiffResult<String>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub parameters: DiffResult<VecDiff<MayBeRefDiff<ParameterDiff>>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub responses: DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub request_body: DiffResult<MayBeRefDiff<RequestBodyDiff>>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub servers: DiffResult<VecDiff<ServerDiff>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub security: DiffResult<VecDiff<MapDiff<VecDiff<String>>>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub deprecated: DiffResult<bool>,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Empty, Diff, DiffOwnChanges,
)]
pub struct PathDiff {
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub get: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub put: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub post: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub delete: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub options: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub head: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub patch: DiffResult<OperationDiff>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub trace: DiffResult<OperationDiff>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub servers: DiffResult<VecDiff<ServerDiff>>,

    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub summary: DiffResult<String>,
    #[serde(skip_serializing_if = "DiffResult::is_none")]
    pub description: DiffResult<String>,
}

impl Referencable for PathDiff {}

pub(crate) fn deref_schema_diff<'a>(
    diff: &'a HttpSchemaDiff,
    may_be_ref: &'a MayBeRefDiff<SchemaDiff>,
) -> Option<&'a DiffResult<SchemaDiff>> {
    match may_be_ref {
        MayBeRefDiff::Value(value) => return Some(value),
        MayBeRefDiff::Ref(value) => {
            if value.reference.starts_with("#/components/schemas/") {
                let key = value.reference.replace("#/components/schemas/", "");
                if diff.components.exists() {
                    if let Some(components) = diff.components.get() {
                        if let Some(schemas) = components.schemas.get() {
                            if let Some(may_be_schema) = schemas.get(&key) {
                                if let Some(MayBeRefDiff::Value(schema)) =
                                    may_be_schema.get()
                                {
                                    return Some(schema);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    None
}

pub(crate) fn deref_parameter_diff<'a>(
    diff: &'a HttpSchemaDiff,
    may_be_ref: &'a MayBeRefDiff<ParameterDiff>,
) -> Option<&'a DiffResult<ParameterDiff>> {
    match may_be_ref {
        MayBeRefDiff::Value(value) => return Some(value),
        MayBeRefDiff::Ref(value) => {
            if value.reference.starts_with("#/components/parameters/") {
                let key =
                    value.reference.replace("#/components/parameters/", "");
                if diff.components.exists() {
                    if let Some(components) = diff.components.get() {
                        if let Some(parameters) = components.parameters.get() {
                            if let Some(may_be_parameter) =
                                parameters.get(&key)
                            {
                                if let Some(MayBeRefDiff::Value(parameter)) =
                                    may_be_parameter.get()
                                {
                                    return Some(parameter);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    None
}

pub(crate) fn deref_request_body_diff<'a>(
    diff: &'a HttpSchemaDiff,
    may_be_ref: &'a MayBeRefDiff<RequestBodyDiff>,
) -> Option<&'a DiffResult<RequestBodyDiff>> {
    match may_be_ref {
        MayBeRefDiff::Value(value) => return Some(value),
        MayBeRefDiff::Ref(value) => {
            if value.reference.starts_with("#/components/requestBodies/") {
                let key =
                    value.reference.replace("#/components/requestBodies/", "");
                if diff.components.exists() {
                    if let Some(components) = diff.components.get() {
                        if let Some(request_bodies) =
                            components.request_bodies.get()
                        {
                            if let Some(request_body) =
                                request_bodies.get(&key)
                            {
                                if let Some(MayBeRefDiff::Value(
                                    request_body,
                                )) = request_body.get()
                                {
                                    return Some(request_body);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    None
}

pub(crate) fn deref_response_diff<'a>(
    diff: &'a HttpSchemaDiff,
    may_be_ref: &'a MayBeRefDiff<ResponseDiff>,
) -> Option<&'a DiffResult<ResponseDiff>> {
    match may_be_ref {
        MayBeRefDiff::Value(value) => return Some(value),
        MayBeRefDiff::Ref(value) => {
            if value.reference.starts_with("#/components/responses/") {
                let key =
                    value.reference.replace("#/components/responses/", "");
                if diff.components.exists() {
                    if let Some(components) = diff.components.get() {
                        if let Some(responses) = components.responses.get() {
                            if let Some(response) = responses.get(&key) {
                                if let Some(MayBeRefDiff::Value(response)) =
                                    response.get()
                                {
                                    return Some(response);
                                }
                            }
                        }
                    }
                }
            }
        }
    };
    None
}

#[cfg(test)]
mod tests {
    use crate::core::Either;
    use crate::schema::*;

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
            "enum": ["link", "text", "image"]
          }
        },
        "required": ["type"]
      }
        "#;
        let op: Schema = serde_json::from_str(sc_def).unwrap();
        assert!(matches!(op.discriminator, Some(_)))
    }
}

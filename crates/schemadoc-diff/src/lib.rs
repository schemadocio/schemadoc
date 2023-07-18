pub mod checker;
pub mod context;
pub mod core;
pub mod diff_own_changes;
pub mod diff_result_type;
pub mod error;
pub mod exporters;
pub mod path_pointer;
pub mod schema;
pub mod schema_diff;
pub mod schema_diff_utils;
pub mod schemas;
pub mod visitor;
pub mod visitors;

use crate::context::HttpSchemaDiffContext;
use crate::core::{Diff, DiffResult};
use once_cell::sync::Lazy;
use std::rc::Rc;
use tracing::info;

use crate::error::Error;
use crate::schema::HttpSchema;
use crate::schema_diff::HttpSchemaDiff;
use crate::schemas::openapi303::schema::OpenApi303;
use crate::schemas::openapi310::schema::OpenApi310;
use crate::schemas::swagger2::schema::SwaggerV2;

use crate::schemas::openapi303::converter::VERSION as OPENAPI303_CONVERTER_VERSION;
use crate::schemas::openapi310::converter::VERSION as OPENAPI310_CONVERTER_VERSION;
use crate::schemas::swagger2::converter::VERSION as SWAGGERV2_CONVERTER_VERSION;

pub static VERSIONS: Lazy<Vec<String>> = Lazy::new(|| {
    let schema_version = schema::HttpSchema::schema_version();
    vec![
        format!(
            "{}-{}-{}",
            schema_version,
            SwaggerV2::id(),
            SWAGGERV2_CONVERTER_VERSION
        ),
        format!(
            "{}-{}-{}",
            schema_version,
            OpenApi303::id(),
            OPENAPI303_CONVERTER_VERSION
        ),
        format!(
            "{}-{}-{}",
            schema_version,
            OpenApi310::id(),
            OPENAPI310_CONVERTER_VERSION
        ),
    ]
});

pub fn is_current_diff_version(diff_version: &str) -> bool {
    VERSIONS.iter().any(|x| x == diff_version)
}

#[tracing::instrument(skip_all, fields(src.schema.decoder, src.schema.version, tgt.schema.decoder, tgt.schema.version))]
pub fn try_deserialize_schema(
    src_content: &str,
    tgt_content: &str,
) -> Result<(schema::HttpSchema, schema::HttpSchema), Error> {
    let source: schema::HttpSchema = if let Ok(schema) =
        serde_json::from_str::<OpenApi310>(src_content)
    {
        Ok(schema.into())
    } else if let Ok(schema) = serde_json::from_str::<OpenApi303>(src_content)
    {
        Ok(schema.into())
    } else if let Ok(schema) = serde_json::from_str::<SwaggerV2>(src_content) {
        Ok(schema.into())
    } else {
        Err(Error::InvalidSourceSchema)
    }?;

    info!(
        src.schema.version = &source.version,
        src.schema.decoder = &source.schema_source
    );

    let target: schema::HttpSchema = if let Ok(schema) =
        serde_json::from_str::<OpenApi310>(tgt_content)
    {
        Ok(schema.into())
    } else if let Ok(schema) = serde_json::from_str::<OpenApi303>(tgt_content)
    {
        Ok(schema.into())
    } else if let Ok(schema) = serde_json::from_str::<SwaggerV2>(tgt_content) {
        Ok(schema.into())
    } else {
        Err(Error::InvalidTargetSchema)
    }?;

    info!(
        tgt.schema.version = &target.version,
        tgt.schema.decoder = &target.schema_source
    );

    Ok((source, target))
}

pub fn get_schema_diff(
    src_schema: HttpSchema,
    tgt_schema: HttpSchema,
) -> DiffResult<HttpSchemaDiff> {
    let src = Rc::new(src_schema);
    let tgt = Rc::new(tgt_schema);

    let context = HttpSchemaDiffContext::new(Rc::clone(&src), Rc::clone(&tgt));
    src.diff(Some(&*tgt), &context)
}

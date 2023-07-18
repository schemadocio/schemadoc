use crate::core::{Either, ReferenceDescriptor};
use indexmap::IndexMap;

use crate::schema as core;

use crate::schemas::swagger2::context::*;
use crate::schemas::swagger2::schema::*;

pub const VERSION: &str = "0.1.0";

struct ConvertContext<'a> {
    pub consumes: &'a Option<Vec<String>>,
    pub produces: &'a Option<Vec<String>>,
    // pub definitions: &'a Option<IndexMap<String, V2Schema>>,
    pub parameters: &'a Option<IndexMap<String, Parameter>>,
    pub responses: &'a Option<IndexMap<String, Response>>,
}

impl From<SwaggerV2> for core::HttpSchema {
    fn from(spec: SwaggerV2) -> Self {
        let paths = {
            let context = ConvertContext {
                consumes: &spec.consumes,
                produces: &spec.produces,
                // definitions: &spec.definitions,
                parameters: &spec.parameters,
                responses: &spec.responses,
            };

            spec.paths.map(|paths| convert_paths(paths, &context))
        };

        let parameters = spec.parameters.map(|parameters| {
            parameters
                .into_iter()
                .map(|(key, parameter)| {
                    (key, core::MayBeRef::Value(convert_parameter(parameter)))
                })
                .collect::<IndexMap<_, _>>()
        });

        let responses = spec.responses.map(|responses| {
            responses
                .into_iter()
                .map(|(key, response)| {
                    (
                        key,
                        core::MayBeRef::Value(convert_response(
                            response,
                            &["application/json".to_string()],
                        )),
                    )
                })
                .collect::<IndexMap<_, _>>()
        });

        let schemas = spec.definitions.map(|definitions| {
            definitions
                .into_iter()
                .map(|(key, schema)| {
                    (key, core::MayBeRef::Value(convert_schema(schema)))
                })
                .collect::<IndexMap<_, _>>()
        });

        let components = core::Components {
            schemas,
            responses,
            parameters,
            examples: None,
            request_bodies: None,
            headers: None,
            security_schemes: None,
            links: None,
        };

        let info = spec.info.map(convert_info);

        core::HttpSchema {
            version: spec.swagger,

            schema_source: SwaggerV2::id().to_owned(),
            schema_source_version: VERSION.to_owned(),
            schema_version: core::HttpSchema::schema_version().to_owned(),

            info,
            servers: None,
            paths,
            components: Some(components),
            tags: None,
            external_docs: None,
        }
    }
}

fn convert_paths(
    paths: IndexMap<String, Path>,
    context: &ConvertContext,
) -> IndexMap<String, core::MayBeRef<core::Path>> {
    paths
        .into_iter()
        .map(|(key, path)| {
            (key, core::MayBeRef::Value(convert_path(path, context)))
        })
        .collect()
}

fn convert_path(path: Path, context: &ConvertContext) -> core::Path {
    let parameters = &path.parameters;

    core::Path {
        get: path
            .get
            .map(|op| convert_operation(op, parameters, context)),
        put: path
            .put
            .map(|op| convert_operation(op, parameters, context)),
        post: path
            .post
            .map(|op| convert_operation(op, parameters, context)),
        delete: path
            .delete
            .map(|op| convert_operation(op, parameters, context)),
        options: path
            .options
            .map(|op| convert_operation(op, parameters, context)),
        head: path
            .head
            .map(|op| convert_operation(op, parameters, context)),
        patch: path
            .patch
            .map(|op| convert_operation(op, parameters, context)),
        trace: None,
        servers: None,
        summary: None,
        description: None,
    }
}

fn convert_info(info: Info) -> core::Info {
    core::Info {
        title: info.title,
        version: info.version,
        description: info.description,
        terms_of_service: info.terms_of_service,
        contact: info.contact.map(convert_contact),
        license: info.license.map(convert_license),
    }
}

fn convert_contact(contact: Contact) -> core::Contact {
    core::Contact {
        url: contact.url,
        name: contact.name,
        email: contact.email,
    }
}

fn convert_license(license: License) -> core::License {
    core::License {
        url: license.url,
        name: license.name,
    }
}

fn merge_parameters(
    components: &Option<IndexMap<String, Parameter>>,
    parameters_refs: Option<Vec<MayBeRef200<Parameter>>>,
    path_parameters_refs: &Option<Vec<MayBeRef200<Parameter>>>,
) -> Vec<(String, String, MayBeRef200<Parameter>)> {
    let mut parameters: Vec<MayBeRef200<Parameter>> = Vec::new();

    if let Some(parameters_refs) = parameters_refs {
        parameters.extend(parameters_refs);
    }

    if let Some(parameters_refs) = path_parameters_refs {
        parameters.extend(parameters_refs.clone());
    }

    let mut visited = Vec::new();

    let mut result = Vec::with_capacity(parameters.len());

    for may_be_parameter in parameters {
        let key = match &may_be_parameter {
            MayBeRef200::Ref(value) => {
                if let Some(parameter) =
                    deref_parameter(components, value.reference())
                {
                    (parameter.name.clone(), parameter.r#in.clone())
                } else {
                    // TODO: handle the case where ref not found
                    continue;
                }
            }
            MayBeRef200::Value(value) => {
                (value.name.clone(), value.r#in.clone())
            }
        };

        if visited.contains(&key) {
            continue;
        }

        result.push((key.0.clone(), key.1.clone(), may_be_parameter));

        visited.push(key);
    }

    result
}

fn convert_operation(
    operation: Operation,
    path_parameters: &Option<Vec<MayBeRef200<Parameter>>>,
    context: &ConvertContext,
) -> core::Operation {
    let merged_parameters = merge_parameters(
        context.parameters, // TODO: pass components
        operation.parameters,
        path_parameters,
    );

    if operation.operation_id == Some("workspace-regions_create".to_string()) {
        println!("found");
    }

    let mut body_parameters: Vec<_> = Vec::with_capacity(1);
    let mut parameters: Vec<_> = Vec::with_capacity(merged_parameters.len());

    for (_name, loc, parameter) in merged_parameters {
        if loc == "body" || loc == "formData" {
            body_parameters.push(parameter)
        } else {
            parameters.push(convert_parameter_ref(parameter))
        }
    }

    let request_body =
        convert_to_request_body(body_parameters, &operation.consumes, context);

    let mut produces = operation
        .produces
        .unwrap_or_else(|| context.produces.clone().unwrap_or_default());

    if produces.is_empty() {
        produces.push("application/json".to_string())
    }

    let unref = context
        .produces
        .as_ref()
        .map(|global_produces| global_produces != &produces)
        .unwrap_or_else(|| produces != vec!["application/json".to_string()]);

    let responses = operation
        .responses
        .into_iter()
        .map(|(code, response_ref)| {
            (
                code,
                convert_response_ref(response_ref, &produces, context, unref),
            )
        })
        .collect();

    let external_docs = operation.external_docs.map(convert_external_docs);

    core::Operation {
        tags: Some(operation.tags),
        summary: operation.summary,
        description: operation.description,
        external_docs,
        operation_id: operation.operation_id,
        responses: Some(responses),
        request_body,
        servers: None,
        parameters: Some(parameters),
        security: None, // TODO: add security policies
        deprecated: operation.deprecated,
    }
}

fn convert_to_request_body(
    parameters: Vec<MayBeRef200<Parameter>>,
    consumes: &Option<Vec<String>>,
    context: &ConvertContext,
) -> Option<core::MayBeRef<core::RequestBody>> {
    if let Some(body) = parameters.first() {
        let parameter = match body {
            MayBeRef200::Ref(value) => {
                deref_parameter(context.parameters, value.reference())
            }
            MayBeRef200::Value(value) => Some(value),
        };

        // deref parameter since to do not move any parameters to components
        if let Some(parameter) = parameter {
            let schema = if let Some(schema) = parameter.schema.clone() {
                schema
            } else {
                MayBeRef200::Value(Schema {
                    multiple_of: parameter.multiple_of,
                    maximum: parameter.maximum,
                    exclusive_maximum: parameter.exclusive_maximum,
                    minimum: parameter.minimum,
                    exclusive_minimum: parameter.exclusive_minimum,
                    max_length: parameter.max_length,
                    min_length: parameter.min_length,
                    pattern: parameter.pattern.clone(),
                    max_items: parameter.max_items,
                    min_items: parameter.min_items,
                    unique_items: parameter.unique_items,
                    r#enum: parameter.r#enum.clone(),
                    r#type: parameter.r#type.clone(),
                    items: Box::new(parameter.items.clone()),
                    format: parameter.format.clone(),
                    default: parameter.default.clone(),
                    ..Default::default()
                })
            };

            let mut media_types = consumes.clone().unwrap_or_else(|| {
                context.consumes.clone().unwrap_or_default()
            });

            if media_types.is_empty() {
                media_types.push("application/json".to_string())
            }

            let content = convert_schema_to_media_type(schema, media_types);

            Some(core::MayBeRef::Value(core::RequestBody {
                content: Some(content),
                required: parameter.required,
                description: parameter.description.clone(),
            }))
        } else {
            None
        }
    } else {
        None
    }
}

fn convert_response_ref(
    response_ref: MayBeRef200<Response>,
    produces: &[String],
    context: &ConvertContext,
    unref: bool,
) -> core::MayBeRef<core::Response> {
    match response_ref {
        MayBeRef200::Ref(value) => {
            if unref {
                let reference = value.reference.replace("#/responses/", "");

                let response = context
                    .responses
                    .as_ref()
                    .and_then(|responses| responses.get(&reference).cloned())
                    .unwrap();

                core::MayBeRef::Value(convert_response(response, produces))
            } else {
                core::MayBeRef::Ref(core::HttpSchemaRef {
                    reference: value
                        .reference
                        .replace("#/responses", "#/components/responses"),
                })
            }
        }
        MayBeRef200::Value(response) => {
            core::MayBeRef::Value(convert_response(response, produces))
        }
    }
}

fn convert_response(
    response: Response,
    produces: &[String],
) -> core::Response {
    let headers = response.headers.map(|headers| {
        headers
            .into_iter()
            .map(|(key, header)| {
                (key, core::MayBeRef::Value(convert_header(header)))
            })
            .collect()
    });

    let content = response
        .schema
        .map(|sc| convert_schema_to_media_type(sc, produces.to_owned()));

    core::Response {
        description: response.description,
        headers,
        content,
        links: None,
    }
}

fn convert_schema_to_media_type(
    schema: MayBeRef200<Schema>,
    media_types: Vec<String>,
) -> IndexMap<String, core::MediaType> {
    let media_type = core::MediaType {
        schema: Some(convert_schema_ref(schema)),
        examples: None,
        encoding: None,
    };

    media_types
        .into_iter()
        .map(|mime_type| (mime_type, media_type.clone()))
        .collect()
}

fn convert_header(header: Header) -> core::Header {
    let items = header.items.map(convert_schema_ref);

    let explode = header.format.as_ref().map(|format| format == "multi");

    let schema = core::Schema {
        multiple_of: header.multiple_of,
        maximum: header.maximum,
        exclusive_maximum: header.exclusive_maximum,
        minimum: header.minimum,
        exclusive_minimum: header.exclusive_minimum,
        max_length: header.max_length,
        min_length: header.min_length,
        pattern: header.pattern,
        max_items: header.max_items,
        min_items: header.min_items,
        unique_items: header.unique_items,
        r#enum: header.r#enum,
        r#type: Some(Either::Left(header.r#type)),
        items: Box::new(items),
        format: header.format,
        default: header.default,
        ..Default::default()
    };

    // collectionFormat in ('ssv', 'pipes', 'tsv') not supported

    core::Header {
        schema: Some(core::MayBeRef::Value(schema)),
        description: header.description,
        required: None,
        deprecated: None,
        allow_empty_value: None,
        style: None,
        explode,
        allow_reserved: None,
        examples: None,
        content: None,
        custom_fields: Default::default(),
    }
}

fn convert_parameter_ref(
    parameter_ref: MayBeRef200<Parameter>,
) -> core::MayBeRef<core::Parameter> {
    match parameter_ref {
        MayBeRef200::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value
                .reference
                .replace("#/parameters", "#/components/parameters"),
        }),
        MayBeRef200::Value(parameter) => {
            core::MayBeRef::Value(convert_parameter(parameter))
        }
    }
}

fn convert_parameter(parameter: Parameter) -> core::Parameter {
    let schema = if let Some(schema) = parameter.schema.map(convert_schema_ref)
    {
        Some(schema)
    } else {
        let all_of = parameter.all_of.map(|all_of| {
            all_of.into_iter().map(convert_schema_ref).collect()
        });

        let one_of = parameter.one_of.map(|one_of| {
            one_of.into_iter().map(convert_schema_ref).collect()
        });

        let any_of = parameter.any_of.map(|any_of| {
            any_of.into_iter().map(convert_schema_ref).collect()
        });

        let not = parameter
            .not
            .map(|not| not.into_iter().map(convert_schema_ref).collect());

        let schema = core::Schema {
            title: None,
            multiple_of: None,
            maximum: parameter.maximum,
            exclusive_maximum: parameter.exclusive_maximum,
            minimum: parameter.minimum,
            exclusive_minimum: parameter.exclusive_minimum,
            max_length: parameter.max_length,
            min_length: parameter.min_length,
            pattern: parameter.pattern,
            max_items: parameter.max_items,
            min_items: parameter.min_items,
            unique_items: parameter.unique_items,
            max_properties: None,
            min_properties: None,
            required: None,
            r#enum: parameter.r#enum,
            r#type: parameter.r#type,

            all_of,
            one_of,
            any_of,
            not,

            items: Box::new(parameter.items.map(convert_schema_ref)),
            properties: None,
            additional_properties: None,
            description: None,
            format: parameter.format,
            default: parameter.default,
            discriminator: None,
            read_only: None,
            write_only: None,
            xml: None,
            external_docs: None,
            example: None,
            deprecated: None,
            custom_fields: Default::default(),
        };

        Some(core::MayBeRef::Value(schema))
    };

    // TODO: add collection_format

    core::Parameter {
        name: parameter.name,
        r#in: parameter.r#in,
        description: parameter.description,
        required: parameter.required,
        deprecated: None,
        allow_empty_value: parameter.allow_empty_value,
        style: None,
        explode: None,
        allow_reserved: None,
        schema,
        examples: None,
        content: None,
        custom_fields: parameter.custom_fields,
    }
}

fn convert_schema_ref(
    schema_ref: MayBeRef200<Schema>,
) -> core::MayBeRef<core::Schema> {
    match schema_ref {
        MayBeRef200::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value
                .reference
                .replace("#/definitions", "#/components/schemas"),
        }),
        MayBeRef200::Value(schema) => {
            core::MayBeRef::Value(convert_schema(schema))
        }
    }
}

fn convert_schema(schema: Schema) -> core::Schema {
    let all_of = schema
        .all_of
        .map(|all_of| all_of.into_iter().map(convert_schema_ref).collect());

    let one_of = schema
        .one_of
        .map(|one_of| one_of.into_iter().map(convert_schema_ref).collect());

    let any_of = schema
        .any_of
        .map(|any_of| any_of.into_iter().map(convert_schema_ref).collect());

    let not = schema
        .not
        .map(|not| not.into_iter().map(convert_schema_ref).collect());

    let properties = schema.properties.map(|properties| {
        properties
            .into_iter()
            .map(|(name, schema)| (name, convert_schema_ref(schema)))
            .collect()
    });

    let additional_properties =
        schema.additional_properties.map(|additional_properties| {
            match additional_properties {
                Either::Left(value) => Either::Left(value),
                Either::Right(schema_ref) => {
                    Either::Right(Box::new(convert_schema_ref(*schema_ref)))
                }
            }
        });

    let discriminator =
        schema
            .discriminator
            .map(|discriminator| core::Discriminator {
                property_name: Some(discriminator),
                mapping: None,
            });

    let items = schema.items.map(convert_schema_ref);

    let external_docs = schema.external_docs.map(convert_external_docs);

    core::Schema {
        title: schema.title,
        multiple_of: schema.multiple_of,
        maximum: schema.maximum,
        exclusive_maximum: schema.exclusive_maximum,
        minimum: schema.minimum,
        exclusive_minimum: schema.exclusive_minimum,
        max_length: schema.max_length,
        min_length: schema.min_length,
        pattern: schema.pattern,
        max_items: schema.max_items,
        min_items: schema.min_items,
        unique_items: schema.unique_items,
        max_properties: schema.max_properties,
        min_properties: schema.min_properties,
        required: schema.required,
        r#enum: schema.r#enum,
        r#type: schema.r#type,
        all_of,
        one_of,
        any_of,
        not,
        items: Box::new(items),
        properties,
        additional_properties,
        description: schema.description,
        format: schema.format,
        default: schema.default,
        discriminator,
        read_only: schema.read_only,
        write_only: None,
        xml: None,
        external_docs,
        example: schema.example,
        deprecated: None,
        custom_fields: schema.custom_fields,
    }
}

fn convert_external_docs(external_docs: ExternalDoc) -> core::ExternalDoc {
    core::ExternalDoc {
        url: Some(external_docs.url),
        description: external_docs.description,
    }
}

#[cfg(test)]
mod tests {
    // use crate::schema::HttpSchema;
    // use crate::schemas::swagger2::schema::SwaggerV2;

    #[test]
    fn test_converter() {
        // let src_schema_content = include_str!("../../../tmp/cvt.json");
        // let tgt_schema_content = include_str!("../../../tmp/cvt-altered.json");
        //
        // let src_schema_v2 = serde_json::from_str::<SwaggerV2>(src_schema_content).unwrap();
        // let tgt_schema_v2 = serde_json::from_str::<SwaggerV2>(tgt_schema_content).unwrap();
        //
        // let src_schema: HttpSchema = src_schema_v2.into();
        // let tgt_schema: HttpSchema = tgt_schema_v2.into();
    }
}

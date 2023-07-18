use indexmap::IndexMap;
use serde_json::Value;

use crate::core::{Either, ReferenceDescriptor};

use crate::schema as core;

use crate::schemas::openapi303::context::*;
use crate::schemas::openapi303::schema::*;

pub const VERSION: &str = "0.1.0";

struct ConvertContext<'a> {
    pub components: &'a Option<Components>,
}

impl From<OpenApi303> for core::HttpSchema {
    fn from(spec: OpenApi303) -> Self {
        let paths = {
            let context = ConvertContext {
                components: &spec.components,
            };
            spec.paths.map(|paths| convert_paths(paths, &context))
        };

        let components = if let Some(components) = spec.components {
            let schemas = components.schemas.map(|schemas| {
                schemas
                    .into_iter()
                    .map(|(key, schema_ref)| {
                        (key, convert_schema_ref(schema_ref))
                    })
                    .collect()
            });

            let responses = components.responses.map(|responses| {
                responses
                    .into_iter()
                    .map(|(key, response_ref)| {
                        (key, convert_response_ref(response_ref))
                    })
                    .collect()
            });

            let parameters = components.parameters.map(|parameters| {
                parameters
                    .into_iter()
                    .map(|(key, parameter_ref)| {
                        (key, convert_parameter_ref(parameter_ref))
                    })
                    .collect()
            });

            let examples = components.examples.map(|examples| {
                examples
                    .into_iter()
                    .map(|(key, example_ref)| {
                        (key, convert_example_ref(example_ref))
                    })
                    .collect()
            });

            let request_bodies =
                components.request_bodies.map(|request_bodies| {
                    request_bodies
                        .into_iter()
                        .map(|(key, request_body_ref)| {
                            (key, convert_request_body_ref(request_body_ref))
                        })
                        .collect()
                });

            let headers = components.headers.map(|headers| {
                headers
                    .into_iter()
                    .map(|(key, header_ref)| {
                        (key, convert_header_ref(header_ref))
                    })
                    .collect()
            });

            let links = components.links.map(|links| {
                links
                    .into_iter()
                    .map(|(key, link_ref)| (key, convert_link_ref(link_ref)))
                    .collect()
            });

            let security_schemes =
                components.security_schemes.map(|security_schemes| {
                    security_schemes
                        .into_iter()
                        .map(|(key, security_scheme_ref)| {
                            (
                                key,
                                convert_security_scheme_ref(
                                    security_scheme_ref,
                                ),
                            )
                        })
                        .collect()
                });

            Some(core::Components {
                schemas,
                responses,
                parameters,
                examples,
                request_bodies,
                headers,
                security_schemes,
                links,
            })
        } else {
            None
        };

        let external_docs = spec.external_docs.map(convert_external_doc);

        let tags = spec
            .tags
            .map(|tags| tags.into_iter().map(convert_tag).collect());

        let info = spec.info.map(convert_info);

        let servers = spec
            .servers
            .map(|servers| servers.into_iter().map(convert_server).collect());

        core::HttpSchema {
            version: spec.openapi,

            schema_source: OpenApi303::id().to_owned(),
            schema_source_version: VERSION.to_owned(),
            schema_version: core::HttpSchema::schema_version().to_owned(),

            info,
            servers,
            paths,
            components,
            tags,
            external_docs,
        }
    }
}

fn convert_paths(
    paths: IndexMap<String, MayBeRef303<Path>>,
    context: &ConvertContext,
) -> IndexMap<String, core::MayBeRef<core::Path>> {
    paths
        .into_iter()
        .map(|(key, path_ref)| (key, convert_path_ref(path_ref, context)))
        .collect()
}

fn convert_path_ref(
    path_ref: MayBeRef303<Path>,
    context: &ConvertContext,
) -> core::MayBeRef<core::Path> {
    match path_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_path(value, context))
        }
    }
}

fn convert_path(path: Path, context: &ConvertContext) -> core::Path {
    let parameters = &path.parameters;

    let servers = path
        .servers
        .map(|servers| servers.into_iter().map(convert_server).collect());

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
        trace: path
            .trace
            .map(|op| convert_operation(op, parameters, context)),
        servers,
        summary: path.summary,
        description: path.description,
    }
}

fn merge_parameters(
    context: &ConvertContext,
    parameters_refs: Option<Vec<MayBeRef303<Parameter>>>,
    path_parameters_refs: &Option<Vec<MayBeRef303<Parameter>>>,
) -> Vec<MayBeRef303<Parameter>> {
    let mut parameters: Vec<MayBeRef303<Parameter>> = Vec::new();

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
            MayBeRef303::Ref(value) => {
                if let Some(parameter) =
                    deref_parameter(context.components, value.reference())
                {
                    (parameter.name.clone(), parameter.r#in.clone())
                } else {
                    // TODO: handle the case where ref not found
                    continue;
                }
            }
            MayBeRef303::Value(value) => {
                (value.name.clone(), value.r#in.clone())
            }
        };

        if visited.contains(&key) {
            continue;
        }

        result.push(may_be_parameter);

        visited.push(key);
    }

    result
}

fn convert_operation(
    operation: Operation,
    path_parameters: &Option<Vec<MayBeRef303<Parameter>>>,
    context: &ConvertContext,
) -> core::Operation {
    let merged_parameters = merge_parameters(
        context, // TODO: pass components
        operation.parameters,
        path_parameters,
    );

    let parameters = merged_parameters
        .into_iter()
        .map(convert_parameter_ref)
        .collect();

    let request_body = operation.request_body.map(convert_request_body_ref);

    let responses = operation.responses.map(|responses| {
        responses
            .into_iter()
            .map(|(code, response_ref)| {
                (code, convert_response_ref(response_ref))
            })
            .collect()
    });

    let external_docs = operation.external_docs.map(convert_external_doc);

    let servers = operation
        .servers
        .map(|servers| servers.into_iter().map(convert_server).collect());

    core::Operation {
        tags: operation.tags,
        summary: operation.summary,
        description: operation.description,
        external_docs,
        operation_id: operation.operation_id,
        responses,
        request_body,
        servers,
        parameters: Some(parameters),
        security: operation.security,
        deprecated: operation.deprecated,
    }
}

fn convert_request_body_ref(
    request_body_ref: MayBeRef303<RequestBody>,
) -> core::MayBeRef<core::RequestBody> {
    match request_body_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_request_body(value))
        }
    }
}

fn convert_request_body(request_body: RequestBody) -> core::RequestBody {
    core::RequestBody {
        description: request_body.description,
        required: request_body.required,
        content: request_body.content.map(convert_media_types),
    }
}

fn convert_media_types(
    media_types: IndexMap<String, MediaType>,
) -> IndexMap<String, core::MediaType> {
    media_types
        .into_iter()
        .map(|(content_type, media_type)| {
            (content_type, convert_media_type(media_type))
        })
        .collect()
}

fn convert_media_type(media_type: MediaType) -> core::MediaType {
    // media_type.example

    // let example = media_type.example;
    //
    // let examples = media_type.examples.map(
    //     |examples| {
    //         let examples = if let Some(example) = example {
    //             let example_ref = MayBeRef::Value(
    //                 Example {
    //                     summary: None,
    //                     description: None,
    //                     value: Some(example),
    //                     external_value: None,
    //                 }
    //             );
    //
    //             examples.into_iter()
    //                 .chain(vec![("-mt-example".to_string(), example_ref)])
    //                 .collect::<IndexMap<_, _>>()
    //         } else {
    //             examples
    //         };
    //
    //         examples.map(convert_examples)
    //     }
    // );
    core::MediaType {
        schema: media_type.schema.map(convert_schema_ref),
        examples: media_type.examples.map(convert_examples),
        encoding: media_type.encoding.map(convert_encodings),
    }
}

fn convert_encodings(
    encodings: IndexMap<String, Encoding>,
) -> IndexMap<String, core::Encoding> {
    encodings
        .into_iter()
        .map(|(key, encoding)| (key, convert_encoding(encoding)))
        .collect()
}

fn convert_encoding(encoding: Encoding) -> core::Encoding {
    core::Encoding {
        style: encoding.style,
        explode: encoding.explode,
        headers: encoding.headers.map(convert_headers),
        content_type: encoding.content_type,
        allow_reserved: encoding.allow_reserved,
    }
}

fn convert_response_ref(
    response_ref: MayBeRef303<Response>,
) -> core::MayBeRef<core::Response> {
    match response_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(response) => {
            core::MayBeRef::Value(convert_response(response))
        }
    }
}

fn convert_response(response: Response) -> core::Response {
    let links = response.links.map(|links| {
        links
            .into_iter()
            .map(|(key, link_ref)| (key, convert_link_ref(link_ref)))
            .collect()
    });

    core::Response {
        links,
        description: response.description,
        headers: response.headers.map(convert_headers),
        content: response.content.map(convert_media_types),
    }
}

fn convert_headers(
    headers: IndexMap<String, MayBeRef303<Header>>,
) -> IndexMap<String, core::MayBeRef<core::Header>> {
    headers
        .into_iter()
        .map(|(key, header_ref)| (key, convert_header_ref(header_ref)))
        .collect()
}

fn convert_header_ref(
    header_ref: MayBeRef303<Header>,
) -> core::MayBeRef<core::Header> {
    match header_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_header(value))
        }
    }
}

fn convert_header(header: Header) -> core::Header {
    core::Header {
        description: header.description,
        required: header.required,
        deprecated: header.deprecated,
        allow_empty_value: header.allow_empty_value,
        style: header.style,
        explode: header.explode,
        allow_reserved: header.allow_reserved,
        schema: header.schema.map(convert_schema_ref),
        examples: header.examples.map(convert_example_values),
        content: header.content.map(convert_media_types),
        custom_fields: header.custom_fields,
    }
}

fn convert_parameter_ref(
    parameter_ref: MayBeRef303<Parameter>,
) -> core::MayBeRef<core::Parameter> {
    match parameter_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(parameter) => {
            core::MayBeRef::Value(convert_parameter(parameter))
        }
    }
}

fn convert_parameter(parameter: Parameter) -> core::Parameter {
    let schema = parameter.schema.map(convert_schema_ref);
    core::Parameter {
        name: parameter.name,
        r#in: parameter.r#in,
        description: parameter.description,
        required: parameter.required,
        deprecated: parameter.deprecated,
        allow_empty_value: parameter.allow_empty_value,
        style: parameter.style,
        explode: parameter.explode,
        allow_reserved: parameter.allow_reserved,
        schema,
        examples: parameter.examples.map(convert_example_values),
        content: parameter.content.map(convert_media_types),
        custom_fields: parameter.custom_fields,
    }
}

fn convert_schema_ref(
    schema_ref: MayBeRef303<Schema>,
) -> core::MayBeRef<core::Schema> {
    match schema_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_schema(value))
        }
    }
}

fn convert_schema(schema: Schema) -> core::Schema {
    let r#type = if let Some(nullable) = schema.nullable {
        if let Some(type_) = schema.r#type {
            match type_ {
                Either::Left(single) => {
                    if nullable {
                        Some(Either::Right(Box::new(vec![
                            single,
                            "null".to_string(),
                        ])))
                    } else {
                        Some(Either::Left(single))
                    }
                }
                Either::Right(multiple) => {
                    let mut values = multiple;

                    if nullable {
                        values.push("null".to_string());
                    }

                    Some(Either::Right(values))
                }
            }
        } else {
            Some(Either::Right(Box::new(vec![
                "object".to_string(),
                "null".to_string(),
            ])))
        }
    } else {
        schema.r#type
    };

    let all_of = schema
        .all_of
        .map(|values| values.into_iter().map(convert_schema_ref).collect());

    let one_of = schema
        .one_of
        .map(|values| values.into_iter().map(convert_schema_ref).collect());

    let any_of = schema
        .any_of
        .map(|values| values.into_iter().map(convert_schema_ref).collect());

    let not = schema
        .not
        .map(|values| values.into_iter().map(convert_schema_ref).collect());

    let items = schema.items.map(convert_schema_ref);

    let properties = schema.properties.map(|properties| {
        properties
            .into_iter()
            .map(|(key, property_ref)| (key, convert_schema_ref(property_ref)))
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
                property_name: discriminator.property_name,
                mapping: discriminator.mapping,
            });

    let xml = schema.xml.map(convert_xml);

    let external_docs = schema.external_docs.map(convert_external_doc);

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
        r#type,
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
        write_only: schema.write_only,
        xml,
        external_docs,
        example: schema.example,
        deprecated: schema.deprecated,
        custom_fields: schema.custom_fields,
    }
}

fn convert_external_doc(external_doc: ExternalDoc) -> core::ExternalDoc {
    core::ExternalDoc {
        url: external_doc.url,
        description: external_doc.description,
    }
}

fn convert_xml(external_docs: Xml) -> core::Xml {
    core::Xml {
        name: external_docs.name,
        namespace: external_docs.namespace,
        prefix: external_docs.prefix,
        attribute: external_docs.attribute,
        wrapped: external_docs.wrapped,
    }
}

fn convert_link_ref(
    link_ref: MayBeRef303<Link>,
) -> core::MayBeRef<core::Link> {
    match link_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_link(value))
        }
    }
}

fn convert_link(link: Link) -> core::Link {
    core::Link {
        operation_ref: link.operation_ref,
        operation_id: link.operation_id,
        parameters: link.parameters,
        request_body: link.request_body,
        description: link.description,
        server: None,
    }
}

fn convert_examples(
    examples: IndexMap<String, MayBeRef303<Example>>,
) -> IndexMap<String, core::MayBeRef<core::Example>> {
    examples
        .into_iter()
        .map(|(key, example_ref)| (key, convert_example_ref(example_ref)))
        .collect()
}

fn convert_example_values(
    examples: IndexMap<String, MayBeRef303<Value>>,
) -> IndexMap<String, core::MayBeRef<Value>> {
    examples
        .into_iter()
        .map(|(key, example)| {
            let example_ref = match example {
                MayBeRef303::Ref(value) => {
                    core::MayBeRef::Ref(core::HttpSchemaRef {
                        reference: value.reference,
                    })
                }
                MayBeRef303::Value(value) => core::MayBeRef::Value(value),
            };
            (key, example_ref)
        })
        .collect()
}

fn convert_example_ref(
    example_ref: MayBeRef303<Example>,
) -> core::MayBeRef<core::Example> {
    match example_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_example(value))
        }
    }
}

fn convert_example(example: Example) -> core::Example {
    core::Example {
        summary: example.summary,
        description: example.description,
        value: example.value,
        external_value: example.external_value,
    }
}

fn convert_security_scheme_ref(
    security_scheme_ref: MayBeRef303<SecurityScheme>,
) -> core::MayBeRef<core::SecurityScheme> {
    match security_scheme_ref {
        MayBeRef303::Ref(value) => core::MayBeRef::Ref(core::HttpSchemaRef {
            reference: value.reference,
        }),
        MayBeRef303::Value(value) => {
            core::MayBeRef::Value(convert_security_scheme(value))
        }
    }
}

fn convert_security_scheme(
    security_scheme: SecurityScheme,
) -> core::SecurityScheme {
    core::SecurityScheme {
        r#type: security_scheme.r#type,
        description: security_scheme.description,
        name: security_scheme.name,
        r#in: security_scheme.r#in,
        scheme: security_scheme.scheme,
        bearer_format: security_scheme.bearer_format,
        flows: security_scheme.flows.map(convert_oauth_flows),
        open_id_connect_url: security_scheme.open_id_connect_url,
    }
}

fn convert_oauth_flows(oauth_flows: OAuthFlows) -> core::OAuthFlows {
    core::OAuthFlows {
        implicit: oauth_flows.implicit.map(convert_oauth_flow),
        password: oauth_flows.password.map(convert_oauth_flow),
        client_credentials: oauth_flows
            .client_credentials
            .map(convert_oauth_flow),
        authorization_code: oauth_flows
            .authorization_code
            .map(convert_oauth_flow),
    }
}

fn convert_oauth_flow(oauth_flow: OAuthFlow) -> core::OAuthFlow {
    core::OAuthFlow {
        authorization_url: oauth_flow.authorization_url,
        token_url: oauth_flow.token_url,
        refresh_url: oauth_flow.refresh_url,
        scopes: oauth_flow.scopes,
    }
}

fn convert_tag(tag: Tag) -> core::Tag {
    core::Tag {
        name: tag.name,
        description: tag.description,
        external_doc: tag.external_doc.map(convert_external_doc),
    }
}

fn convert_info(info: Info) -> core::Info {
    core::Info {
        title: info.title,
        description: info.description,
        terms_of_service: info.terms_of_service,
        contact: info.contact.map(convert_contact),
        license: info.license.map(convert_license),
        version: info.version,
    }
}

fn convert_contact(contact: Contact) -> core::Contact {
    core::Contact {
        name: contact.name,
        url: contact.url,
        email: contact.email,
    }
}

fn convert_license(license: License) -> core::License {
    core::License {
        name: license.name,
        url: license.url,
    }
}

fn convert_server(server: Server) -> core::Server {
    core::Server {
        url: server.url,
        description: server.description,
        variables: server.variables.map(|variables| {
            variables
                .into_iter()
                .map(|(key, variable)| {
                    (key, convert_server_variable(variable))
                })
                .collect()
        }),
    }
}

fn convert_server_variable(
    server_variable: ServerVariable,
) -> core::ServerVariable {
    core::ServerVariable {
        r#enum: server_variable.r#enum,
        default: server_variable.default,
        description: server_variable.description,
    }
}

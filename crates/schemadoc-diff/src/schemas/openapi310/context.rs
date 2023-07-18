use crate::schemas::openapi310::schema::*;

pub fn deref_schema<'a>(
    components: &'a Option<Components>,
    reference: &str,
) -> Option<&'a Schema> {
    components.as_ref().and_then(|components| {
        components.schemas.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/schemas/", ""))
                .map(|may_be_schema| match may_be_schema {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_parameter<'a>(
    components: &'a Option<Components>,
    reference: &str,
) -> Option<&'a Parameter> {
    components.as_ref().and_then(|components| {
        components.parameters.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/parameters/", ""))
                .map(|may_be_parameter| match may_be_parameter {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_example<'a>(
    swagger: &'a OpenApi310,
    reference: &str,
) -> Option<&'a Example> {
    swagger.components.as_ref().and_then(|components| {
        components.examples.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/examples/", ""))
                .map(|may_be_example| match may_be_example {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_request_body<'a>(
    swagger: &'a OpenApi310,
    reference: &str,
) -> Option<&'a RequestBody> {
    swagger.components.as_ref().and_then(|components| {
        components.request_bodies.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/requestBodies/", ""))
                .map(|may_be_request_body| match may_be_request_body {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_header<'a>(
    swagger: &'a OpenApi310,
    reference: &str,
) -> Option<&'a Header> {
    swagger.components.as_ref().and_then(|components| {
        components.headers.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/headers/", ""))
                .map(|may_be_header| match may_be_header {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_security_scheme<'a>(
    swagger: &'a OpenApi310,
    reference: &str,
) -> Option<&'a SecurityScheme> {
    swagger.components.as_ref().and_then(|components| {
        components.security_schemes.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/securitySchemes/", ""))
                .map(|may_be_security_scheme| match may_be_security_scheme {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_link<'a>(
    swagger: &'a OpenApi310,
    reference: &str,
) -> Option<&'a Link> {
    swagger.components.as_ref().and_then(|components| {
        components.links.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/links/", "")).map(
                |may_be_link| match may_be_link {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                },
            )
        })
    })
}

pub fn deref_response<'a>(
    swagger: &'a OpenApi310,
    reference: &str,
) -> Option<&'a Response> {
    swagger.components.as_ref().and_then(|components| {
        components.responses.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/responses/", ""))
                .map(|may_be_response| match may_be_response {
                    MayBeRef310::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

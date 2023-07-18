use serde_json::Value;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::Arc;

use crate::core::{
    ComponentContainer, DiffCache, DiffContext, DiffResult, MayBeRefCore,
};
use crate::schema::{
    Components, Example, Header, HttpSchema, Link, Parameter, Path,
    RequestBody, Response, Schema, SecurityScheme,
};
use crate::schema_diff::{
    ExampleDiff, HeaderDiff, LinkDiff, ParameterDiff, PathDiff,
    RequestBodyDiff, ResponseDiff, SchemaDiff, SecuritySchemeDiff,
};

#[derive(Clone)]
pub struct HttpSchemaDiffContext {
    depth: usize,
    direct: bool,

    source: Rc<HttpSchema>,
    target: Rc<HttpSchema>,

    source_visited_references: Rc<BTreeMap<String, usize>>,
    target_visited_references: Rc<BTreeMap<String, usize>>,

    schema_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<SchemaDiff>>>>>,
    header_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<HeaderDiff>>>>>,
    response_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<ResponseDiff>>>>>,
    parameter_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<ParameterDiff>>>>>,

    example_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<ExampleDiff>>>>>,
    request_body_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<RequestBodyDiff>>>>>,

    link_diff_cache: Rc<RefCell<HashMap<String, Arc<DiffResult<LinkDiff>>>>>,
    security_scheme_diff_cache:
        Rc<RefCell<HashMap<String, Arc<DiffResult<SecuritySchemeDiff>>>>>,
}

impl HttpSchemaDiffContext {
    pub fn new(source: Rc<HttpSchema>, target: Rc<HttpSchema>) -> Self {
        Self {
            depth: 0,
            direct: true,

            source,
            target,

            schema_diff_cache: Rc::new(RefCell::new(HashMap::new())),
            header_diff_cache: Rc::new(RefCell::new(HashMap::new())),
            response_diff_cache: Rc::new(RefCell::new(HashMap::new())),
            parameter_diff_cache: Rc::new(RefCell::new(HashMap::new())),

            example_diff_cache: Rc::new(RefCell::new(HashMap::new())),
            request_body_diff_cache: Rc::new(RefCell::new(HashMap::new())),
            link_diff_cache: Rc::new(RefCell::new(HashMap::new())),
            security_scheme_diff_cache: Rc::new(RefCell::new(HashMap::new())),

            source_visited_references: Rc::new(BTreeMap::new()),
            target_visited_references: Rc::new(BTreeMap::new()),
        }
    }
}

impl DiffContext for HttpSchemaDiffContext {
    fn removing(&self) -> HttpSchemaDiffContext {
        Self {
            source: Rc::clone(&self.source),
            target: Rc::clone(&self.target),

            depth: self.depth,
            direct: self.direct,

            schema_diff_cache: Rc::clone(&self.schema_diff_cache),
            header_diff_cache: Rc::clone(&self.header_diff_cache),
            response_diff_cache: Rc::clone(&self.response_diff_cache),
            parameter_diff_cache: Rc::clone(&self.parameter_diff_cache),

            example_diff_cache: Rc::clone(&self.example_diff_cache),
            request_body_diff_cache: Rc::clone(&self.request_body_diff_cache),
            link_diff_cache: Rc::clone(&self.link_diff_cache),
            security_scheme_diff_cache: Rc::clone(
                &self.security_scheme_diff_cache,
            ),

            source_visited_references: Rc::clone(
                &self.source_visited_references,
            ),
            target_visited_references: Rc::clone(
                &self.target_visited_references,
            ),
        }
    }

    fn switch_flow(&self) -> HttpSchemaDiffContext {
        Self {
            depth: self.depth,
            direct: !self.direct,

            source: Rc::clone(&self.source),
            target: Rc::clone(&self.target),

            source_visited_references: Rc::clone(
                &self.source_visited_references,
            ),
            target_visited_references: Rc::clone(
                &self.target_visited_references,
            ),

            schema_diff_cache: Rc::clone(&self.schema_diff_cache),
            header_diff_cache: Rc::clone(&self.header_diff_cache),
            response_diff_cache: Rc::clone(&self.response_diff_cache),
            parameter_diff_cache: Rc::clone(&self.parameter_diff_cache),

            example_diff_cache: Rc::clone(&self.example_diff_cache),
            request_body_diff_cache: Rc::clone(&self.request_body_diff_cache),
            link_diff_cache: Rc::clone(&self.link_diff_cache),
            security_scheme_diff_cache: Rc::clone(
                &self.security_scheme_diff_cache,
            ),
        }
    }

    fn is_direct_flow(&self) -> bool {
        self.direct
    }

    fn add_visited_reference_source(&self, reference: &str) -> Self {
        let mut source_visited_references =
            (*self.source_visited_references).clone();
        source_visited_references
            .entry(reference.to_owned())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        Self {
            depth: self.depth,
            direct: self.direct,

            source: Rc::clone(&self.source),
            target: Rc::clone(&self.target),

            source_visited_references: Rc::new(source_visited_references),
            target_visited_references: Rc::clone(
                &self.target_visited_references,
            ),

            schema_diff_cache: Rc::clone(&self.schema_diff_cache),
            header_diff_cache: Rc::clone(&self.header_diff_cache),
            response_diff_cache: Rc::clone(&self.response_diff_cache),
            parameter_diff_cache: Rc::clone(&self.parameter_diff_cache),

            example_diff_cache: Rc::clone(&self.example_diff_cache),
            request_body_diff_cache: Rc::clone(&self.request_body_diff_cache),
            link_diff_cache: Rc::clone(&self.link_diff_cache),
            security_scheme_diff_cache: Rc::clone(
                &self.security_scheme_diff_cache,
            ),
        }
    }

    fn check_visited_reference_source(&self, reference: &str) -> usize {
        *self.source_visited_references.get(reference).unwrap_or(&0)
    }

    fn add_visited_reference_target(&self, reference: &str) -> Self {
        let mut target_visited_references =
            (*self.target_visited_references).clone();
        target_visited_references
            .entry(reference.to_owned())
            .and_modify(|count| *count += 1)
            .or_insert(1);

        Self {
            depth: self.depth,
            direct: self.direct,

            source: Rc::clone(&self.source),
            target: Rc::clone(&self.target),

            source_visited_references: Rc::clone(
                &self.source_visited_references,
            ),
            target_visited_references: Rc::new(target_visited_references),

            schema_diff_cache: Rc::clone(&self.schema_diff_cache),
            header_diff_cache: Rc::clone(&self.header_diff_cache),
            response_diff_cache: Rc::clone(&self.response_diff_cache),
            parameter_diff_cache: Rc::clone(&self.parameter_diff_cache),

            example_diff_cache: Rc::clone(&self.example_diff_cache),
            request_body_diff_cache: Rc::clone(&self.request_body_diff_cache),
            link_diff_cache: Rc::clone(&self.link_diff_cache),
            security_scheme_diff_cache: Rc::clone(
                &self.security_scheme_diff_cache,
            ),
        }
    }

    fn check_visited_reference_target(&self, reference: &str) -> usize {
        *self.target_visited_references.get(reference).unwrap_or(&0)
    }
}

pub fn deref_schema<'a>(
    components: &'a Option<Components>,
    reference: &str,
) -> Option<&'a Schema> {
    components.as_ref().and_then(|components| {
        components.schemas.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/schemas/", ""))
                .map(|may_be_schema| match may_be_schema {
                    MayBeRefCore::Value(value) => value,
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
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_example<'a>(
    schema: &'a HttpSchema,
    reference: &str,
) -> Option<&'a Example> {
    schema.components.as_ref().and_then(|components| {
        components.examples.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/examples/", ""))
                .map(|may_be_example| match may_be_example {
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_request_body<'a>(
    schema: &'a HttpSchema,
    reference: &str,
) -> Option<&'a RequestBody> {
    schema.components.as_ref().and_then(|components| {
        components.request_bodies.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/requestBodies/", ""))
                .map(|may_be_request_body| match may_be_request_body {
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_header<'a>(
    schema: &'a HttpSchema,
    reference: &str,
) -> Option<&'a Header> {
    schema.components.as_ref().and_then(|components| {
        components.headers.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/headers/", ""))
                .map(|may_be_header| match may_be_header {
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_security_scheme<'a>(
    schema: &'a HttpSchema,
    reference: &str,
) -> Option<&'a SecurityScheme> {
    schema.components.as_ref().and_then(|components| {
        components.security_schemes.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/securitySchemes/", ""))
                .map(|may_be_security_scheme| match may_be_security_scheme {
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

pub fn deref_link<'a>(
    schema: &'a HttpSchema,
    reference: &str,
) -> Option<&'a Link> {
    schema.components.as_ref().and_then(|components| {
        components.links.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/links/", "")).map(
                |may_be_link| match may_be_link {
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                },
            )
        })
    })
}

pub fn deref_response<'a>(
    schema: &'a HttpSchema,
    reference: &str,
) -> Option<&'a Response> {
    schema.components.as_ref().and_then(|components| {
        components.responses.as_ref().and_then(|map| {
            map.get(&reference.replace("#/components/responses/", ""))
                .map(|may_be_response| match may_be_response {
                    MayBeRefCore::Value(value) => value,
                    _ => unimplemented!(),
                })
        })
    })
}

impl ComponentContainer<Path> for HttpSchemaDiffContext {
    fn deref_source(&self, _reference: &str) -> Option<&Path> {
        None
    }

    fn deref_target(&self, _reference: &str) -> Option<&Path> {
        None
    }
}

impl ComponentContainer<Value> for HttpSchemaDiffContext {
    fn deref_source(&self, _reference: &str) -> Option<&Value> {
        None
    }

    fn deref_target(&self, _reference: &str) -> Option<&Value> {
        None
    }
}

impl ComponentContainer<Schema> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&Schema> {
        deref_schema(&self.source.components, reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&Schema> {
        deref_schema(&self.target.components, reference)
    }
}

impl ComponentContainer<Parameter> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&Parameter> {
        deref_parameter(&self.source.components, reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&Parameter> {
        deref_parameter(&self.target.components, reference)
    }
}

impl ComponentContainer<Example> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&Example> {
        deref_example(self.source.borrow(), reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&Example> {
        deref_example(self.target.borrow(), reference)
    }
}

impl ComponentContainer<RequestBody> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&RequestBody> {
        deref_request_body(self.source.borrow(), reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&RequestBody> {
        deref_request_body(self.target.borrow(), reference)
    }
}

impl ComponentContainer<Header> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&Header> {
        deref_header(self.source.borrow(), reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&Header> {
        deref_header(self.target.borrow(), reference)
    }
}

impl ComponentContainer<SecurityScheme> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&SecurityScheme> {
        deref_security_scheme(self.source.borrow(), reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&SecurityScheme> {
        deref_security_scheme(self.target.borrow(), reference)
    }
}

impl ComponentContainer<Link> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&Link> {
        deref_link(self.source.borrow(), reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&Link> {
        deref_link(self.target.borrow(), reference)
    }
}

impl ComponentContainer<Response> for HttpSchemaDiffContext {
    fn deref_source(&self, reference: &str) -> Option<&Response> {
        deref_response(self.source.borrow(), reference)
    }

    fn deref_target(&self, reference: &str) -> Option<&Response> {
        deref_response(self.target.borrow(), reference)
    }
}

impl DiffCache<PathDiff> for HttpSchemaDiffContext {
    fn get_diff(&self, _reference: &str) -> Option<Arc<DiffResult<PathDiff>>> {
        None
    }

    fn set_diff(
        &self,
        _reference: &str,
        _component: Arc<DiffResult<PathDiff>>,
    ) {
    }
}

impl DiffCache<Value> for HttpSchemaDiffContext {
    fn get_diff(&self, _reference: &str) -> Option<Arc<DiffResult<Value>>> {
        None
    }

    fn set_diff(&self, _reference: &str, _component: Arc<DiffResult<Value>>) {}
}

impl DiffCache<SchemaDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<SchemaDiff>>> {
        (*self.schema_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<SchemaDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.schema_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<ParameterDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<ParameterDiff>>> {
        (*self.parameter_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<ParameterDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.parameter_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<HeaderDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<HeaderDiff>>> {
        (*self.header_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<HeaderDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.header_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<ResponseDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<ResponseDiff>>> {
        (*self.response_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<ResponseDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.response_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<ExampleDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<ExampleDiff>>> {
        (*self.example_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<ExampleDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.example_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<RequestBodyDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<RequestBodyDiff>>> {
        (*self.request_body_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<RequestBodyDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.request_body_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<SecuritySchemeDiff> for HttpSchemaDiffContext {
    fn get_diff(
        &self,
        reference: &str,
    ) -> Option<Arc<DiffResult<SecuritySchemeDiff>>> {
        (*self.security_scheme_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(
        &self,
        reference: &str,
        component: Arc<DiffResult<SecuritySchemeDiff>>,
    ) {
        let _ = RefCell::borrow_mut(&*self.security_scheme_diff_cache)
            .insert(reference.to_string(), component);
    }
}

impl DiffCache<LinkDiff> for HttpSchemaDiffContext {
    fn get_diff(&self, reference: &str) -> Option<Arc<DiffResult<LinkDiff>>> {
        (*self.link_diff_cache)
            .borrow()
            .get(reference)
            .map(Arc::clone)
    }

    fn set_diff(&self, reference: &str, component: Arc<DiffResult<LinkDiff>>) {
        let _ = RefCell::borrow_mut(&*self.link_diff_cache)
            .insert(reference.to_string(), component);
    }
}

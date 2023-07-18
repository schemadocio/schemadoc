use indexmap::IndexMap;

use crate::schemas::swagger2::schema::{Parameter, Schema};

pub fn deref_parameter<'a>(
    parameters: &'a Option<IndexMap<String, Parameter>>,
    reference: &str,
) -> Option<&'a Parameter> {
    parameters.as_ref().and_then(|parameters| {
        parameters.get(&reference.replace("#/parameters/", ""))
    })
}

pub fn deref_schema<'a>(
    definitions: &'a Option<IndexMap<String, Schema>>,
    reference: &str,
) -> Option<&'a Schema> {
    definitions.as_ref().and_then(|definitions| {
        definitions.get(&reference.replace("#/definitions/", ""))
    })
}

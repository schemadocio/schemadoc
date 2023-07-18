pub mod added_required_body_property_check;
pub mod added_required_parameter_check;
pub mod added_required_request_body_check;
pub mod removed_media_type_check;
pub mod removed_operation_check;
pub mod removed_response_property_check;
pub mod removed_schema_enum_value_check;
pub mod updated_schema_type_check;

use crate::path_pointer::PathPointer;
use crate::schema_diff::HttpSchemaDiff;

use crate::visitor::{DiffVisitor, MergedVisitor};

use crate::checker::added_required_body_property_check::AddedRequiredBodyPropertyCheck;
use crate::checker::added_required_parameter_check::AddedRequiredParameterCheck;
use crate::checker::added_required_request_body_check::AddedRequiredRequestBodyCheck;
use crate::checker::removed_media_type_check::RemovedMediaTypeCheck;
use crate::checker::removed_operation_check::RemovedOperationCheck;
use crate::checker::removed_response_property_check::RemovedResponsePropertyCheck;
use crate::checker::removed_schema_enum_value_check::RemovedSchemaEnumValueCheck;
use crate::checker::updated_schema_type_check::UpdatedSchemaTypeCheck;

#[derive(Debug)]
pub struct ValidationIssue {
    pub path: PathPointer,
    pub breaking: bool,
    pub kind: &'static str,
}

impl ValidationIssue {
    pub fn new(path: PathPointer, kind: &'static str, breaking: bool) -> Self {
        Self {
            path,
            kind,
            breaking,
        }
    }
}

pub trait HasBreakingChange {
    fn has_breaking_changes(&self) -> bool;
}

impl HasBreakingChange for &[ValidationIssue] {
    fn has_breaking_changes(&self) -> bool {
        self.iter().any(|x| x.breaking)
    }
}

impl HasBreakingChange for &[&ValidationIssue] {
    fn has_breaking_changes(&self) -> bool {
        self.iter().any(|x| x.breaking)
    }
}

trait ValidationIssuer<'s> {
    fn id(&self) -> &'static str;
    fn visitor(&self) -> &dyn DiffVisitor<'s>;
    fn issues(&self) -> Option<Vec<ValidationIssue>>;
}

pub fn validate(
    diff: &HttpSchemaDiff,
    checkers: &[&str],
) -> Vec<ValidationIssue> {
    let removed_operation = Box::<RemovedOperationCheck>::default();
    let removed_media_type = Box::<RemovedMediaTypeCheck>::default();
    let changed_schema_type = Box::<UpdatedSchemaTypeCheck>::default();
    let removed_schema_enum_value =
        Box::<RemovedSchemaEnumValueCheck>::default();
    let added_required_parameter =
        Box::<AddedRequiredParameterCheck>::default();
    let removed_response_property =
        Box::<RemovedResponsePropertyCheck>::default();
    let added_required_request_body =
        Box::<AddedRequiredRequestBodyCheck>::default();
    let added_required_body_property =
        Box::<AddedRequiredBodyPropertyCheck>::default();

    let available_issuers: Vec<&dyn ValidationIssuer> = vec![
        &*removed_operation,
        &*removed_media_type,
        &*changed_schema_type,
        &*added_required_parameter,
        &*removed_response_property,
        &*removed_schema_enum_value,
        &*added_required_request_body,
        &*added_required_body_property,
    ];

    let issuers: Vec<_> = if checkers.contains(&"*") {
        available_issuers
    } else {
        available_issuers
            .into_iter()
            .filter(|issuer| checkers.contains(&issuer.id()))
            .collect()
    };

    let visitors: Vec<_> = issuers.iter().map(|v| v.visitor()).collect();

    {
        let visitor = MergedVisitor::new(visitors.as_slice());
        crate::visitor::dispatch_visitor(diff, &visitor);
    }

    let results: Vec<_> = issuers
        .into_iter()
        .filter_map(|v| v.issues())
        .flatten()
        .collect();

    results
}

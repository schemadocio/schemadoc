use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, VecDiff};
use crate::path_pointer::PathPointer;
use crate::schema_diff::{MayBeRefDiff, OperationDiff, ParameterDiff};

use crate::visitor::DiffVisitor;

pub struct AddedRequiredParameterCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for AddedRequiredParameterCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_parameters(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<VecDiff<MayBeRefDiff<ParameterDiff>>>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_parameter(
        &self,
        pointer: &PathPointer,
        parameter_diff_result: &'s DiffResult<ParameterDiff>,
    ) -> bool {
        // Parameter Must be UPSERTED
        if !pointer.is_upserted() {
            return false;
        }

        if let Some(parameter_diff) = parameter_diff_result.get() {
            if parameter_diff.required.is_upserted() {
                if let Some(required) = parameter_diff.required.get() {
                    if *required {
                        self.pointers.borrow_mut().push(pointer.clone())
                    }
                }
            }
        }

        false
    }
}

impl Default for AddedRequiredParameterCheck {
    fn default() -> Self {
        AddedRequiredParameterCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for AddedRequiredParameterCheck {
    fn id(&self) -> &'static str {
        "added-required-parameter"
    }

    fn visitor(&self) -> &dyn DiffVisitor<'s> {
        self
    }

    fn issues(&self) -> Option<Vec<ValidationIssue>> {
        let pointers = std::mem::take(&mut *self.pointers.borrow_mut());

        let issues = pointers
            .into_iter()
            .map(|path| ValidationIssue::new(path, self.id(), true))
            .collect::<Vec<ValidationIssue>>();

        Some(issues)
    }
}

#[cfg(test)]
mod tests {
    use crate::checker::added_required_parameter_check::AddedRequiredParameterCheck;
    use crate::checker::ValidationIssuer;
    use crate::get_schema_diff;
    use crate::schema::HttpSchema;
    use crate::schemas::openapi303::schema::OpenApi303;

    #[test]
    fn test_added_required_parameter_check() {
        let src_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/added-required-parameter/schema-with-parameters.json"
        ))
        .unwrap()
        .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/added-required-parameter/schema-with-update-parameters.json"
        ))
        .unwrap()
        .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = AddedRequiredParameterCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert_eq!(issues.len(), 2);
        // `param4defaultToUpdate` updated to required=true
        assert_eq!(
            issues.get(0).unwrap().path.get_path(),
            "paths//test/get/parameters/1",
        );
        // `param6newRequired` added with required=true
        assert_eq!(
            issues.get(1).unwrap().path.get_path(),
            "paths//test/get/parameters/5",
        );
    }
}

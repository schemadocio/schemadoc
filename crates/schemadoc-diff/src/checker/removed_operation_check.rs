use std::cell::RefCell;

use crate::core::DiffResult;
use crate::path_pointer::PathPointer;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::schema_diff::OperationDiff;
use crate::visitor::DiffVisitor;

pub struct RemovedOperationCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedOperationCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _method: &str,
        operation_diff_result: &'s DiffResult<OperationDiff>,
    ) -> bool {
        if let DiffResult::Removed(_) = operation_diff_result {
            self.pointers.borrow_mut().push(pointer.clone())
        }
        false
    }
}

impl Default for RemovedOperationCheck {
    fn default() -> Self {
        Self {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedOperationCheck {
    fn id(&self) -> &'static str {
        "removed-operation"
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
    use crate::checker::removed_operation_check::RemovedOperationCheck;
    use crate::checker::ValidationIssuer;
    use crate::get_schema_diff;
    use crate::schema::HttpSchema;
    use crate::schemas::openapi303::schema::OpenApi303;

    #[test]
    fn test_removed_operation_check() {
        let src_schema: HttpSchema =
            serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-operation/schema-with-operations.json"
        ))
            .unwrap()
            .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-operation/schema-with-operations-altered.json"
        ))
        .unwrap()
        .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = RemovedOperationCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert_eq!(issues.len(), 2);
        // replaced with `delete`
        assert_eq!(issues.get(0).unwrap().path.get_path(), "paths//test/put",);
        //removed from paths
        assert_eq!(
            issues.get(1).unwrap().path.get_path(),
            "paths//test2/post",
        );
    }
}

use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, MapDiff};
use crate::path_pointer::PathPointer;
use crate::schema_diff::{
    MediaTypeDiff, OperationDiff, RequestBodyDiff, SchemaDiff,
};

use crate::visitor::DiffVisitor;

pub struct AddedRequiredBodyPropertyCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for AddedRequiredBodyPropertyCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_request_body(
        &self,
        pointer: &PathPointer,
        request_body_diff_result: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        if !pointer.is_updated() {
            return false;
        }
        // Continue only if request body is required
        if let Some(request_body_diff) = request_body_diff_result.get() {
            request_body_diff
                .required
                .get()
                .map(|v| *v)
                .unwrap_or(false)
        } else {
            false
        }
    }

    fn visit_media_types(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_media_type(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MediaTypeDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        _schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        if pointer.is_added() {
            self.pointers.borrow_mut().push(pointer.clone());
            return false;
        }

        pointer.is_updated()
    }
}

impl Default for AddedRequiredBodyPropertyCheck {
    fn default() -> Self {
        AddedRequiredBodyPropertyCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for AddedRequiredBodyPropertyCheck {
    fn id(&self) -> &'static str {
        "added-required-body-property"
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
    use crate::checker::added_required_body_property_check::AddedRequiredBodyPropertyCheck;
    use crate::checker::ValidationIssuer;
    use crate::get_schema_diff;
    use crate::schema::HttpSchema;
    use crate::schemas::openapi303::schema::OpenApi303;

    #[test]
    fn test_added_required_property_check() {
        let src_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/added-required-property/schema-with-required-body.json"
        ))
        .unwrap()
        .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/added-required-property/schema-with-required-body-altered.json"
        ))
        .unwrap()
        .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = AddedRequiredBodyPropertyCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert_eq!(issues.len(), 2);
        // new schema to allOf added
        assert_eq!(
            issues.get(0).unwrap().path.get_path(),
            "paths//test/put/requestBody/content/application/json/schema/properties/field1/allOf/0",
        );
        // new property added
        assert_eq!(
            issues.get(1).unwrap().path.get_path(),
            "paths//test/put/requestBody/content/application/json/schema/properties/field2",
        );
    }

    #[test]
    fn test_added_not_required_property_check() {
        let src_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/added-required-property/schema-with-required-body.json"
        ))
        .unwrap()
        .into();

        let get_tgt_schema = || {
            let mut schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
                "../../data/checks/added-required-property/schema-with-required-body-altered.json"
            ))
            .unwrap()
            .into();

            schema
                .paths
                .as_mut()?
                .get_mut("/test")
                .as_mut()?
                .value_mut()?
                .put
                .as_mut()?
                .request_body
                .as_mut()?
                .value_mut()?
                .required = Some(false);

            Some(schema)
        };

        let tgt_schema: HttpSchema = get_tgt_schema().unwrap();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = AddedRequiredBodyPropertyCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert!(issues.is_empty());
    }
}

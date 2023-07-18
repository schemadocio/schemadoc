use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, MapDiff};
use crate::path_pointer::{PathPointer, PointerAncestor};
use crate::schema_diff::{
    MayBeRefDiff, MediaTypeDiff, OperationDiff, ResponseDiff, SchemaDiff,
};
use crate::visitor::DiffVisitor;

pub struct RemovedResponsePropertyCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedResponsePropertyCheck {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_responses(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,
    ) -> bool {
        pointer.is_updated()
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
        _: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        if pointer.ancestor(PointerAncestor::schema()).is_removed() {
            return false;
        }

        if pointer
            .ancestor(PointerAncestor::schema_property())
            .is_removed()
        {
            self.pointers.borrow_mut().push(pointer.clone());
            return false;
        }

        pointer.is_updated()
    }
}

impl Default for RemovedResponsePropertyCheck {
    fn default() -> Self {
        Self {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedResponsePropertyCheck {
    fn id(&self) -> &'static str {
        "removed-response-property"
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
    use crate::checker::removed_response_property_check::RemovedResponsePropertyCheck;
    use crate::checker::ValidationIssuer;
    use crate::get_schema_diff;
    use crate::schema::HttpSchema;
    use crate::schemas::openapi303::schema::OpenApi303;

    #[test]
    fn test_removed_response_property_check() {
        let src_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-response-property/schema-with-responses.json"
        ))
        .unwrap()
        .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-response-property/schema-with-responses-altered.json"
        ))
        .unwrap()
        .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = RemovedResponsePropertyCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert_eq!(issues.len(), 3);
        // replaced with `shortname` property
        assert_eq!(
            issues.get(0).unwrap().path.get_path(),
            "paths//test/post/responses/200/content/application/json/schema/properties/description",
        );
        //removed
        assert_eq!(
            issues.get(1).unwrap().path.get_path(),
            "paths//test/post/responses/200/content/application/json/schema/properties/settings/properties/s2",
        );
        // parent schema type changed from "object" to "string"
        assert_eq!(
            issues.get(2).unwrap().path.get_path(),
            "paths//test/put/responses/200/content/application/json/schema/properties/id",
        );
    }
}

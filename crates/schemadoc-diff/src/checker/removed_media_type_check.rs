use std::cell::RefCell;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::core::{DiffResult, MapDiff};
use crate::path_pointer::PathPointer;
use crate::schema_diff::{
    MayBeRefDiff, MediaTypeDiff, OperationDiff, RequestBodyDiff, ResponseDiff,
};

use crate::visitor::DiffVisitor;

pub struct RemovedMediaTypeCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedMediaTypeCheck {
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
        _: &'s DiffResult<RequestBodyDiff>,
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
        _: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        true
    }

    fn visit_media_type(
        &self,
        pointer: &PathPointer,
        _: &'s DiffResult<MediaTypeDiff>,
    ) -> bool {
        if pointer.is_removed() {
            self.pointers.borrow_mut().push(pointer.clone());
            return false;
        }

        false
    }
}

impl Default for RemovedMediaTypeCheck {
    fn default() -> Self {
        RemovedMediaTypeCheck {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedMediaTypeCheck {
    fn id(&self) -> &'static str {
        "removed-media-type"
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
    use crate::checker::removed_media_type_check::RemovedMediaTypeCheck;
    use crate::checker::ValidationIssuer;
    use crate::get_schema_diff;
    use crate::schema::HttpSchema;
    use crate::schemas::openapi303::schema::OpenApi303;

    #[test]
    fn test_removed_media_type_check() {
        let src_schema: HttpSchema =
            serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-media-type/schema-with-media-types.json"
        ))
            .unwrap()
            .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-media-type/schema-with-media-types-altered.json"
        ))
        .unwrap()
        .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = RemovedMediaTypeCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert_eq!(issues.len(), 2);
        assert_eq!(
            issues.get(0).unwrap().path.get_path(),
            "paths//test/put/requestBody/content/text/plain",
        );
        assert_eq!(
            issues.get(1).unwrap().path.get_path(),
            "paths//test/put/responses/200/content/text/plain",
        );
    }
}

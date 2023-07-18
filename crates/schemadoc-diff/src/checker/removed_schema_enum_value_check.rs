use std::cell::RefCell;

use crate::core::{DiffResult, MapDiff, VecDiff};
use crate::path_pointer::PathPointer;

use crate::checker::{ValidationIssue, ValidationIssuer};
use crate::schema_diff::{
    MayBeRefDiff, MediaTypeDiff, OperationDiff, ParameterDiff,
    RequestBodyDiff, ResponseDiff, SchemaDiff,
};
use crate::visitor::DiffVisitor;

pub struct RemovedSchemaEnumValueCheck {
    pointers: RefCell<Vec<PathPointer>>,
}

impl<'s> DiffVisitor<'s> for RemovedSchemaEnumValueCheck {
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
        _: &'s DiffResult<ParameterDiff>,
    ) -> bool {
        pointer.is_updated()
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        if !pointer.is_updated() {
            return false;
        }

        let Some(schema) = schema_diff_result.get() else {
            return false;
        };

        if schema.r#enum.is_updated() {
            let has_removed = match schema.r#enum.get() {
                None => false,
                Some(values) => values.iter().any(|v| v.is_removed()),
            };
            if has_removed {
                self.pointers.borrow_mut().push(pointer.add_component(
                    &schema.r#enum,
                    Some("enum"),
                    None,
                ))
            }
        }

        true
    }
}

impl Default for RemovedSchemaEnumValueCheck {
    fn default() -> Self {
        Self {
            pointers: RefCell::new(vec![]),
        }
    }
}

impl<'s> ValidationIssuer<'s> for RemovedSchemaEnumValueCheck {
    fn id(&self) -> &'static str {
        "removed-schema-enum-value"
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
    use crate::checker::removed_schema_enum_value_check::RemovedSchemaEnumValueCheck;
    use crate::checker::ValidationIssuer;
    use crate::get_schema_diff;
    use crate::schema::HttpSchema;
    use crate::schemas::openapi303::schema::OpenApi303;

    #[test]
    fn test_removed_schema_enum_value_check() {
        let src_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-schema-enum-value/schema-with-enums.json"
        ))
            .unwrap()
            .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(include_str!(
            "../../data/checks/removed-schema-enum-value/schema-with-enums-altered.json"
        ))
            .unwrap()
            .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        let checker = RemovedSchemaEnumValueCheck::default();
        crate::visitor::dispatch_visitor(diff.get().unwrap(), &checker);
        let issues = checker.issues().unwrap();

        assert_eq!(issues.len(), 2);
        assert_eq!(
            issues.get(0).unwrap().path.get_path(),
            "paths//test/post/requestBody/content/application/json/schema/enum",
        );
        assert_eq!(
            issues.get(1).unwrap().path.get_path(),
            "paths//test/post/responses/200/content/application/json/schema/properties/prop1/enum",
        );
    }
}

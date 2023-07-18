use crate::core::{DiffResult, EitherDiff, MapDiff, VecDiff};
use crate::path_pointer::{PathPointer, PathPointerScope};
use std::cell::RefCell;

use crate::schema_diff::{
    deref_parameter_diff, deref_request_body_diff, deref_response_diff,
    deref_schema_diff, HttpSchemaDiff, MayBeRefDiff, MediaTypeDiff,
    OperationDiff, ParameterDiff, PathDiff, RequestBodyDiff, ResponseDiff,
    SchemaDiff,
};
use crate::schema_diff_utils::PathsMapPathResolver;

#[allow(unused_variables)]
pub trait DiffVisitor<'s> {
    fn visit_root(&self) {}

    // Always look into by default

    fn visit_paths(
        &self,
        pointer: &PathPointer,
        paths_diff_result: &'s DiffResult<
            MapDiff<MayBeRefDiff<PathDiff>, PathsMapPathResolver>,
        >,
    ) -> bool {
        true
    }
    fn visit_path(
        &self,
        pointer: &PathPointer,
        path: &str,
        path_diff_result: &'s DiffResult<PathDiff>,
    ) -> bool {
        true
    }

    fn visit_schema_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<SchemaDiff>>,
    ) -> bool {
        true
    }

    fn visit_response_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<ResponseDiff>>,
    ) -> bool {
        true
    }

    fn visit_parameter_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<ParameterDiff>>,
    ) -> bool {
        true
    }

    fn visit_request_body_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<RequestBodyDiff>>,
    ) -> bool {
        true
    }

    // Specify in concrete visitor whether to visit deeper entities

    fn visit_operation(
        &self,
        pointer: &PathPointer,
        method: &str,
        operation_diff_result: &'s DiffResult<OperationDiff>,
    ) -> bool {
        false
    }

    fn visit_request_body(
        &self,
        pointer: &PathPointer,
        request_body_diff_result: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        false
    }

    fn visit_responses(
        &self,
        pointer: &PathPointer,
        responses_diff_result: &'s DiffResult<
            MapDiff<MayBeRefDiff<ResponseDiff>>,
        >,
    ) -> bool {
        false
    }

    fn visit_media_types(
        &self,
        pointer: &PathPointer,
        media_types_diff_result: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        false
    }
    fn visit_media_type(
        &self,
        pointer: &PathPointer,
        media_type_diff_result: &'s DiffResult<MediaTypeDiff>,
    ) -> bool {
        false
    }

    fn visit_parameters(
        &self,
        pointer: &PathPointer,
        parameters_diff_result: &'s DiffResult<
            VecDiff<MayBeRefDiff<ParameterDiff>>,
        >,
    ) -> bool {
        false
    }

    fn visit_parameter(
        &self,
        pointer: &PathPointer,
        parameter_diff_result: &'s DiffResult<ParameterDiff>,
    ) -> bool {
        false
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        false
    }
}

pub fn dispatch_visitor<'s, T: DiffVisitor<'s>>(
    root: &'s HttpSchemaDiff,
    visitor: &T,
) {
    visitor.visit_root();

    let pointer = PathPointer::new(
        &root.paths,
        Some("paths"),
        Some(PathPointerScope::Paths),
    );

    dispatch_paths(root, &pointer, &root.paths, visitor);
}

pub fn dispatch_paths<'s, T>(
    root: &'s HttpSchemaDiff,
    pointer: &PathPointer,
    paths_diff_result: &'s DiffResult<
        MapDiff<MayBeRefDiff<PathDiff>, PathsMapPathResolver>,
    >,
    visitor: &T,
) where
    T: DiffVisitor<'s>,
{
    if !visitor.visit_paths(pointer, paths_diff_result) {
        return;
    }

    if let Some(paths) = paths_diff_result.get() {
        for (path, may_be_path_diff_result) in paths.iter() {
            let pointer = pointer.add_context(may_be_path_diff_result);
            if let Some(MayBeRefDiff::Value(path_diff_result)) =
                may_be_path_diff_result.get()
            {
                let pointer = pointer.add(
                    &**path_diff_result,
                    path,
                    Some(PathPointerScope::Path),
                );
                dispatch_path(root, &pointer, path, path_diff_result, visitor)
            }
        }
    }
}

pub fn dispatch_path<'s, T: DiffVisitor<'s>>(
    root: &'s HttpSchemaDiff,
    pointer: &PathPointer,
    path: &'s str,
    path_diff_result: &'s DiffResult<PathDiff>,
    visitor: &T,
) {
    if !visitor.visit_path(pointer, path, path_diff_result) {
        return;
    }

    if let Some(path) = path_diff_result.get() {
        dispatch_operation(root, pointer, "get", &path.get, visitor);
        dispatch_operation(root, pointer, "post", &path.post, visitor);
        dispatch_operation(root, pointer, "put", &path.put, visitor);
        dispatch_operation(root, pointer, "patch", &path.patch, visitor);
        dispatch_operation(root, pointer, "delete", &path.delete, visitor);
        dispatch_operation(root, pointer, "head", &path.head, visitor);
        dispatch_operation(root, pointer, "options", &path.options, visitor);
        dispatch_operation(root, pointer, "trace", &path.trace, visitor);
    }
}

pub fn dispatch_operation<'s, T: DiffVisitor<'s>>(
    root: &'s HttpSchemaDiff,
    pointer: &PathPointer,
    method: &'s str,
    operation_diff_result: &'s DiffResult<OperationDiff>,
    visitor: &T,
) {
    if operation_diff_result.is_none() {
        return;
    }

    let pointer = pointer.add(
        operation_diff_result,
        method,
        Some(PathPointerScope::Operation),
    );

    if !visitor.visit_operation(&pointer, method, operation_diff_result) {
        return;
    }

    if let Some(operation) = operation_diff_result.get() {
        // operation.request_body
        let p = pointer.add_context(&operation.request_body);
        if visitor.visit_request_body_ref(&p, &operation.request_body) {
            if let Some(request_body) = operation.request_body.get() {
                if let Some(request_body_diff_result) =
                    deref_request_body_diff(root, request_body)
                {
                    let pointer = p.add(
                        request_body_diff_result,
                        "requestBody",
                        Some(PathPointerScope::RequestBody),
                    );
                    if visitor
                        .visit_request_body(&pointer, request_body_diff_result)
                    {
                        if let Some(request_body) =
                            request_body_diff_result.get()
                        {
                            let pointer = pointer.add(
                                &request_body.content,
                                "content",
                                None,
                            );
                            dispatch_media_types(
                                root,
                                &pointer,
                                &request_body.content,
                                visitor,
                                5,
                            );
                        }
                    }
                }
            }
        }

        // operation.responses
        let p = pointer.add(
            &operation.responses,
            "responses",
            Some(PathPointerScope::Responses),
        );
        if visitor.visit_responses(&p, &operation.responses) {
            if let Some(responses) = operation.responses.get() {
                for (code, response_diff_result) in responses.iter() {
                    let pointer = p.add(
                        response_diff_result,
                        code,
                        Some(PathPointerScope::ResponseCode),
                    );
                    if visitor
                        .visit_response_ref(&pointer, response_diff_result)
                    {
                        if let Some(response_diff_result_ref) =
                            response_diff_result.get()
                        {
                            if let Some(response_diff_result) =
                                deref_response_diff(
                                    root,
                                    response_diff_result_ref,
                                )
                            {
                                let pointer =
                                    pointer.add_context(response_diff_result);
                                if let Some(response_diff) =
                                    response_diff_result.get()
                                {
                                    // response_diff.content
                                    let p = pointer.add(
                                        &response_diff.content,
                                        "content",
                                        None,
                                    );
                                    dispatch_media_types(
                                        root,
                                        &p,
                                        &response_diff.content,
                                        visitor,
                                        5,
                                    );
                                    // response_diff.headers
                                    // TODO
                                }
                            }
                        }
                    }
                }
            }
        }

        let p = pointer.add(
            &operation.parameters,
            "parameters",
            Some(PathPointerScope::Parameters),
        );
        if visitor.visit_parameters(&p, &operation.parameters) {
            if let Some(parameters) = operation.parameters.get() {
                for (idx, may_be_parameter_diff_result) in
                    parameters.iter().enumerate()
                {
                    let pointer = p.add(
                        may_be_parameter_diff_result,
                        idx.to_string(),
                        None,
                    );
                    if visitor.visit_parameter_ref(
                        &pointer,
                        may_be_parameter_diff_result,
                    ) {
                        if let Some(may_be_parameter) =
                            may_be_parameter_diff_result.get()
                        {
                            if let Some(parameter_diff) =
                                deref_parameter_diff(root, may_be_parameter)
                            {
                                let pointer =
                                    pointer.add_context(parameter_diff);
                                visitor
                                    .visit_parameter(&pointer, parameter_diff);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn dispatch_media_types<'s, T: DiffVisitor<'s>>(
    root: &'s HttpSchemaDiff,
    pointer: &PathPointer,
    media_types_diff_result: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    visitor: &T,
    depth: usize,
) {
    if !visitor.visit_media_types(pointer, media_types_diff_result) {
        return;
    }

    if let Some(media_types) = media_types_diff_result.get() {
        for (mime_type, media_type_diff_result) in media_types.iter() {
            let p = pointer.add(
                media_type_diff_result,
                mime_type,
                Some(PathPointerScope::MediaType),
            );

            if !visitor.visit_media_type(&p, media_type_diff_result) {
                continue;
            }

            if let Some(media_type) = media_type_diff_result.get() {
                let p = p.add(
                    &media_type.schema,
                    "schema",
                    Some(PathPointerScope::Schema),
                );
                dispatch_schema(root, &p, &media_type.schema, visitor, depth);
            }
        }
    }
}

pub fn dispatch_schema<'s, T: DiffVisitor<'s>>(
    root: &'s HttpSchemaDiff,
    pointer: &PathPointer,
    may_be_schema_diff_result: &'s DiffResult<MayBeRefDiff<SchemaDiff>>,
    visitor: &T,
    depth: usize,
) {
    if depth == 0 {
        return;
    }

    if !visitor.visit_schema_ref(pointer, may_be_schema_diff_result) {
        return;
    }

    if let Some(may_be_schema) = may_be_schema_diff_result.get() {
        if let Some(schema_diff_result) =
            deref_schema_diff(root, may_be_schema)
        {
            let pointer = pointer.add_context(schema_diff_result);

            if !visitor.visit_schema(&pointer, schema_diff_result) {
                return;
            }

            if let Some(schema) = schema_diff_result.get() {
                // schema.properties
                if let Some(properties) = schema.properties.get() {
                    let pointer = pointer.add(
                        &schema.properties,
                        "properties",
                        Some(PathPointerScope::SchemaProperties),
                    );
                    for (name, property) in properties.iter() {
                        let pointer = pointer.add(
                            property,
                            name,
                            Some(PathPointerScope::SchemaProperty),
                        );
                        dispatch_schema(
                            root,
                            &pointer,
                            property,
                            visitor,
                            depth - 1,
                        )
                    }
                }

                // schema.items
                if !schema.items.is_none() {
                    let items_pointer = pointer.add(
                        &*schema.items,
                        "items",
                        Some(PathPointerScope::SchemaItems),
                    );
                    dispatch_schema(
                        root,
                        &items_pointer,
                        &schema.items,
                        visitor,
                        depth - 1,
                    );
                }

                // schema.not
                if let Some(may_be_schema_vec) = schema.not.get() {
                    let pointer = pointer.add(
                        &schema.not,
                        "not",
                        Some(PathPointerScope::SchemaNot),
                    );
                    for (idx, may_be_schema_diff_result) in
                        may_be_schema_vec.iter().enumerate()
                    {
                        let pointer = pointer.add(
                            may_be_schema_diff_result,
                            idx.to_string(),
                            None,
                        );
                        dispatch_schema(
                            root,
                            &pointer,
                            may_be_schema_diff_result,
                            visitor,
                            depth - 1,
                        );
                    }
                }

                // schema.oneOf
                if let Some(may_be_schema_vec) = schema.one_of.get() {
                    let pointer = pointer.add(
                        &schema.one_of,
                        "oneOf",
                        Some(PathPointerScope::SchemaOneOf),
                    );
                    for (idx, may_be_schema_diff_result) in
                        may_be_schema_vec.iter().enumerate()
                    {
                        let pointer = pointer.add(
                            may_be_schema_diff_result,
                            idx.to_string(),
                            None,
                        );
                        dispatch_schema(
                            root,
                            &pointer,
                            may_be_schema_diff_result,
                            visitor,
                            depth - 1,
                        );
                    }
                }

                // schema.anyOf
                if let Some(may_be_schema_vec) = schema.any_of.get() {
                    let pointer = pointer.add(
                        &schema.any_of,
                        "anyOf",
                        Some(PathPointerScope::SchemaAnyOf),
                    );
                    for (idx, may_be_schema_diff_result) in
                        may_be_schema_vec.iter().enumerate()
                    {
                        let pointer = pointer.add(
                            may_be_schema_diff_result,
                            idx.to_string(),
                            None,
                        );
                        dispatch_schema(
                            root,
                            &pointer,
                            may_be_schema_diff_result,
                            visitor,
                            depth - 1,
                        );
                    }
                }

                // schema.allOf
                if let Some(may_be_schema_vec) = schema.all_of.get() {
                    let pointer = pointer.add(
                        &schema.all_of,
                        "allOf",
                        Some(PathPointerScope::SchemaAllOf),
                    );
                    for (idx, may_be_schema_diff_result) in
                        may_be_schema_vec.iter().enumerate()
                    {
                        let pointer = pointer.add(
                            may_be_schema_diff_result,
                            idx.to_string(),
                            None,
                        );
                        dispatch_schema(
                            root,
                            &pointer,
                            may_be_schema_diff_result,
                            visitor,
                            depth - 1,
                        );
                    }
                }

                // schema.additionalProperties
                if let Some(either_schema) = schema.additional_properties.get()
                {
                    let pointer = pointer.add(
                        &schema.additional_properties,
                        "additionalProperties",
                        Some(PathPointerScope::SchemaAdditionalProperties),
                    );
                    if let EitherDiff::Right(may_be_schema_diff_result) =
                        either_schema
                    {
                        let pointer =
                            pointer.add_context(&**may_be_schema_diff_result);
                        dispatch_schema(
                            root,
                            &pointer,
                            may_be_schema_diff_result,
                            visitor,
                            depth - 1,
                        );
                    }
                    if let EitherDiff::ToRight(may_be_schema_diff_result) =
                        either_schema
                    {
                        let pointer =
                            pointer.add_context(&**may_be_schema_diff_result);
                        dispatch_schema(
                            root,
                            &pointer,
                            may_be_schema_diff_result,
                            visitor,
                            depth - 1,
                        );
                    }
                }
            }
        }
    }
}

pub struct MergedVisitor<'a, 's> {
    visitors: &'a [&'a dyn DiffVisitor<'s>],
    config: RefCell<Vec<Option<PathPointer>>>,
}

impl<'a, 's> MergedVisitor<'a, 's> {
    pub fn new(visitors: &'s [&'a dyn DiffVisitor<'s>]) -> Self {
        let config = RefCell::new(vec![None; visitors.len()]);

        Self { config, visitors }
    }

    #[inline(always)]
    fn visit<C>(&self, pointer: &PathPointer, visit: C) -> bool
    where
        C: Fn(&&'a dyn DiffVisitor<'s>) -> bool,
    {
        let mut config = self.config.borrow_mut();
        let mut result = false;

        for (idx, visitor) in self.visitors.iter().enumerate() {
            let skip = config[idx]
                .as_ref()
                .map(|stopper| pointer.startswith(stopper))
                .unwrap_or(false);
            if skip {
                continue;
            }

            let proceed = visit(visitor);
            if !proceed {
                config[idx] = Some(pointer.clone());
            }

            result |= proceed;
        }
        result
    }
}

impl<'a, 's> DiffVisitor<'s> for MergedVisitor<'a, 's> {
    fn visit_root(&self) {
        self.visitors.iter().for_each(|v| v.visit_root());
    }

    fn visit_paths(
        &self,
        pointer: &PathPointer,
        paths_diff_result: &'s DiffResult<
            MapDiff<MayBeRefDiff<PathDiff>, PathsMapPathResolver>,
        >,
    ) -> bool {
        self.visit(pointer, |v| v.visit_paths(pointer, paths_diff_result))
    }

    fn visit_path(
        &self,
        pointer: &PathPointer,
        path: &str,
        path_diff_result: &'s DiffResult<PathDiff>,
    ) -> bool {
        self.visit(pointer, |v| v.visit_path(pointer, path, path_diff_result))
    }

    fn visit_operation(
        &self,
        pointer: &PathPointer,
        method: &str,
        operation_diff_result: &'s DiffResult<OperationDiff>,
    ) -> bool {
        self.visit(pointer, |v| {
            v.visit_operation(pointer, method, operation_diff_result)
        })
    }

    fn visit_request_body(
        &self,
        pointer: &PathPointer,
        request_body_diff_result: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        self.visit(pointer, |v| {
            v.visit_request_body(pointer, request_body_diff_result)
        })
    }

    fn visit_media_types(
        &self,
        pointer: &PathPointer,
        media_types_diff_result: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        self.visit(pointer, |v| {
            v.visit_media_types(pointer, media_types_diff_result)
        })
    }

    fn visit_media_type(
        &self,
        pointer: &PathPointer,
        media_type_diff_result: &'s DiffResult<MediaTypeDiff>,
    ) -> bool {
        self.visit(pointer, |v| {
            v.visit_media_type(pointer, media_type_diff_result)
        })
    }

    fn visit_parameters(
        &self,
        pointer: &PathPointer,
        parameters_diff_result: &'s DiffResult<
            VecDiff<MayBeRefDiff<ParameterDiff>>,
        >,
    ) -> bool {
        self.visit(pointer, |v| {
            v.visit_parameters(pointer, parameters_diff_result)
        })
    }

    fn visit_parameter(
        &self,
        pointer: &PathPointer,
        parameter_diff_result: &'s DiffResult<ParameterDiff>,
    ) -> bool {
        self.visit(pointer, |v| {
            v.visit_parameter(pointer, parameter_diff_result)
        })
    }

    fn visit_schema(
        &self,
        pointer: &PathPointer,
        schema_diff_result: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        self.visit(pointer, |v| v.visit_schema(pointer, schema_diff_result))
    }
}

#[cfg(test)]
mod test {
    use crate::core::{DiffResult, MapDiff};
    use crate::get_schema_diff;
    use crate::path_pointer::{PathPointer, PathPointerScope};
    use crate::schema::HttpSchema;
    use crate::schema_diff::{
        HttpSchemaDiff, MayBeRefDiff, OperationDiff, PathDiff,
        RequestBodyDiff, ResponseDiff,
    };
    use crate::schema_diff_utils::PathsMapPathResolver;
    use crate::schemas::openapi303::schema::OpenApi303;
    use crate::visitor::{dispatch_visitor, DiffVisitor};

    #[test]
    fn test_pointer_level_values() {
        let src_schema: HttpSchema = serde_json::from_str::<OpenApi303>(
            include_str!("../data/visitor-pointer-test.json"),
        )
        .unwrap()
        .into();

        let tgt_schema: HttpSchema = serde_json::from_str::<OpenApi303>(
            include_str!("../data/visitor-pointer-test.json"),
        )
        .unwrap()
        .into();

        let diff = get_schema_diff(src_schema, tgt_schema);

        struct PointerLevelVisitor<'s>(&'s HttpSchemaDiff);

        impl<'s> DiffVisitor<'s> for PointerLevelVisitor<'s> {
            fn visit_paths(
                &self,
                pointer: &PathPointer,
                _: &'s DiffResult<
                    MapDiff<MayBeRefDiff<PathDiff>, PathsMapPathResolver>,
                >,
            ) -> bool {
                let component = pointer.get(PathPointerScope::Paths).unwrap();
                assert_eq!(component.path, Some("paths".to_string()));
                true
            }

            fn visit_path(
                &self,
                pointer: &PathPointer,
                _: &str,
                _: &'s DiffResult<PathDiff>,
            ) -> bool {
                let component = pointer.get(PathPointerScope::Path).unwrap();
                assert_eq!(
                    component.path,
                    Some("/{entity_type}/{id}/change_tags".to_string())
                );
                true
            }

            fn visit_operation(
                &self,
                pointer: &PathPointer,
                _: &str,
                _: &'s DiffResult<OperationDiff>,
            ) -> bool {
                let component =
                    pointer.get(PathPointerScope::Operation).unwrap();
                assert_eq!(component.path, Some("post".to_string()));
                true
            }

            fn visit_request_body(
                &self,
                pointer: &PathPointer,
                _: &'s DiffResult<RequestBodyDiff>,
            ) -> bool {
                let component =
                    pointer.get(PathPointerScope::RequestBody).unwrap();
                assert_eq!(component.path, Some("requestBody".to_string()));
                false
            }

            fn visit_responses(
                &self,
                pointer: &PathPointer,
                _: &'s DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,
            ) -> bool {
                let component =
                    pointer.get(PathPointerScope::Responses).unwrap();
                assert_eq!(component.path, Some("responses".to_string()));
                false
            }
        }

        dispatch_visitor(
            diff.get().unwrap(),
            &PointerLevelVisitor(&diff.get().unwrap()),
        );
    }
}

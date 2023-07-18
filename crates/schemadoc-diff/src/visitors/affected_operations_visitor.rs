use std::cell::RefCell;
use std::collections::HashMap;

use crate::core::{DiffResult, MapDiff, VecDiff};
use crate::diff_result_type::DiffResultType;
use crate::path_pointer::PathPointer;
use crate::schema_diff::{
    deref_parameter_diff, deref_request_body_diff, deref_response_diff,
    deref_schema_diff, HttpSchemaDiff, MayBeRefDiff, MediaTypeDiff,
    OperationDiff, ParameterDiff, RequestBodyDiff, ResponseDiff, SchemaDiff,
};

use crate::visitor::{dispatch_visitor, DiffVisitor};

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum SharedChangeComponent {
    RequestBody,
    Parameter,
    Response,
    Schema,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct SharedChange {
    kind: DiffResultType,
    reference: String,
    component: SharedChangeComponent,
}

struct SharedChangesVisitor<'s> {
    diff: &'s HttpSchemaDiff,
    pointers: RefCell<HashMap<SharedChange, Vec<PathPointer>>>,
}

impl<'s> SharedChangesVisitor<'s> {
    pub fn new(diff: &'s HttpSchemaDiff) -> Self {
        Self {
            diff,
            pointers: RefCell::new(HashMap::new()),
        }
    }
}

impl<'s> DiffVisitor<'s> for SharedChangesVisitor<'s> {
    /// Actual code

    fn visit_schema_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<SchemaDiff>>,
    ) -> bool {
        if !pointer.parent().is_updated() || may_be_ref.is_same_or_none() {
            return false;
        }

        let Some(value) = may_be_ref.get() else {
            return false;
        };

        let MayBeRefDiff::Ref(reference) = value else {
            return true;
        };

        let Some(diff) = deref_schema_diff(self.diff, value) else {
            return false;
        };

        let key = SharedChange {
            kind: diff.into(),
            reference: reference.reference.clone(),
            component: SharedChangeComponent::Schema,
        };

        self.pointers
            .borrow_mut()
            .entry(key)
            .and_modify(|arr| arr.push(pointer.clone()))
            .or_insert_with(|| vec![pointer.clone()]);

        false
    }

    fn visit_response_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<ResponseDiff>>,
    ) -> bool {
        if !pointer.parent().is_updated() || may_be_ref.is_same_or_none() {
            return false;
        }

        let Some(value) = may_be_ref.get() else {
            return false;
        };

        let MayBeRefDiff::Ref(reference) = value else {
            return true;
        };

        let Some(diff) = deref_response_diff(self.diff, value) else {
            return false;
        };

        let key = SharedChange {
            kind: diff.into(),
            reference: reference.reference.clone(),
            component: SharedChangeComponent::Response,
        };

        self.pointers
            .borrow_mut()
            .entry(key)
            .and_modify(|arr| arr.push(pointer.clone()))
            .or_insert_with(|| vec![pointer.clone()]);

        false
    }

    fn visit_parameter_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<ParameterDiff>>,
    ) -> bool {
        if !pointer.parent().is_updated() || may_be_ref.is_same_or_none() {
            return false;
        }

        let Some(value) = may_be_ref.get() else {
            return false;
        };

        let MayBeRefDiff::Ref(reference) = value else {
            return true;
        };

        let Some(diff) = deref_parameter_diff(self.diff, value) else {
            return false;
        };

        let key = SharedChange {
            kind: diff.into(),
            reference: reference.reference.clone(),
            component: SharedChangeComponent::Parameter,
        };

        self.pointers
            .borrow_mut()
            .entry(key)
            .and_modify(|arr| arr.push(pointer.clone()))
            .or_insert_with(|| vec![pointer.clone()]);

        false
    }

    fn visit_request_body_ref(
        &self,
        pointer: &PathPointer,
        may_be_ref: &'s DiffResult<MayBeRefDiff<RequestBodyDiff>>,
    ) -> bool {
        if !pointer.parent().is_updated() || may_be_ref.is_same_or_none() {
            return false;
        }

        let Some(value) = may_be_ref.get() else {
            return false;
        };

        let MayBeRefDiff::Ref(reference) = value else {
            return true;
        };

        let Some(diff) = deref_request_body_diff(self.diff, value) else {
            return false;
        };

        let key = SharedChange {
            kind: diff.into(),
            reference: reference.reference.clone(),
            component: SharedChangeComponent::RequestBody,
        };

        self.pointers
            .borrow_mut()
            .entry(key)
            .and_modify(|arr| arr.push(pointer.clone()))
            .or_insert_with(|| vec![pointer.clone()]);

        false
    }
    fn visit_operation(
        &self,
        p: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_request_body(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<RequestBodyDiff>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_responses(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<MapDiff<MayBeRefDiff<ResponseDiff>>>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_media_types(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<MapDiff<MediaTypeDiff>>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_media_type(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<MediaTypeDiff>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_parameters(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<VecDiff<MayBeRefDiff<ParameterDiff>>>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_parameter(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<ParameterDiff>,
    ) -> bool {
        p.parent().is_updated()
    }

    fn visit_schema(
        &self,
        p: &PathPointer,
        _: &'s DiffResult<SchemaDiff>,
    ) -> bool {
        p.parent().is_updated()
    }
}

pub fn get_shared_changes(
    diff: &HttpSchemaDiff,
) -> HashMap<SharedChange, Vec<PathPointer>> {
    let visitor = SharedChangesVisitor::new(diff);

    dispatch_visitor(diff, &visitor);

    visitor.pointers.into_inner()
}

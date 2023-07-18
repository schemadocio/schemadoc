pub mod markdown;

use indexmap::IndexMap;

use crate::checker::ValidationIssue;
use crate::path_pointer::{PathPointer, PathPointerScope};

pub struct Text(String, bool);

impl Text {
    pub fn new(text: String, is_empty: bool) -> Self {
        Text(text, is_empty)
    }
    pub fn inner(&self) -> &str {
        &self.0
    }
    pub fn is_empty(&self) -> bool {
        self.1
    }
}

pub struct Markdown(String, bool);

impl Markdown {
    pub fn new(text: String, is_empty: bool) -> Self {
        Markdown(text, is_empty)
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
    pub fn is_empty(&self) -> bool {
        self.1
    }
}

pub trait Exporter<R> {
    fn export(
        &self,
        info: IndexMap<&str, &str>,
        version_url: &str,
        invalid_only: bool,
        path_filters: Option<&[String]>,
        validations: Option<&[ValidationIssue]>,
    ) -> R;
}

pub fn display_uri(pointer: &PathPointer) -> String {
    if let Some(component) = pointer.get(PathPointerScope::Path) {
        component
            .path
            .as_ref()
            .map_or_else(|| "".to_string(), |x| x.clone())
    } else {
        "".to_string()
    }
}

pub fn display_method(pointer: &PathPointer) -> String {
    if let Some(component) = pointer.get(PathPointerScope::Operation) {
        component
            .path
            .as_ref()
            .map_or_else(|| "".to_string(), |x| x.clone())
    } else {
        "".to_string()
    }
}

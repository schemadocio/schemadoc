use indexmap::IndexMap;
use std::cell::RefCell;

use crate::core::DiffResult;
use crate::exporters::{display_method, display_uri, Exporter, Markdown};

use crate::checker::ValidationIssue;
use crate::path_pointer::PathPointer;
use crate::schema_diff::{HttpSchemaDiff, OperationDiff};

use crate::visitor::{dispatch_visitor, DiffVisitor};

struct PathToMarkdownVisitor<'s, 'v> {
    invalid_only: bool,
    endpoints: Option<&'v [String]>,
    validations: Option<&'v [ValidationIssue]>,

    added: RefCell<Vec<(PathPointer, &'s OperationDiff, bool)>>,
    updated: RefCell<Vec<(PathPointer, &'s OperationDiff, bool)>>,
    removed: RefCell<Vec<(PathPointer, &'s OperationDiff, bool)>>,
}

impl<'s, 'v> DiffVisitor<'s> for PathToMarkdownVisitor<'s, 'v> {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _method: &str,
        operation_diff_result: &'s DiffResult<OperationDiff>,
    ) -> bool {
        if let Some(endpoints) = self.endpoints {
            if !endpoints.is_empty() {
                let is_matches =
                    endpoints.iter().any(|filter| pointer.matches(filter));
                if !is_matches {
                    return false;
                }
            }
        }

        let mut has_breaking = false;
        if let Some(validations) = self.validations {
            let is_invalid = validations
                .iter()
                .any(|validation| validation.path.startswith(pointer));
            if self.invalid_only && !is_invalid {
                return false;
            }

            has_breaking = validations.iter().any(|validation| {
                validation.path.startswith(pointer) && validation.breaking
            });
        }

        match operation_diff_result {
            DiffResult::None => {}
            DiffResult::Same(_) => {}
            DiffResult::Added(value) => {
                self.added.borrow_mut().push((
                    pointer.clone(),
                    value,
                    has_breaking,
                ));
            }
            DiffResult::Updated(value, _) => {
                self.updated.borrow_mut().push((
                    pointer.clone(),
                    value,
                    has_breaking,
                ));
            }
            DiffResult::Removed(value) => {
                self.removed.borrow_mut().push((
                    pointer.clone(),
                    value,
                    has_breaking,
                ));
            }
        };

        false
    }
}

impl Exporter<Markdown> for HttpSchemaDiff {
    fn export(
        &self,
        info: IndexMap<&str, &str>,
        version_url: &str,
        invalid_only: bool,
        endpoints: Option<&[String]>,
        validations: Option<&[ValidationIssue]>,
    ) -> Markdown {
        let visitor = PathToMarkdownVisitor {
            invalid_only,
            endpoints,
            validations,
            added: RefCell::new(vec![]),
            updated: RefCell::new(vec![]),
            removed: RefCell::new(vec![]),
        };

        dispatch_visitor(self, &visitor);

        let mut markdown = String::new();

        let added = visitor.added.borrow();
        let updated = visitor.updated.borrow();
        let removed = visitor.removed.borrow();

        let is_unchanged =
            added.is_empty() && updated.is_empty() && removed.is_empty();
        if !is_unchanged {
            markdown.push_str("*API Schema diff*\n");

            info.iter().for_each(|(field, value)| {
                markdown.push_str(&format!("{field}: *{value}*\n"))
            });

            let now = chrono::Utc::now()
                .to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
            markdown.push_str(&format!("Generated at: *{now} UTC*\n"));
        }

        if added.len() > 0 {
            markdown.push_str(&format!("\n*Added ({})*\n", added.len()));
            for (path, _, breaking) in added.iter() {
                markdown.push_str(&format_path(path, *breaking, version_url));
            }
        }

        if updated.len() > 0 {
            markdown.push_str(&format!("\n*Updated ({})*\n", updated.len()));
            for (path, _, breaking) in updated.iter() {
                markdown.push_str(&format_path(path, *breaking, version_url));
            }
        }

        if removed.len() > 0 {
            markdown.push_str(&format!("\n*Removed ({})*\n", removed.len()));
            for (path, _, breaking) in removed.iter() {
                markdown.push_str(&format_path(path, *breaking, version_url));
            }
        }

        Markdown::new(markdown, is_unchanged)
    }
}

fn format_path(
    path: &PathPointer,
    breaking: bool,
    version_url: &str,
) -> String {
    let breaking = if breaking { "!" } else { "-" };

    let url = format!("{}#{}", version_url, path.get_path());

    let method = display_method(path).to_uppercase();
    let uri = display_uri(path);

    format!(" {breaking} `{method:^8}` `{uri}` <{url}|view>\n")
}

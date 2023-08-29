use schemadoc_diff::core::DiffResult;
use schemadoc_diff::path_pointer::PathPointer;
use schemadoc_diff::schema_diff::{HttpSchemaDiff, OperationDiff};
use schemadoc_diff::visitor::{dispatch_visitor, DiffVisitor};
use serde::{Deserialize, Serialize};
use std::cell::Cell;

struct StatisticsVisitor {
    total: Cell<u32>,
    added: Cell<u32>,
    removed: Cell<u32>,
    updated: Cell<u32>,
}

impl<'s> DiffVisitor<'s> for StatisticsVisitor {
    fn visit_operation(
        &self,
        pointer: &PathPointer,
        _: &str,
        _: &'s DiffResult<OperationDiff>,
    ) -> bool {
        if !pointer.is_removed() {
            self.total.set(self.total.get() + 1)
        }

        if pointer.is_added() {
            self.added.set(self.added.get() + 1);
        } else if pointer.is_updated() {
            self.updated.set(self.updated.get() + 1);
        } else if pointer.is_removed() {
            self.removed.set(self.removed.get() + 1);
        }
        // do not go deeper
        false
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiffStatistics {
    pub total: u32,
    pub added: u32,
    pub removed: u32,
    pub updated: u32,
}

pub fn get_diff_statistics(diff: &HttpSchemaDiff) -> DiffStatistics {
    let visitor = StatisticsVisitor {
        total: Cell::new(0),
        added: Cell::new(0),
        removed: Cell::new(0),
        updated: Cell::new(0),
    };

    dispatch_visitor(diff, &visitor);

    DiffStatistics {
        total: visitor.total.get(),
        added: visitor.added.get(),
        removed: visitor.removed.get(),
        updated: visitor.updated.get(),
    }
}

#[cfg(test)]
mod test {
    use crate::versions::statistics::get_diff_statistics;

    #[test]
    fn test_diff_statistics() {
        let src_content = r#"
            {
                "openapi": "3.0.3",
                "paths": {
                    "/path1": {
                        "get": {},
                        "post": {},
                        "delete":{}
                    },
                    "/path2": {
                        "delete": {},
                        "patch": {},
                        "put": {},
                        "post": {}
                    }
                }
            }
        "#;

        let tgt_content = r#"
            {
                "openapi": "3.0.3",
                "paths": {
                    "/path1": {
                        "post": {},
                        "delete": {"description": "updated path operation"}
                    },
                    "/path3": {
                        "put":{},
                        "post":{}
                    }
                }
            }
        "#;

        let (src_schema, tgt_schema) =
            schemadoc_diff::try_deserialize_schema(src_content, tgt_content).unwrap();

        let diff = schemadoc_diff::get_schema_diff(src_schema, tgt_schema);

        let statistics = get_diff_statistics(&diff.get().unwrap());

        assert_eq!(statistics.total, 4);
        assert_eq!(statistics.added, 2);
        assert_eq!(statistics.updated, 1);
        assert_eq!(statistics.removed, 5);
    }
}

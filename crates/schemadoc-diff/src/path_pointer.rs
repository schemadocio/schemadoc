use crate::diff_result_type::DiffResultType;

#[derive(Debug, Clone)]
pub enum PointerAncestor {
    Scope(PathPointerScope),
    Relative(usize),
}

impl PointerAncestor {
    pub fn parent() -> Self {
        Self::Relative(1)
    }

    pub fn path() -> Self {
        Self::Scope(PathPointerScope::Path)
    }

    pub fn operation() -> Self {
        Self::Scope(PathPointerScope::Operation)
    }

    pub fn schema() -> Self {
        Self::Scope(PathPointerScope::Schema)
    }

    pub fn schema_property() -> Self {
        Self::Scope(PathPointerScope::SchemaProperty)
    }

    pub fn schema_properties() -> Self {
        Self::Scope(PathPointerScope::SchemaProperties)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PathPointerScope {
    Paths,
    Path,
    Operation,

    RequestBody,
    Responses,
    ResponseCode,
    Parameters,

    MediaType,

    Schema,
    SchemaProperties,
    SchemaProperty,

    SchemaItems,
    SchemaNot,

    SchemaAllOf,
    SchemaAnyOf,
    SchemaOneOf,
    SchemaAdditionalProperties,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathPointerComponent {
    pub kind: DiffResultType,
    pub path: Option<String>,
    pub scope: Option<PathPointerScope>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PathPointer {
    pub components: Vec<PathPointerComponent>,
}

impl PathPointer {
    pub fn new<S: Into<String>, C: Into<DiffResultType>>(
        context: C,
        path: Option<S>,
        scope: Option<PathPointerScope>,
    ) -> Self {
        Self {
            components: vec![PathPointerComponent {
                scope,
                kind: context.into(),
                path: path.map(|path| path.into()),
            }],
        }
    }

    pub fn add<S: Into<String>, C: Into<DiffResultType>>(
        &self,
        context: C,
        path: S,
        scope: Option<PathPointerScope>,
    ) -> Self {
        self.add_component(context, Some(path), scope)
    }

    pub fn add_context<C: Into<DiffResultType>>(&self, context: C) -> Self {
        self.add_component::<String, C>(context, None, None)
    }

    pub fn add_component<S: Into<String>, C: Into<DiffResultType>>(
        &self,
        context: C,
        path: Option<S>,
        scope: Option<PathPointerScope>,
    ) -> Self {
        let mut new = self.clone();
        new.components.push(PathPointerComponent {
            scope,
            kind: context.into(),
            path: path.map(|path| path.into()),
        });
        new
    }

    pub fn get(
        &self,
        scope: PathPointerScope,
    ) -> Option<&PathPointerComponent> {
        self.components
            .iter()
            .find(|c| c.scope.as_ref().map_or(false, |v| v == &scope))
    }

    pub fn is_in(&self, scope: PathPointerScope) -> bool {
        self.get(scope).is_some()
    }

    pub fn this(&self) -> DiffResultType {
        self.get_primary(None)
    }

    pub fn parent(&self) -> DiffResultType {
        self.get_primary(Some(PointerAncestor::parent()))
    }

    pub fn ancestor(&self, ancestor: PointerAncestor) -> DiffResultType {
        self.get_primary(Some(ancestor))
    }

    fn get_primary(
        &self,
        ancestor: Option<PointerAncestor>,
    ) -> DiffResultType {
        let mut found = true;
        let mut result = DiffResultType::None;

        if let Some(ancestor) = ancestor {
            match ancestor {
                PointerAncestor::Scope(ref target_scope) => {
                    let latest = self
                        .components
                        .iter()
                        .rfind(|c| c.scope.as_ref() == Some(target_scope));
                    found = match latest {
                        Some(latest) => {
                            let mut is_latest_found = false;
                            let iterator =
                                self.components.iter().take_while(|c| {
                                    if is_latest_found {
                                        return false;
                                    }
                                    is_latest_found |= *c == latest;
                                    true
                                });

                            for PathPointerComponent { kind, .. } in iterator {
                                if kind.is_added() | kind.is_removed() {
                                    return *kind;
                                } else if kind.is_same() || kind.is_updated() {
                                    result = *kind;
                                }
                            }
                            true
                        }
                        None => false,
                    };
                }
                PointerAncestor::Relative(value) => {
                    let slice =
                        &self.components[..(self.components.len() - value)];
                    for PathPointerComponent { kind, .. } in slice {
                        if kind.is_added() | kind.is_removed() {
                            return *kind;
                        } else if kind.is_same() || kind.is_updated() {
                            result = *kind;
                        }
                    }
                }
            }
        } else {
            for PathPointerComponent { kind, .. } in self.components.iter() {
                if kind.is_added() | kind.is_removed() {
                    return *kind;
                } else if kind.is_same() || kind.is_updated() {
                    result = *kind;
                }
            }
        };

        if found {
            result
        } else {
            DiffResultType::None
        }
    }

    pub fn is_same(&self) -> bool {
        self.this().is_same()
    }

    pub fn is_added(&self) -> bool {
        self.this().is_added()
    }

    pub fn is_updated(&self) -> bool {
        self.this().is_updated()
    }

    pub fn is_upserted(&self) -> bool {
        self.this().is_upserted()
    }

    pub fn is_removed(&self) -> bool {
        self.this().is_removed()
    }

    pub fn get_path(&self) -> String {
        self.components
            .iter()
            .filter_map(|c| c.path.clone())
            .collect::<Vec<String>>()
            .join("/")
    }

    pub fn startswith(&self, value: &PathPointer) -> bool {
        self.get_path().starts_with(&value.get_path())
    }

    pub fn matches(&self, value: &str) -> bool {
        if value == "*" {
            true
        } else {
            value.trim_matches('/') == self.get_path().trim_matches('/')
        }
    }
}

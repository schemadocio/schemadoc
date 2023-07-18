use crate::core::DiffResult;

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DiffResultType {
    None,
    Same,
    Added,
    Updated,
    Removed,
}

impl DiffResultType {
    pub fn is_none(&self) -> bool {
        matches!(self, DiffResultType::None)
    }

    pub fn is_same(&self) -> bool {
        matches!(self, DiffResultType::Same)
    }

    pub fn is_added(&self) -> bool {
        matches!(self, DiffResultType::Added)
    }

    pub fn is_updated(&self) -> bool {
        matches!(self, DiffResultType::Updated)
    }

    pub fn is_removed(&self) -> bool {
        matches!(self, DiffResultType::Removed)
    }

    pub fn is_upserted(&self) -> bool {
        self.is_added() || self.is_updated()
    }
}

impl<T> From<&DiffResult<T>> for DiffResultType {
    fn from(diff: &DiffResult<T>) -> Self {
        match diff {
            DiffResult::None => DiffResultType::None,
            DiffResult::Same(_) => DiffResultType::Same,
            DiffResult::Added(_) => DiffResultType::Added,
            DiffResult::Updated(_, _) => DiffResultType::Updated,
            DiffResult::Removed(_) => DiffResultType::Removed,
        }
    }
}

impl<T> From<DiffResult<&T>> for DiffResultType {
    fn from(diff: DiffResult<&T>) -> Self {
        match diff {
            DiffResult::None => DiffResultType::None,
            DiffResult::Same(_) => DiffResultType::Same,
            DiffResult::Added(_) => DiffResultType::Added,
            DiffResult::Updated(_, _) => DiffResultType::Updated,
            DiffResult::Removed(_) => DiffResultType::Removed,
        }
    }
}

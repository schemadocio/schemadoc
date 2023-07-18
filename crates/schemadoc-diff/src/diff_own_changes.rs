use crate::core::{DiffResult, MapDiff, VecDiff};
use crate::diff_result_type::DiffResultType;
use std::borrow::Cow;

pub trait DiffOwnChanges {
    fn get_own_changes(&self) -> Vec<(Cow<str>, DiffResultType)>;
}

impl<T> DiffOwnChanges for DiffResult<T>
where
    T: DiffOwnChanges,
{
    fn get_own_changes(&self) -> Vec<(Cow<str>, DiffResultType)> {
        self.get().map_or(vec![], |v| v.get_own_changes())
    }
}

impl<T> DiffOwnChanges for VecDiff<T> {
    fn get_own_changes(&self) -> Vec<(Cow<str>, DiffResultType)> {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(idx, e)| {
                if e.is_added() || e.is_removed() {
                    Some((idx.to_string().into(), e.into()))
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T, R> DiffOwnChanges for MapDiff<T, R> {
    fn get_own_changes(&self) -> Vec<(Cow<str>, DiffResultType)> {
        self.0
            .iter()
            .filter_map(|(key, e)| {
                if e.is_added() || e.is_removed() {
                    Some((key.into(), e.into()))
                } else {
                    None
                }
            })
            .collect()
    }
}

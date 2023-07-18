use crate::core::{DiffResult, PathResolver, VecDiffTransformer};
use regex::Regex;
use std::collections::HashMap;

/// Moves `null` in `schema.type` to the end of array
#[derive(Debug, Default, Clone)]
pub struct TypeVecDiffSorter;

impl VecDiffTransformer<Vec<DiffResult<String>>> for TypeVecDiffSorter {
    fn transform(
        mut vector: Vec<DiffResult<String>>,
    ) -> Vec<DiffResult<String>> {
        let index = vector.iter().position(|v| match v.get() {
            Some(value) => value == "null",
            None => false,
        });

        if let Some(index) = index {
            let value = vector.remove(index);
            vector.push(value)
        }

        vector
    }
}

/// Tries to find paths from `.paths` with same endpoints and different path's parameter names
/// Example:
///     `v1/datasets/{name}/tags` considers the same endpoint as `v1/datasets/{uuid}/tags`
/// Right now we do not care about parameter type when merging
#[derive(Debug, Clone)]
pub struct PathsMapPathResolver(
    HashMap<String, String>,
    HashMap<String, String>,
);

fn get_key(key: &str) -> String {
    let re = Regex::new(r"\{.+?}").unwrap();
    re.replace_all(key, "$").to_string()
}

impl PathResolver for PathsMapPathResolver {
    fn new<'a, T>(k1: T, k2: T) -> Self
    where
        T: Iterator<Item = &'a String>,
    {
        let keys1: HashMap<_, _> =
            k1.map(|key| (get_key(key), key.clone())).collect();

        let keys2: HashMap<_, _> =
            k2.map(|key| (get_key(key), key.clone())).collect();

        Self(keys1, keys2)
    }

    fn k1tok2(&self, k1: &String) -> String {
        let key = get_key(k1);
        self.1.get(&key).unwrap_or(k1).to_owned()
    }

    fn k2tok1(&self, k2: &String) -> String {
        let key = get_key(k2);
        self.0.get(&key).unwrap_or(k2).to_owned()
    }
}

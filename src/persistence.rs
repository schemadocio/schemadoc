use serde::{Deserialize, Serialize};
use std::io;
use std::path::PathBuf;

use crate::storage::Storer;

#[derive(thiserror::Error, Debug)]
pub enum PersistenceError {
    #[error("IO error")]
    Io(#[from] io::Error),
    #[error("Deserialization error")]
    Serde(#[from] serde_yaml::Error),
}

pub trait Versioned {
    // fn migrate(self) -> Self;
    fn latest() -> &'static str;
    // fn migrations() -> &'static HashMap<&'static str, fn(Value) -> (Value, &'static str)>;
}

pub trait PersistentData<T> {
    fn version(&self) -> &str;
    fn data(self) -> T;
    fn new(version: impl Into<String>, data: T) -> Self;
}

#[derive(Serialize, Deserialize)]
pub struct PersistentDataFile<T> {
    pub version: String,
    pub data: T,
}

impl<T> PersistentData<T> for PersistentDataFile<T> {
    fn version(&self) -> &str {
        &self.version
    }

    fn data(self) -> T {
        self.data
    }

    fn new(version: impl Into<String>, data: T) -> Self {
        PersistentDataFile {
            version: version.into(),
            data,
        }
    }
}

pub async fn persist_data_file<'s, T, S, P, C>(
    storage: &S,
    path: P,
    state: &'s T,
) -> Result<(), PersistenceError>
    where
        T: Versioned,
        S: Storer,
        P: Into<PathBuf>,
        C: PersistentData<&'s T> + Serialize
{
    let path = path.into();

    let file = C::new(T::latest(), state);

    let data = serde_yaml::to_string(&file)?;

    storage.put_file(&path, data.as_bytes()).await?;

    Ok(())
}

pub async fn load_data_file<T, S, P, C>(storage: &S, path: P) -> Result<T, PersistenceError>
    where
        T: Versioned + Default, // + for<'a> Deserialize<'a>,
        S: Storer,
        P: Into<PathBuf>,
        C: PersistentData<T> + for<'a> Deserialize<'a>
{
    let path = path.into();

    let state = if storage.exists(&path).await? {
        let data = storage.read_file(&path).await?;
        let state: C = serde_yaml::from_slice(data.as_slice())?;
        state.data()
    } else {
        Default::default()
    };

    Ok(state)
}

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

#[derive(Serialize, Deserialize)]
pub struct PersistentDataFile<T> {
    pub version: String,
    pub data: T,
}

pub async fn persist_data_file<T, S, P>(
    storage: &S,
    path: P,
    state: &T,
) -> Result<(), PersistenceError>
where
    T: Versioned + Serialize,
    S: Storer,
    P: Into<PathBuf>,
{
    let path = path.into();

    let file = PersistentDataFile {
        version: T::latest().to_string(),
        data: state,
    };

    let data = serde_yaml::to_string(&file)?;

    storage.put_file(&path, data.as_bytes()).await?;

    Ok(())
}

pub async fn load_data_file<T, S, P>(storage: &S, path: P) -> Result<T, PersistenceError>
where
    T: Versioned + Default + for<'a> Deserialize<'a>,
    S: Storer,
    P: Into<PathBuf>,
{
    let path = path.into();

    let state = if storage.exists(&path).await? {
        let data = storage.read_file(&path).await?;
        let state: PersistentDataFile<T> = serde_yaml::from_slice(data.as_slice())?;
        state.data
    } else {
        Default::default()
    };

    Ok(state)
}

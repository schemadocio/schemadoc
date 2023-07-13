use std::io;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;

#[async_trait::async_trait]
pub trait Storer {
    async fn put_file<P: AsRef<Path> + Send>(&self, path: P, file: &[u8]) -> io::Result<()>;
    async fn read_file<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<Vec<u8>>;
    async fn remove_file<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<()>;
    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<bool>;
}

#[derive(Debug, Clone)]
pub struct LocalStorage {
    location: PathBuf,
}

impl LocalStorage {
    pub fn new<L: Into<PathBuf>>(location: L) -> Self {
        Self {
            location: location.into(),
        }
    }
}

#[async_trait::async_trait]
impl Storer for LocalStorage {
    async fn put_file<P: AsRef<Path> + Send>(&self, path: P, file: &[u8]) -> io::Result<()> {
        let path = self.location.join(path);
        // Create intermediate folders
        if let Some(folder) = path.parent() {
            tokio::fs::create_dir_all(folder).await?;
        }

        let mut fd = tokio::fs::File::create(path).await?;

        fd.write_all(file).await?;
        fd.sync_all().await
    }

    async fn read_file<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<Vec<u8>> {
        let path = self.location.join(path);
        println!("Read from local storage: {:?}", path);
        tokio::fs::read(path).await
    }

    async fn remove_file<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<()> {
        let path = self.location.join(path);
        tokio::fs::remove_file(path).await
    }

    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<bool> {
        let path = self.location.join(path);
        tokio::fs::try_exists(path).await
    }
}

#[derive(Debug, Clone)]
pub enum Storage {
    Local(LocalStorage),
}

#[async_trait::async_trait]
impl Storer for Storage {
    async fn put_file<P: AsRef<Path> + Send>(&self, path: P, file: &[u8]) -> io::Result<()> {
        match self {
            Storage::Local(ls) => ls.put_file(path, file),
        }
        .await
    }

    async fn read_file<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<Vec<u8>> {
        match self {
            Storage::Local(ls) => ls.read_file(path),
        }
        .await
    }

    async fn remove_file<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<()> {
        match self {
            Storage::Local(ls) => ls.remove_file(path),
        }
        .await
    }

    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> io::Result<bool> {
        match self {
            Storage::Local(ls) => ls.exists(path),
        }
        .await
    }
}

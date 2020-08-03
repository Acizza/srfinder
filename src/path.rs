use anyhow::Result;
use once_cell::sync::Lazy;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Copy, Clone)]
pub enum FilePath {
    LocalData,
}

impl FilePath {
    fn path(self) -> PathBuf {
        static LOCAL_DATA_PATH: Lazy<PathBuf> = Lazy::new(|| {
            let mut dir =
                dirs_next::data_local_dir().unwrap_or_else(|| PathBuf::from("~/.local/share/"));
            dir.push(env!("CARGO_PKG_NAME"));
            dir
        });

        match self {
            Self::LocalData => LOCAL_DATA_PATH.clone(),
        }
    }

    pub fn validated_subdir<P>(self, subdir: P) -> Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let mut dir = self.path();
        dir.push(subdir);
        validate_dir(&dir)?;
        Ok(dir)
    }
}

pub fn validate_dir<P>(dir: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let dir = dir.as_ref();

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    Ok(())
}

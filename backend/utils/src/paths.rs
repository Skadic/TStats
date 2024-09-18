use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use const_format::concatcp;

const BASE_TOURNAMENT_PATH: &str = "tournament/";
const BASE_TOURNAMENT_BANNER_PATH: &str = concatcp!(BASE_TOURNAMENT_PATH, "banner/");

const PATHS: [&str; 1] = [BASE_TOURNAMENT_BANNER_PATH];

/// Can provide full paths to any TStats data.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct TStatsPaths {
    base_path: PathBuf,
}

impl TStatsPaths {
    /// The function `new` creates a new instance with a base path specified by the input parameter.
    ///
    /// Arguments:
    ///
    /// * `path`: The base path where all TStats data is put.
    ///  
    pub fn new<T: AsRef<Path>>(path: T) -> std::io::Result<Self> {
        let base_path = match path.as_ref().to_owned().canonicalize() {
            Ok(path) => path,
            Err(err) if err.kind() == ErrorKind::NotFound => {
                std::fs::create_dir_all(&path)?;
                path.as_ref().to_owned().canonicalize()?
            }
            error => error?,
        };
        for path in PATHS.iter().map(|cur| base_path.join(cur)) {
            std::fs::create_dir_all(path)?;
        }
        Ok(Self { base_path })
    }

    /// Get the full path to the tournament banner with the given file name.
    ///
    /// Arguments:
    ///
    /// * `file_name`: The file name of the banner.
    ///
    pub fn banner(&self, file_name: &impl AsRef<Path>) -> PathBuf {
        self.base_path
            .join(BASE_TOURNAMENT_BANNER_PATH)
            .join(file_name)
    }

    /// Returns the base path of all TStats storage
    pub fn base(&self) -> &Path {
        self.base_path.as_path()
    }
}

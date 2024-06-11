use std::path::PathBuf;

use log::info;

#[derive(Debug)]
pub enum FindFilesError {
    PatternError(glob::PatternError),
    GlobError(glob::GlobError),
}

impl std::fmt::Display for FindFilesError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::PatternError(ref err) => {
                write!(f, "Failed to build pattern: {err}")
            }
            Self::GlobError(ref err) => {
                write!(f, "Failed to read file: {err}")
            }
        }
    }
}

impl std::convert::From<glob::PatternError> for FindFilesError {
    fn from(e: glob::PatternError) -> Self {
        Self::PatternError(e)
    }
}
impl std::convert::From<glob::GlobError> for FindFilesError {
    fn from(e: glob::GlobError) -> Self {
        Self::GlobError(e)
    }
}

/// Given a list of patterns or paths, along with exclusion patterns, find matching files.
pub fn find_paths(
    path_patterns: &[String],
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>, FindFilesError> {
    let mut matched_paths = vec![];
    let exclude_paths: Vec<_> = exclude_patterns
        .iter()
        .map(|x| glob::Pattern::new(x))
        .collect::<Result<_, _>>()?;
    for path in path_patterns
        .iter()
        .flat_map(|x| glob::glob(x))
        .flatten()
        .collect::<Result<Vec<_>, _>>()?
    {
        if path.is_dir() {
            info!("skipping directory path: {}", path.display());
            continue;
        }
        if let Some(pattern) = exclude_paths
            .iter()
            .find(|&excluded| excluded.matches(path.to_str().unwrap()))
        {
            info!(
                "skipping excluded file path: {}. pattern: {}",
                path.display(),
                pattern
            );

            continue;
        }
        matched_paths.push(path);
    }
    Ok(matched_paths)
}

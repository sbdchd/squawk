use anyhow::Result;
use std::path::PathBuf;

use log::info;

/// Given a list of patterns or paths, along with exclusion patterns, find matching files.
pub(crate) fn find_paths(
    path_patterns: &[String],
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>> {
    let mut matched_paths = vec![];
    let exclude_paths: Vec<_> = exclude_patterns
        .iter()
        .map(|x| glob::Pattern::new(x))
        .collect::<Result<_, _>>()?;
    let paths = path_patterns
        .iter()
        .map(|x| glob::glob(x))
        .collect::<Result<Vec<_>, _>>()?;
    for path in paths.into_iter().flatten().collect::<Result<Vec<_>, _>>()? {
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

#[cfg(test)]
mod tests {
    use super::find_paths;

    #[test]
    fn valid_include_and_exclude_patterns_still_match() {
        let directory = tempfile::tempdir().unwrap();
        let file = directory.path().join("migration.sql");
        std::fs::write(&file, "select 1;").unwrap();
        let patterns = [directory
            .path()
            .join("*.sql")
            .to_string_lossy()
            .into_owned()];

        assert_eq!(find_paths(&patterns, &[]).unwrap(), vec![file]);
        assert!(
            find_paths(&patterns, &["**/*.sql".to_owned()])
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn invalid_include_pattern_returns_error() {
        assert!(find_paths(&["[".to_owned()], &[]).is_err());
    }
}

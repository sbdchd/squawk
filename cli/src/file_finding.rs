use std::path::PathBuf;

use log::info;

/// Given a list of patterns or paths, along with exclusion patterns, find matching files.
pub fn find_paths(path_patterns: &[String], exclude_patterns: &[String]) -> Vec<PathBuf> {
    let mut matched_paths = vec![];
    let exclude_paths: Vec<_> = exclude_patterns
        .iter()
        .map(|x| glob::Pattern::new(x).unwrap())
        .collect();
    for path in path_patterns
        .iter()
        .flat_map(|x| glob::glob(x).unwrap().filter_map(Result::ok))
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
    matched_paths
}

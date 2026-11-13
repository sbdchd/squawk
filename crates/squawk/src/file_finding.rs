use anyhow::Result;
use ignore::WalkBuilder;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use log::info;

// based on https://github.com/astral-sh/ruff/blob/2742f956d87314cf1522c69c9a95a71d972bd76c/crates/ruff_workspace/src/settings.rs#L118-L144
const DEFAULT_EXCLUDES: &[&str] = &[
    ".bzr",
    ".direnv",
    ".git",
    ".git-rewrite",
    ".hg",
    ".svn",
    ".venv",
    ".vscode",
    "_build",
    "buck-out",
    "dist",
    "node_modules",
    "venv",
];

/// Given a list of patterns or paths, along with exclusion patterns, find matching files.
pub(crate) fn find_paths(
    path_patterns: &[String],
    exclude_patterns: &[String],
) -> Result<Vec<PathBuf>> {
    let mut matched_paths = vec![];
    let excludes: Arc<[glob::Pattern]> = exclude_patterns
        .iter()
        .map(|x| glob::Pattern::new(x))
        .collect::<Result<_, _>>()?;
    for pattern in path_patterns {
        let pattern_path = Path::new(pattern);
        if pattern_path.is_dir() {
            walk_directory(pattern_path, &excludes, &mut matched_paths)?;
            continue;
        }
        for path in glob::glob(pattern)? {
            let path = path?;
            if path.is_dir() {
                info!("skipping directory path: {}", path.display());
                continue;
            }
            if let Some(pattern) = path
                .ancestors()
                .find_map(|ancestor| user_exclude(ancestor, &excludes))
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
    }
    matched_paths.sort_unstable();
    matched_paths.dedup();
    Ok(matched_paths)
}

fn walk_directory(
    directory: &Path,
    excludes: &Arc<[glob::Pattern]>,
    matched_paths: &mut Vec<PathBuf>,
) -> Result<()> {
    let excludes = Arc::clone(excludes);
    let walker = WalkBuilder::new(directory)
        .hidden(false)
        .filter_entry(move |entry| entry.depth() == 0 || !is_excluded(entry.path(), &excludes))
        .build();
    for entry in walker {
        let entry = entry?;
        if entry.depth() == 0 || entry.file_type().is_none_or(|ft| ft.is_dir()) {
            continue;
        }
        if entry.path().extension().is_some_and(|ext| ext == "sql") {
            matched_paths.push(entry.into_path());
        }
    }
    Ok(())
}

fn is_excluded(path: &Path, excludes: &[glob::Pattern]) -> bool {
    if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| DEFAULT_EXCLUDES.contains(&name))
    {
        info!("skipping path via default excludes: {}", path.display());
        return true;
    }
    if let Some(pattern) = user_exclude(path, excludes) {
        info!(
            "skipping excluded path: {}. pattern: {}",
            path.display(),
            pattern
        );
        return true;
    }
    false
}

fn user_exclude<'a>(path: &Path, excludes: &'a [glob::Pattern]) -> Option<&'a glob::Pattern> {
    let file_name = path.file_name().map(Path::new);
    excludes.iter().find(|excluded| {
        excluded.matches_path(path) || file_name.is_some_and(|name| excluded.matches_path(name))
    })
}

#[cfg(test)]
mod tests {
    use super::find_paths;
    use std::path::{Path, PathBuf};

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

    fn write_files(root: &Path, files: &[&str]) -> Vec<PathBuf> {
        files
            .iter()
            .map(|file| {
                let path = root.join(file);
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                std::fs::write(&path, "select 1;").unwrap();
                path
            })
            .collect()
    }

    fn dir_pattern(directory: &Path) -> String {
        format!("{}/", directory.display())
    }

    #[test]
    fn directory_is_walked_for_sql_files() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let files = write_files(
            root,
            &[".hidden/c.sql", "a.sql", "nested/b.sql", "nested/skip.sql"],
        );
        write_files(root, &["notes.txt"]);

        assert_eq!(
            find_paths(&[dir_pattern(root)], &["skip.sql".to_owned()]).unwrap(),
            files[..3]
        );
    }

    #[test]
    fn directory_walk_respects_gitignore_and_default_excludes() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        std::fs::create_dir(root.join(".git")).unwrap();
        std::fs::write(root.join(".gitignore"), "generated/\n").unwrap();
        let files = write_files(
            root,
            &["generated/a.sql", "keep.sql", "node_modules/pkg/b.sql"],
        );

        assert_eq!(
            find_paths(&[dir_pattern(root)], &[]).unwrap(),
            vec![files[1].clone()]
        );
    }

    #[test]
    fn excluded_directory_is_skipped() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let files = write_files(root, &["a.sql", "vendor/b.sql"]);
        let excludes = ["vendor".to_owned()];

        assert_eq!(
            find_paths(&[dir_pattern(root)], &excludes).unwrap(),
            vec![files[0].clone()]
        );
        assert!(
            find_paths(&[format!("{}/vendor/*.sql", root.display())], &excludes)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn overlapping_inputs_are_deduplicated() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let files = write_files(root, &["a.sql"]);
        let patterns = [dir_pattern(root), files[0].display().to_string()];

        assert_eq!(find_paths(&patterns, &[]).unwrap(), files);
    }

    #[test]
    fn invalid_include_pattern_returns_error() {
        assert!(find_paths(&["[".to_owned()], &[]).is_err());
    }
}

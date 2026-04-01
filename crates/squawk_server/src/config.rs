use squawk_linter::config::ConfigFile;
use squawk_linter::{Linter, Rule, Version};
use std::path::PathBuf;
use std::sync::Arc;

/// Linter-relevant config extracted from `.squawk.toml`, stored once
/// in `GlobalState` and shared via Arc into Snapshots.
pub(crate) struct LintConfig {
    pub excluded_rules: Vec<Rule>,
    pub pg_version: Option<Version>,
    pub assume_in_transaction: bool,
    pub excluded_paths: Vec<glob::Pattern>,
    pub workspace_root: Option<PathBuf>,
}

impl LintConfig {
    pub fn from_config_file(
        config: Option<ConfigFile>,
        workspace_root: Option<PathBuf>,
    ) -> Arc<Self> {
        let config = config.unwrap_or_default();
        let excluded_paths = config
            .excluded_paths
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();
        Arc::new(Self {
            excluded_rules: config.excluded_rules,
            pg_version: config.pg_version,
            assume_in_transaction: config.assume_in_transaction.unwrap_or(false),
            excluded_paths,
            workspace_root,
        })
    }

    pub fn new_linter(&self) -> Linter {
        let mut linter = Linter::without_rules(&self.excluded_rules);
        if let Some(pg_version) = self.pg_version {
            linter.settings.pg_version = pg_version;
        }
        linter.settings.assume_in_transaction = self.assume_in_transaction;
        linter
    }
}

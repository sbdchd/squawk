use lsp_types::InitializeParams;
use squawk_linter::config::ConfigFile;
use squawk_linter::{Linter, Rule, Version};
use std::path::PathBuf;
use std::sync::Arc;

/// Linter-relevant config extracted from `.squawk.toml`, stored once
/// in `GlobalState` and shared via Arc into Snapshots.
#[derive(Default)]
pub(crate) struct LintConfig {
    pub excluded_rules: Vec<Rule>,
    pub included_rules: Vec<Rule>,
    pub pg_version: Option<Version>,
    pub assume_in_transaction: bool,
    pub excluded_paths: Vec<glob::Pattern>,
    pub workspace_root: Option<PathBuf>,
}

impl LintConfig {
    /// Resolve the lint config from LSP initialization parameters by locating
    /// the workspace root and parsing any `.squawk.toml` found above it.
    pub fn from_init_params(init_params: &InitializeParams) -> Arc<Self> {
        let workspace_root = workspace_root_from_init_params(init_params);
        let config = workspace_root.as_ref().and_then(|root| {
            ConfigFile::find_and_parse(root)
                .map_err(|e| log::warn!("error loading config: {e}"))
                .ok()
                .flatten()
        });
        Self::build(config.unwrap_or_default(), workspace_root)
    }

    fn build(config: ConfigFile, workspace_root: Option<PathBuf>) -> Arc<Self> {
        let excluded_paths = config
            .excluded_paths
            .iter()
            .filter_map(|p| glob::Pattern::new(p).ok())
            .collect();
        Arc::new(Self {
            excluded_rules: config.excluded_rules,
            included_rules: config.included_rules,
            pg_version: config.pg_version,
            assume_in_transaction: config.assume_in_transaction.unwrap_or(false),
            excluded_paths,
            workspace_root,
        })
    }

    pub fn new_linter(&self) -> Linter {
        let mut linter = Linter::with_rules(&self.included_rules, &self.excluded_rules);
        if let Some(pg_version) = self.pg_version {
            linter.settings.pg_version = pg_version;
        }
        linter.settings.assume_in_transaction = self.assume_in_transaction;
        linter
    }
}

/// Resolve the workspace root from LSP `InitializeParams`, preferring the
/// modern `workspace_folders` field and falling back to the deprecated
/// `root_uri` and `root_path` for older clients.
fn workspace_root_from_init_params(init_params: &InitializeParams) -> Option<PathBuf> {
    if let Some(folder) = init_params
        .workspace_folders
        .as_ref()
        .and_then(|folders| folders.first())
    {
        if let Ok(path) = folder.uri.to_file_path() {
            return Some(path);
        }
    }

    #[allow(deprecated)]
    {
        if let Some(path) = init_params
            .root_uri
            .as_ref()
            .and_then(|uri| uri.to_file_path().ok())
        {
            return Some(path);
        }
        init_params.root_path.as_ref().map(PathBuf::from)
    }
}

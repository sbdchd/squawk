use anyhow::{Context, Result, bail};
use clap::Args;
use jiff::Zoned;
use regex::Regex;
use xshell::{Shell, cmd};

use crate::path::project_root;

#[derive(Args, Debug)]
pub(crate) struct UpdateVersionArgs {
    /// New version, e.g. 2.51.0. If omitted, auto-increments the minor version.
    new_version: Option<String>,
}

pub(crate) fn update_version(args: UpdateVersionArgs) -> Result<()> {
    let sh = Shell::new()?;
    sh.change_dir(project_root());

    let new_version = match args.new_version {
        Some(v) => v,
        None => auto_increment_version(&sh)?,
    };

    cmd!(sh, "git switch master")
        .run()
        .context("Failed to switch to master branch")?;

    let release_branch = format!("release-{new_version}");
    if cmd!(sh, "git switch {release_branch}")
        .ignore_stdout()
        .ignore_stderr()
        .run()
        .is_err()
    {
        cmd!(sh, "git switch -c {release_branch}").run()?;
    }

    update_versions(&sh, &new_version)?;

    println!("Updating CHANGELOG.md...");
    let current_date = Zoned::now().strftime("%Y-%m-%d").to_string();

    let tags = cmd!(sh, "git tag --sort=-version:refname").read()?;
    let latest_tag = tags
        .lines()
        .next()
        .filter(|t| !t.is_empty())
        .context("No previous git tag found")?;
    println!("Fetching commits since {latest_tag}...");
    let pretty = "format:- %s";
    let commits = cmd!(sh, "git log {latest_tag}..master --pretty={pretty}").read()?;

    update_changelog(&sh, &new_version, &current_date, &commits)?;

    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
    let mut editor_parts = editor.split_whitespace();
    let program = editor_parts.next().context("EDITOR is empty")?;
    let extra_args: Vec<&str> = editor_parts.collect();
    cmd!(sh, "{program} {extra_args...} CHANGELOG.md").run()?;

    Ok(())
}

fn auto_increment_version(sh: &Shell) -> Result<String> {
    let cargo_toml = sh.read_file(project_root().join("Cargo.toml"))?;
    let parsed: toml::Value = toml::from_str(&cargo_toml)?;
    let version = parsed
        .get("workspace")
        .and_then(|w| w.get("package"))
        .and_then(|p| p.get("version"))
        .and_then(|v| v.as_str())
        .context("workspace.package.version not found in Cargo.toml")?;

    let parts: Vec<&str> = version.split('.').collect();
    let [major, minor, _patch] = parts.as_slice() else {
        bail!("invalid version format: {version}");
    };
    let minor: u32 = minor.parse().context("invalid minor version")?;
    let next = format!("{major}.{}.0", minor + 1);
    println!("Auto-incrementing to {next}");
    Ok(next)
}

fn update_versions(sh: &Shell, v: &str) -> Result<()> {
    let version_rep = format!(r#"version = "{v}""#);
    replace_in_file(sh, "Cargo.toml", r#"(?m)^version = ".*""#, &version_rep)?;
    replace_in_file(
        sh,
        "Cargo.toml",
        r#"(squawk-[a-z_]+ = \{ path = "[^"]+", )version = "[^"]+""#,
        &format!(r#"${{1}}version = "{v}""#),
    )?;
    replace_in_file(
        sh,
        "Cargo.lock",
        r#"(name = "squawk"\n)version = ".*?""#,
        &format!(r#"${{1}}version = "{v}""#),
    )?;

    let json_rep = format!(r#""version": "{v}""#);
    replace_in_file(sh, "package.json", r#""version": ".*""#, &json_rep)?;
    replace_in_file(
        sh,
        "squawk-vscode/package.json",
        r#""version": ".*""#,
        &json_rep,
    )?;

    replace_in_file(
        sh,
        "flake.nix",
        r#"(?s)(pname = "squawk";.*?)version = ".*?""#,
        &format!(r#"${{1}}version = "{v}""#),
    )?;
    replace_in_file(
        sh,
        "crates/squawk_github/src/app.rs",
        r#"const SQUAWK_USER_AGENT: &str = "squawk/.*""#,
        &format!(r#"const SQUAWK_USER_AGENT: &str = "squawk/{v}""#),
    )?;
    replace_in_file(sh, "README.md", "rev: .*", &format!("rev: {v}"))?;

    Ok(())
}

fn replace_in_file(sh: &Shell, path: &str, pattern: &str, replacement: &str) -> Result<()> {
    let full = project_root().join(path);
    let content = sh.read_file(&full)?;
    let re = Regex::new(pattern)?;
    if !re.is_match(&content) {
        bail!("pattern {pattern:?} not found in {path}");
    }
    let updated = re.replace_all(&content, replacement);
    sh.write_file(&full, updated.as_ref())?;
    Ok(())
}

fn update_changelog(sh: &Shell, version: &str, date: &str, commits: &str) -> Result<()> {
    let path = project_root().join("CHANGELOG.md");
    let content = sh.read_file(&path)?;
    let needle = "## [Unreleased]\n";
    if !content.contains(needle) {
        bail!("CHANGELOG.md is missing the '## [Unreleased]' header");
    }
    let replacement = format!("## [Unreleased]\n\n## v{version} - {date}\n\n{commits}\n");
    let updated = content.replacen(needle, &replacement, 1);
    sh.write_file(&path, updated)?;
    Ok(())
}

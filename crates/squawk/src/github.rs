use crate::config::Config;
use crate::reporter::{fmt_tty_violation, CheckReport};
use crate::Command;
use crate::{file_finding::find_paths, reporter::check_files};
use anyhow::{anyhow, bail, Result};
use console::strip_ansi_codes;
use log::info;
use squawk_github::{actions, app, comment_on_pr, GitHubApi};
use squawk_linter::{Rule, Version};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_github_private_key(
    github_private_key: Option<String>,
    github_private_key_base64: Option<String>,
) -> Result<String> {
    if let Some(private_key) = github_private_key {
        Ok(private_key)
    } else {
        let Some(key) = github_private_key_base64 else {
            bail!("Missing GitHub private key");
        };
        let bytes = base64::decode(key).map_err(|err| {
            anyhow!(
                "Failed to decode GitHub private key from base64 encoding: {}",
                err
            )
        })?;
        Ok(String::from_utf8(bytes)
            .map_err(|err| anyhow!("Could not decode GitHub private key to string: {}", err))?)
    }
}

fn create_gh_app(
    github_install_id: Option<i64>,
    github_app_id: Option<i64>,
    github_token: Option<String>,
    github_private_key: Option<String>,
    github_private_key_base64: Option<String>,
) -> Result<Box<dyn GitHubApi>> {
    if let Some(github_install_id) = github_install_id {
        if let Some(github_app_id) = github_app_id {
            info!("using github app client");
            let gh_private_key =
                get_github_private_key(github_private_key, github_private_key_base64)?;
            let app = app::GitHub::new(&gh_private_key, github_app_id, github_install_id)?;
            return Ok(Box::new(app));
        }
    }

    if let Some(github_token) = github_token {
        info!("using github actions client");
        return Ok(Box::new(actions::GitHub::new(&github_token)));
    };
    bail!(
        "Missing GitHub credentials:

        For a GitHub token:
        --github-token is required
        
        For a GitHub App:
        --github-app-id is required
        --github-install-id is required
        --github-private-key or --github-private-key-base64 is required
        "
    )
}

pub fn check_and_comment_on_pr(
    cmd: Command,
    cfg: &Config,
    is_stdin: bool,
    stdin_path: Option<String>,
    exclude: &[Rule],
    exclude_paths: &[String],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
) -> Result<()> {
    let Command::UploadToGithub {
        paths,
        fail_on_violations,
        github_private_key,
        github_token,
        github_app_id,
        github_install_id,
        github_repo_owner,
        github_repo_name,
        github_pr_number,
        github_private_key_base64,
    } = cmd;

    let fail_on_violations =
        if let Some(fail_on_violations_cfg) = cfg.upload_to_github.fail_on_violations {
            fail_on_violations_cfg
        } else {
            fail_on_violations
        };

    let github_app = create_gh_app(
        github_install_id,
        github_app_id,
        github_token,
        github_private_key,
        github_private_key_base64,
    )?;

    let found_paths = find_paths(&paths, exclude_paths)?;

    info!("checking files");
    let file_results = check_files(
        &found_paths,
        is_stdin,
        stdin_path,
        exclude,
        pg_version,
        assume_in_transaction,
    )?;

    // We should only leave a comment when there are files checked.
    if paths.is_empty() {
        info!("no files checked, exiting");
        return Ok(());
    }
    info!("generating github comment body");
    let comment_body = get_comment_body(&file_results, VERSION);

    comment_on_pr(
        github_app.as_ref(),
        &github_repo_owner,
        &github_repo_name,
        github_pr_number,
        &comment_body,
    )?;

    let violations: usize = file_results.iter().map(|f| f.violations.len()).sum();

    if fail_on_violations && violations > 0 {
        let files = file_results.len();
        bail!("Found {violations} violation(s) across {files} file(s)");
    }

    Ok(())
}

fn get_comment_body(files: &[CheckReport], version: &str) -> String {
    let violations_count: usize = files.iter().map(|x| x.violations.len()).sum();

    let violations_emoji = get_violations_emoji(violations_count);

    format!(
        r"
# Squawk Report

### **{violations_emoji} {violation_count}** violations across **{file_count}** file(s)

---
{sql_file_content}

[📚 More info on rules](https://github.com/sbdchd/squawk#rules)

⚡️ Powered by [`Squawk`](https://github.com/sbdchd/squawk) ({version}), a linter for PostgreSQL, focused on migrations
",
        violations_emoji = violations_emoji,
        violation_count = violations_count,
        file_count = files.len(),
        sql_file_content = files
            .iter()
            .filter_map(|x| get_sql_file_content(x).ok())
            .collect::<Vec<String>>()
            .join("\n"),
        version = version
    )
    .trim_matches('\n')
    .into()
}

const fn get_violations_emoji(count: usize) -> &'static str {
    if count > 0 {
        "🚒"
    } else {
        "✅"
    }
}

fn get_sql_file_content(violation: &CheckReport) -> Result<String> {
    let sql = &violation.sql;
    let mut buff = Vec::new();
    let violation_count = violation.violations.len();
    for v in &violation.violations {
        fmt_tty_violation(&mut buff, v, &violation.filename, sql)?;
    }
    let violations_text_raw = &String::from_utf8_lossy(&buff);
    let violations_text = strip_ansi_codes(violations_text_raw);

    let violation_content = if violation_count > 0 {
        format!(
            r"
```
{}
```",
            violations_text.trim_matches('\n')
        )
    } else {
        "No violations found.".to_string()
    };

    let violations_emoji = get_violations_emoji(violation_count);

    Ok(format!(
        r"
<h3><code>{filename}</code></h3>

```sql
{sql}
```

<h4>{violations_emoji} Rule Violations ({violation_count})</h4>

{violation_content}
    
---
    ",
        violations_emoji = violations_emoji,
        filename = violation.filename,
        sql = sql,
        violation_count = violation_count,
        violation_content = violation_content
    ))
}

#[cfg(test)]
mod test_github_comment {
    use crate::{
        github::get_comment_body,
        reporter::{CheckReport, ReportViolation, ViolationLevel},
    };

    use insta::assert_snapshot;
    use line_index::{TextRange, TextSize};

    /// Most cases, hopefully, will be a single migration for a given PR, but
    /// let's check the case of multiple migrations
    #[test]
    fn generating_comment_multiple_files() {
        let violations = vec![CheckReport {
            filename: "alpha.sql".into(),
            sql: r"
SELECT 1;
                "
            .into(),
            violations: vec![ReportViolation {
                file: "alpha.sql".into(),
                line: 1,
                column: 0,
                level: ViolationLevel::Warning,
                rule_name: "adding-not-nullable-field".to_string(),
                range: TextRange::new(TextSize::new(0), TextSize::new(0)),
                message: "Adding a NOT NULL field requires exclusive locks and table rewrites."
                    .to_string(),
                help: Some("Make the field nullable.".to_string()),
            }],
        }];

        let body = get_comment_body(&violations, "0.2.3");

        assert_snapshot!(body);
    }

    /// Even when we don't have violations we still want to output the SQL for
    /// easy human reading.
    #[test]
    fn generating_comment_no_violations() {
        let violations = vec![
            CheckReport {
                filename: "alpha.sql".into(),
                sql: r#"
BEGIN;
--
-- Create model Bar
--
CREATE TABLE "core_bar" (
    "id" serial NOT NULL PRIMARY KEY,
    "alpha" varchar(100) NOT NULL
);
                "#
                .into(),
                violations: vec![],
            },
            CheckReport {
                filename: "bravo.sql".into(),
                sql: r#"
ALTER TABLE "core_recipe" ADD COLUMN "foo" integer DEFAULT 10;
                "#
                .into(),
                violations: vec![],
            },
        ];

        let body = get_comment_body(&violations, "0.2.3");

        assert_snapshot!(body);
    }

    /// Ideally the logic won't leave a comment when there are no migrations but
    /// better safe than sorry
    #[test]
    fn generating_no_violations_no_files() {
        let violations = vec![];

        let body = get_comment_body(&violations, "0.2.3");

        assert_snapshot!(body);
    }
}

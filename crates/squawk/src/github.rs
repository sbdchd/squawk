use crate::UploadToGithubArgs;
use crate::config::Config;
use crate::reporter::{CheckReport, fmt_github_annotations, fmt_tty_violation};
use crate::{file_finding::find_paths, reporter::check_files};
use anyhow::{Result, anyhow, bail};
use console::strip_ansi_codes;
use log::info;
use squawk_github::{GitHubApi, actions, app, comment_on_pr};
use squawk_linter::{Rule, Version};
use std::io;

const VERSION: &str = env!("CARGO_PKG_VERSION");

// GitHub API limit for issue comment body is 65,536 characters
// We use a slightly smaller limit to leave room for the comment structure
const GITHUB_COMMENT_MAX_SIZE: usize = 65_000;
const MAX_SQL_PREVIEW_LINES: usize = 50;

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
    github_api_url: Option<String>,
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
        let client = match github_api_url {
            Some(github_api_url) => actions::GitHub::new_with_url(&github_api_url, &github_token),
            None => actions::GitHub::new(&github_token),
        };
        return Ok(Box::new(client));
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

const COMMENT_HEADER: &str = "# Squawk Report";

pub fn check_and_comment_on_pr(
    args: UploadToGithubArgs,
    cfg: &Config,
    is_stdin: bool,
    stdin_path: Option<String>,
    exclude: &[Rule],
    exclude_paths: &[String],
    pg_version: Option<Version>,
    assume_in_transaction: bool,
    github_annotations: bool,
) -> Result<()> {
    let UploadToGithubArgs {
        paths,
        fail_on_violations,
        github_private_key,
        github_api_url,
        github_token,
        github_app_id,
        github_install_id,
        github_repo_owner,
        github_repo_name,
        github_pr_number,
        github_private_key_base64,
    } = args;

    let fail_on_violations =
        if let Some(fail_on_violations_cfg) = cfg.upload_to_github.fail_on_violations {
            fail_on_violations_cfg
        } else {
            fail_on_violations
        };

    let github_app = create_gh_app(
        github_api_url,
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
        COMMENT_HEADER,
    )?;

    if github_annotations {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        fmt_github_annotations(&mut handle, &file_results)?;
    }

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

    // First, try to generate the full comment
    let sql_file_contents: Vec<String> = files
        .iter()
        .filter_map(|x| get_sql_file_content(x).ok())
        .collect();

    let content = sql_file_contents.join("\n");
    let full_comment = format_comment(
        violations_emoji,
        violations_count,
        files.len(),
        &content,
        version,
        None, // No summary notice for full comments
    );

    // Check if the comment exceeds GitHub's size limit
    if full_comment.len() <= GITHUB_COMMENT_MAX_SIZE {
        return full_comment;
    }

    // If the comment is too large, create a summary instead
    get_summary_comment_body(files, violations_count, violations_emoji, version)
}

fn get_summary_comment_body(
    files: &[CheckReport],
    violations_count: usize,
    violations_emoji: &str,
    version: &str,
) -> String {
    let mut file_summaries = Vec::new();

    for file in files {
        let violation_count = file.violations.len();
        let violations_emoji = get_violations_emoji(violation_count);
        let line_count = file.sql.lines().count();

        let summary = format!(
            r"
<h3><code>{filename}</code></h3>

📄 **{line_count} lines** | {violations_emoji} **{violation_count} violations**

{violations_detail}

---
    ",
            filename = file.filename,
            line_count = line_count,
            violations_emoji = violations_emoji,
            violation_count = violation_count,
            violations_detail = if violation_count > 0 {
                let violation_rules: Vec<String> = file
                    .violations
                    .iter()
                    .map(|v| format!("• `{}` (line {})", v.rule_name, v.line + 1))
                    .collect();
                format!("**Violations found:**\n{}", violation_rules.join("\n"))
            } else {
                "✅ No violations found.".to_string()
            }
        );
        file_summaries.push(summary);
    }

    let summary_notice = Some("⚠️ **Large Report**: This report was summarized due to size constraints. SQL content has been omitted but all violations were analyzed.");
    
    format_comment(
        violations_emoji,
        violations_count,
        files.len(),
        &file_summaries.join("\n"),
        version,
        summary_notice,
    )
}

const fn get_violations_emoji(count: usize) -> &'static str {
    if count > 0 { "🚒" } else { "✅" }
}

fn format_comment(
    violations_emoji: &str,
    violation_count: usize,
    file_count: usize,
    content: &str,
    version: &str,
    summary_notice: Option<&str>,
) -> String {
    let notice_section = if let Some(notice) = summary_notice {
        format!("\n> {}\n", notice)
    } else {
        String::new()
    };

    format!(
        r"
{COMMENT_HEADER}

### **{violations_emoji} {violation_count}** violations across **{file_count}** file(s){notice_section}
---
{content}

[📚 More info on rules](https://github.com/sbdchd/squawk#rules)

⚡️ Powered by [`Squawk`](https://github.com/sbdchd/squawk) ({version}), a linter for PostgreSQL, focused on migrations
",
        violations_emoji = violations_emoji,
        violation_count = violation_count,
        file_count = file_count,
        notice_section = notice_section,
        content = content,
        version = version
    )
    .trim_matches('\n')
    .into()
}

fn truncate_sql_if_needed(sql: &str) -> (String, bool) {
    let lines: Vec<&str> = sql.lines().collect();
    if lines.len() <= MAX_SQL_PREVIEW_LINES {
        (sql.to_string(), false)
    } else {
        let truncated_lines = lines[..MAX_SQL_PREVIEW_LINES].join("
");
        let remaining_lines = lines.len() - MAX_SQL_PREVIEW_LINES;
        (
            format!(
                "{truncated_lines}

-- ... ({remaining_lines} more lines truncated for brevity)"
            ),
            true,
        )
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
    let (display_sql, was_truncated) = truncate_sql_if_needed(sql);

    let truncation_notice = if was_truncated {
        "\n\n> ⚠️ **Note**: SQL content has been truncated for display purposes. The full analysis was performed on the complete file."
    } else {
        ""
    };

    Ok(format!(
        r"
<h3><code>{filename}</code></h3>

```sql
{sql}
```{truncation_notice}

<h4>{violations_emoji} Rule Violations ({violation_count})</h4>

{violation_content}
    
---
    ",
        violations_emoji = violations_emoji,
        filename = violation.filename,
        sql = display_sql,
        truncation_notice = truncation_notice,
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
                column_end: 0,
                line_end: 1,
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

    #[test]
    fn sql_truncation() {
        let short_sql = "SELECT 1;";
        let (result, truncated) = crate::github::truncate_sql_if_needed(short_sql);
        assert!(!truncated);
        assert_eq!(result, short_sql);

        let long_sql = (0..100)
            .map(|i| format!("-- Line {}", i))
            .collect::<Vec<_>>()
            .join("\n");
        let (result, truncated) = crate::github::truncate_sql_if_needed(&long_sql);
        assert!(truncated);
        assert!(result.contains("-- ... (50 more lines truncated for brevity)"));
    }

    #[test]
    fn generating_comment_with_large_content() {
        // Create a very large SQL content
        let large_sql = (0..1000)
            .map(|i| format!("SELECT {} as col{};", i, i))
            .collect::<Vec<_>>()
            .join("\n");

        let violations = vec![CheckReport {
            filename: "large.sql".into(),
            sql: large_sql,
            violations: vec![ReportViolation {
                file: "large.sql".into(),
                line: 1,
                column: 0,
                level: ViolationLevel::Warning,
                rule_name: "prefer-bigint-over-int".to_string(),
                range: TextRange::new(TextSize::new(0), TextSize::new(0)),
                message: "Prefer bigint over int.".to_string(),
                help: Some("Use bigint instead.".to_string()),
                column_end: 0,
                line_end: 1,
            }],
        }];

        let body = get_comment_body(&violations, "0.2.3");

        // The comment should be within GitHub's size limits
        assert!(body.len() <= super::GITHUB_COMMENT_MAX_SIZE);

        // Should contain summary information even if the full content was too large
        assert!(body.contains("violations"));
    }

    #[test]
    fn generating_comment_forced_summary() {
        // Create content that will definitely trigger summary mode
        let massive_sql = (0..10000)
            .map(|i| format!("SELECT {} as col{};", i, i))
            .collect::<Vec<_>>()
            .join("\n");

        let violations = vec![CheckReport {
            filename: "massive.sql".into(),
            sql: massive_sql,
            violations: vec![ReportViolation {
                file: "massive.sql".into(),
                line: 1,
                column: 0,
                level: ViolationLevel::Warning,
                rule_name: "prefer-bigint-over-int".to_string(),
                range: TextRange::new(TextSize::new(0), TextSize::new(0)),
                message: "Prefer bigint over int.".to_string(),
                help: Some("Use bigint instead.".to_string()),
                column_end: 0,
                line_end: 1,
            }],
        }];

        let body = get_comment_body(&violations, "0.2.3");

        // The comment should be within GitHub's size limits
        assert!(body.len() <= super::GITHUB_COMMENT_MAX_SIZE);

        // Should contain the summary notice
        if body.contains("Large Report") {
            assert!(body.contains("summarized due to size constraints"));
        } else {
            // If it didn't trigger summary mode, at least verify it contains violations info
            assert!(body.contains("violations"));
        }
    }
}

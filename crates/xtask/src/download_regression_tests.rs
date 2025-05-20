use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use std::fs::{create_dir_all, remove_dir_all, File};
use std::io::{BufRead, Cursor, Write};
use std::process::Command;

pub(crate) fn download_regression_tests() -> Result<()> {
    let target_dir = Utf8PathBuf::from("crates/squawk_parser/tests/data/regression_suite");

    if target_dir.exists() {
        println!("Cleaning target directory: {:?}", target_dir);
        remove_dir_all(&target_dir)?;
    }

    create_dir_all(&target_dir)?;

    let urls = fetch_download_urls()?;
    let total_files = urls.len();

    for (index, url) in urls.iter().enumerate() {
        let filename = url.split('/').last().unwrap();
        let filepath = target_dir.join(filename);

        println!(
            "[{}/{}] Downloading {}... ",
            index + 1,
            total_files,
            filename
        );

        let output = Command::new("curl").args(["-s", url]).output()?;

        if !output.status.success() {
            let error_msg = String::from_utf8_lossy(&output.stderr);
            bail!(anyhow::anyhow!(
                "Failed to download '{}': {}",
                url,
                error_msg
            ));
        }

        let mut processed_content = Vec::new();

        let cursor = Cursor::new(&output.stdout);

        if let Err(e) = preprocess_sql(cursor, &mut processed_content) {
            eprintln!("Error: Failed to process file: {}", e);
            continue;
        }

        let mut dest = File::create(&filepath)?;
        dest.write_all(&processed_content)?
    }

    Ok(())
}

fn fetch_download_urls() -> Result<Vec<String>> {
    // Fetch list of SQL file URLs
    println!("Fetching SQL file URLs...");
    let output = Command::new("gh")
        .args([
            "api",
            "-H",
            "Accept: application/vnd.github+json",
            "/repos/postgres/postgres/contents/src/test/regress/sql",
        ])
        .output()?;

    if !output.status.success() {
        bail!(anyhow::anyhow!(
            "Failed to fetch SQL files: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let json_str = String::from_utf8(output.stdout)?;
    let files: Vec<serde_json::Value> = serde_json::from_str(&json_str)?;

    // Extract download URLs for SQL files
    let urls: Vec<String> = files
        .into_iter()
        .filter(|file| {
            file["name"]
                .as_str()
                .map(|name| name.ends_with(".sql"))
                .unwrap_or(false)
        })
        .filter_map(|file| file["download_url"].as_str().map(String::from))
        .collect();

    if urls.is_empty() {
        bail!(anyhow::anyhow!("No SQL files found"));
    }

    Ok(urls)
}

fn preprocess_sql<R: BufRead, W: Write>(source: R, mut dest: W) -> Result<()> {
    let mut skipping_copy_block = false;

    for line in source.lines() {
        let mut line = line?;

        // Detect the start of the COPY block
        if line.starts_with("COPY ") && line.to_lowercase().contains("from stdin") {
            skipping_copy_block = true;
            continue;
        }

        // Detect the end of the COPY block
        if skipping_copy_block && (line.starts_with("\\.") || line.is_empty()) {
            skipping_copy_block = false;
            continue;
        }

        // Skip lines if inside a COPY block
        if skipping_copy_block {
            continue;
        }

        if line.starts_with("\\") {
            // Skip plpgsql commands (for now)
            continue;
        }

        // replace "\gset" with ";"
        if line.contains("\\gset") {
            line = line.replace("\\gset", ";");
        }

        // Write the cleaned line
        writeln!(dest, "{}", line)?;
    }

    Ok(())
}

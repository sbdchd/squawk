use anyhow::{Context, Result};
use reqwest::header::USER_AGENT;
use serde::Deserialize;

pub(crate) fn sync_kwlist() -> Result<()> {
    latest_pg_git_sha()?;
    Ok(())
}

use std::fs;
use std::io::Write;

use crate::path_util::cwd_to_workspace_root;

#[derive(Deserialize, Debug)]
struct CommitResponse {
    sha: String,
    commit: Commit,
}

#[derive(Deserialize, Debug)]
struct Commit {
    committer: Commiter,
}

#[derive(Deserialize, Debug)]
struct Commiter {
    date: String,
}

fn latest_pg_git_sha() -> Result<()> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/repos/postgres/postgres/commits/master")
        .header(USER_AGENT, "squawk xtask sync-kwlist")
        .send()?;

    let res: CommitResponse = response.json()?;

    let file_response = client
        .get(format!(
            "https://raw.githubusercontent.com/postgres/postgres/{}/src/include/parser/kwlist.h",
            res.sha
        ))
        .send()?;

    let file_content = file_response.text()?;

    cwd_to_workspace_root().context("Failed to cwd to root")?;

    let preamble = format!(
        r"// synced from: 
//   commit: {}
//   committed at: {}
//   file: https://github.com/postgres/postgres/blob/{}/src/include/parser/kwlist.h
//
// update via:
//   cargo xtask sync-kwlist

",
        res.sha, res.commit.committer.date, res.sha
    )
    .to_owned();

    let mut file = fs::File::create("postgres/kwlist.h")?;
    file.write_all((preamble + &file_content).as_bytes())?;

    Ok(())
}

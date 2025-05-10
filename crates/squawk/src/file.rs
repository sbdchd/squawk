use std::{
    fs::File,
    io::{self, Read},
    path::PathBuf,
};

use anyhow::Result;

pub(crate) fn sql_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub(crate) fn sql_from_path(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::{env, fs, io, vec};

use anyhow::{Result, anyhow};

pub fn whoami() -> String {
    let output: Output = Command::new("whoami")
        .output()
        .expect("failed to execute process");

    let username: String = String::from_utf8(output.stdout).unwrap();
    username
}

pub fn fs_present(path: &Path) -> Result<bool> {
    if let Err(err) = fs::metadata(path) {
        if let io::ErrorKind::NotFound = err.kind() {
            return Ok(true);
        }

        return Err(anyhow!("Failed to get file metadata: {err}"));
    }

    Ok(true)
}

pub fn create_dir(path: &Path) -> Result<()> {
    if let Err(err) = fs::create_dir(path) {
        return Err(anyhow!("Failed to create directory: {err}"));
    }

    Ok(())
}

pub fn get_entry_name(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().to_string()
}

pub fn option_to_vec<T>(opt: Option<T>) -> Vec<T> {
    let mut v = Vec::new();

    if let Some(t) = opt {
        v.push(t);
    }

    v
}

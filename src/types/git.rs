use std::{error::Error, process::Command};

#[derive(Debug)]
pub struct GitDiff {
    files: Vec<String>,
    diff: String,
}

impl GitDiff {
    pub fn new() -> Result<GitDiff, Box<dyn Error>> {
        let output = Command::new("git")
            .args(["diff", "--cached", "--name-only"])
            .output()?;

        if !output.status.success() {
            return Err("Failed to execute git diff --cached --name-only".into());
        }
        if output.stdout.is_empty() {
            return Err("No files staged for commit".into());
        }

        let files = String::from_utf8(output.stdout)?
            .lines()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        println!("Files staged for commit: {files:#?}");

        let output = Command::new("git").args(["diff", "--cached"]).output()?;

        if !output.status.success() {
            return Err("Failed to execute git diff --cached".into());
        }

        let diff = String::from_utf8(output.stdout)?;

        Ok(GitDiff { files, diff })
    }

    #[must_use]
    pub fn get_files(&self) -> &Vec<String> {
        &self.files
    }

    #[must_use]
    pub fn get_diff(&self) -> &String {
        &self.diff
    }
}

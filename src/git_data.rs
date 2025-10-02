use std::{
    path::PathBuf,
    process::Command,
    sync::mpsc::{self, Sender},
};

use color_eyre::eyre::{Result, eyre};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug)]
pub struct GitData {
    pub status: String,
    pub diff: String,
}

enum GitCmd {
    Status,
    Diff,
}

fn execute_git_command(r#type: GitCmd, repo_path: &PathBuf) -> Result<String> {
    let subcommand = match r#type {
        GitCmd::Status => "status",
        GitCmd::Diff => "diff",
    };

    let output = Command::new("git")
        .arg(subcommand)
        .current_dir(repo_path)
        .output()
        .map_err(|e| eyre!(format!("git {subcommand} could not be executed: {e:#?}")))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

pub fn collect_git_data(
    fuzzit_path: Option<PathBuf>,
    fuzzit_base_path: Option<PathBuf>,
) -> Result<(PathBuf, Vec<(String, GitData)>)> {
    let mut repo_paths = Vec::new();
    let (tx, rx) = mpsc::channel();

    let base_path = if let Some(fuzzit_path) = fuzzit_path {
        fuzzit_path
    } else if let Some(fuzzit_base_path) = fuzzit_base_path {
        fuzzit_base_path
    } else {
        Err(eyre!(
            "Add FUZZIT_BASE_PATH to your environment (ex: ~/.zshrc) or use FUZZIT_PATH before command"
        ))?
    };

    recursive_repo_search(&base_path, tx)?;

    while let Ok(repo_path) = rx.recv() {
        repo_paths.push(repo_path)
    }

    // Iteratite through collected repos with parallelism
    let git_data = repo_paths
        .par_iter()
        .map(|repo_path| {
            // Concurrently get git data
            let (status, diff) = rayon::join(
                || execute_git_command(GitCmd::Status, repo_path).unwrap_or_default(),
                || execute_git_command(GitCmd::Diff, repo_path).unwrap_or_default(),
            );

            (repo_path.display().to_string(), GitData { status, diff })
        })
        .collect();

    Ok((base_path, git_data))
}

fn recursive_repo_search(current_path: &PathBuf, repo_path_sender: Sender<PathBuf>) -> Result<()> {
    let mut git_path = current_path.clone();
    git_path.push(".git");

    if git_path.exists() {
        git_path.pop();
        repo_path_sender.send(git_path)?;
        return Ok(()); // Don't recurse into repos
    }

    let subfolders = std::fs::read_dir(current_path)?;
    for subfolder in subfolders {
        let subfolder_path = subfolder?.path();

        if subfolder_path.is_dir()
            && let Some(subfolder_name) = subfolder_path.file_name()
        {
            let subfolder_name_str = subfolder_name.to_str().unwrap_or_default();

            // Skip hidden directories except .git and common build/dependency directories
            if !(subfolder_name_str.starts_with('.') && subfolder_name_str != ".git")
                || !(subfolder_name_str == "node_modules"
                    || subfolder_name_str == "target"
                    || subfolder_name_str == "dist"
                    || subfolder_name_str == "build")
            {
                // Only reason I don't want to use parallelism/concurrency here is to maintain a
                // natural/consistent a-z name sorting coming from .read_dir() without having to
                // spend resources to actually sort after collecting (barely decreases time for my
                // use case anyways)
                recursive_repo_search(&subfolder_path, repo_path_sender.clone())?;
            }
        }
    }

    Ok(())
}

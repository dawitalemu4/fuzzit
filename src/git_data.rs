use std::{
    path::PathBuf,
    process::Command,
    sync::mpsc::{self, Sender},
};

use color_eyre::eyre::{Result, eyre};
use rayon::{
    iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator},
    slice::ParallelSliceMut,
};

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
        GitCmd::Status => vec!["status"],
        GitCmd::Diff => vec!["diff", "@{upstream}"], // Compare against remote of current branch
    };

    let output = Command::new("git")
        .args(&subcommand)
        .current_dir(repo_path)
        .output()
        .map_err(|e| eyre!(format!("git {subcommand:#?} could not be executed: {e:#?}")))?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn collect_git_data(
    fuzzit_path: Option<PathBuf>,
    fuzzit_base_path: Option<PathBuf>,
) -> Result<(String, Vec<(String, GitData)>)> {
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

    let parsed_base_path = if base_path.starts_with("~") {
        if let Some(mut home) = dirs::home_dir() {
            let mut components = base_path.components();
            components.next(); // Skip the tilde component to use home dir instead

            home.extend(components);
            home
        } else {
            Err(eyre!(
                "Home directory could not be determined, please use full path for FUZZIT_BASE_PATH or FUZZIT_PATH"
            ))?
        }
    } else {
        base_path.clone()
    };

    recursive_repo_search(&parsed_base_path, tx)?;
    while let Ok(repo_path) = rx.recv() {
        repo_paths.push(repo_path)
    }

    // Parallel sort repo paths a-z (unstable is faster)
    repo_paths.par_sort_unstable();

    // Parallel iterate through collected repos
    let git_data = repo_paths
        .par_iter()
        .map(|repo_path| {
            let stripped_repo_path = {
                let removed_base_path = repo_path
                    .display()
                    .to_string()
                    .replace(&parsed_base_path.display().to_string(), "");

                if cfg!(windows) {
                    removed_base_path.replacen(r#"\"#, "", 1)
                } else {
                    removed_base_path.replacen("/", "", 1)
                }
            };

            // Concurrently get git data
            let (status, diff) = rayon::join(
                || execute_git_command(GitCmd::Status, repo_path).unwrap_or_default(),
                || execute_git_command(GitCmd::Diff, repo_path).unwrap_or_default(),
            );

            (stripped_repo_path, GitData { status, diff })
        })
        .collect();

    Ok((base_path.display().to_string(), git_data))
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
    subfolders
        .par_bridge() // Parallel iterate through subfolders
        .map(|subfolder_res| {
            if let Ok(subfolder) = subfolder_res {
                let subfolder_path = subfolder.path();

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
                        recursive_repo_search(&subfolder_path, repo_path_sender.clone())
                            .unwrap_or_default();
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    Ok(())
}

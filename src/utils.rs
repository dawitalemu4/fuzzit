use std::{collections::HashMap, path::PathBuf, process::Command, sync::Arc};

use color_eyre::eyre::{Result, eyre};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tokio::sync::{Semaphore, mpsc};

enum GitCmd {
    Status,
    Diff,
}

#[derive(Debug)]
pub struct GitData {
    pub status: String,
    pub diff: String,
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

pub async fn collect_git_data(
    fuzzit_path: Option<PathBuf>,
    fuzzit_base_path: Option<PathBuf>,
) -> Result<(PathBuf, HashMap<PathBuf, GitData>)> {
    let mut repo_paths = Vec::new();
    let (tx, mut rx) = mpsc::unbounded_channel();
    let semaphore = Arc::new(Semaphore::new(Semaphore::MAX_PERMITS)); // Max concurrent tasks

    let mut base_path = if let Some(fuzzit_path) = fuzzit_path {
        fuzzit_path
    } else if let Some(fuzzit_base_path) = fuzzit_base_path {
        fuzzit_base_path
    } else {
        Err(eyre!(
            "Add FUZZIT_BASE_PATH to your environment (ex: ~/.zshrc) or use FUZZIT_PATH before command"
        ))?
    };

    recursive_repo_search(&mut base_path, tx, semaphore).await?;

    while let Some(repo_path) = rx.recv().await {
        repo_paths.push(repo_path)
    }

    // Iteratite through collected repos with parallelism
    let git_data: HashMap<PathBuf, GitData> = repo_paths
        .par_iter()
        .map(|repo| {
            // Concurrently get git data
            let (status, diff) = rayon::join(
                || execute_git_command(GitCmd::Status, repo).unwrap_or_default(),
                || execute_git_command(GitCmd::Diff, repo).unwrap_or_default(),
            );

            (repo.clone(), GitData { status, diff })
        })
        .collect();

    Ok((base_path, git_data))
}

// Get non repo path and spawn thread to parallel read dir while making sure you dont go over
// semaphore limit, then recurse if child of non repo path has another non repo child path
async fn recursive_repo_search(
    current_path: &mut PathBuf,
    repo_path_sender: mpsc::UnboundedSender<PathBuf>,
    semaphore: Arc<Semaphore>,
) -> Result<()> {
    let _permit = semaphore.clone().acquire_owned().await?; // Drops automatically
    let mut git_path = current_path.clone();
    git_path.push(".git");

    if git_path.exists() {
        git_path.pop();
        let _ = repo_path_sender.send(git_path);
        // return Ok(()); // Don't recurse into repos
    }

    let mut sub_folders = tokio::fs::read_dir(current_path).await?;
    while let Some(entry) = sub_folders.next_entry().await? {
        let mut path = entry.path();

        if path.is_dir()
            && let Some(folder_name) = path.file_name()
        {
            let folder_name_str = folder_name.to_string_lossy();

            // Skip hidden directories except .git and common build/dependency directories
            if (folder_name_str.starts_with('.') && folder_name_str != ".git")
                || (folder_name_str == "node_modules"
                    || folder_name_str == "target"
                    || folder_name_str == "dist"
                    || folder_name_str == "build")
            {
                continue;
            }

            // Recursively search this subdirectory
            let _ = Box::pin(recursive_repo_search(
                &mut path,
                repo_path_sender.clone(),
                semaphore.clone(),
            ))
            .await;
        }
    }

    Ok(())
}

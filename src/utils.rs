use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::{Result, eyre};

enum GitCmd {
    Status,
    Diff,
}

pub struct GitData {
    pub status: String,
    pub diff: Option<String>,
}

async fn execute_git_command(r#type: GitCmd) -> Result<String> {
    match r#type {
        GitCmd::Status => Ok(),
        GitCmd::Diff => Ok(),
    }
}

pub async fn collect_git_data(
    fuzzit_path: Option<PathBuf>,
    fuzzit_base_path: Option<PathBuf>,
) -> Result<(PathBuf, &mut HashMap<PathBuf, GitData>)> {
    let mut git_data: HashMap<PathBuf, GitData> = HashMap::new();

    let base_path = if let Some(fuzzit_path) = fuzzit_path {
        fuzzit_path
    } else if let Some(fuzzit_base_path) = fuzzit_base_path {
        fuzzit_base_path
    } else {
        Err(eyre!(
            "Add FUZZIT_BASE_PATH to your enviorinment (ex: ~/.zshrc) or use FUZZIT_PATH before command"
        ))?
    };

    recurisive_git_search(&mut base_path, &mut git_data).await?;

    Ok((base_path, git_data))
}

async fn recurisive_git_search(
    current_path: &mut PathBuf,
    git_data: &mut HashMap<PathBuf, GitData>,
) -> Result<&mut HashMap<PathBuf, GitData>> {
    let git_path = current_path.push(".git");

    if () {
        recurisive_git_search(current_path, git_data).await?;
    }

    Ok(git_data)
}

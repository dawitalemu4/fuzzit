use std::{collections::HashMap, path::PathBuf};

use color_eyre::{
    eyre::{Result, eyre},
    owo_colors::OwoColorize,
};

use crate::utils::GitData;

pub async fn display(base_path: PathBuf, git_data: HashMap<PathBuf, GitData>) -> Result<()> {
    if let Some(base_path) = base_path.to_str() {
        println!("Iterating git repos from {base_path}\n");
    }

    for (path, git_data) in git_data {
        if let Some(repo_path) = path.to_str() {
            if git_data.status.contains("nothing to commit") {
                println!("{repo_path} .. {}", "CLEAN".green().italic());
            } else if git_data.status.contains("no changes added to commit") {
                println!(
                    "{repo_path} .. {} (changes not added)",
                    "DIRTY".fg_rgb::<255, 184, 108>() // orange
                );
            } else if git_data.status.contains("Changes to be committed") {
                println!(
                    "{repo_path} .. {} (changes added, not committed)",
                    "DIRTY".red()
                );
            } else if git_data.status.contains("Your branch is ahead of") {
                println!(
                    "{repo_path} .. {} (changes committed, not pushed)",
                    "DIRTY".red().bold()
                );
            } else {
                println!("{repo_path} .. {}", "UNKNOWN".yellow());
            }
        } else {
            Err(eyre!(
                "Make sure to add FUZZIT_BASE_PATH to your environment (ex: ~/.zshrc) or use FUZZIT_PATH before command"
            ))?
        }
    }

    Ok(())
}

use color_eyre::{eyre::Result, owo_colors::OwoColorize};

use crate::git_data::GitData;

pub fn display(
    base_path: String,
    disable_ascii_art: bool,
    git_data: Vec<(String, GitData)>,
) -> Result<()> {
    if !disable_ascii_art {
        let ascii_art = r#"
 ________ ___  ___  ________  ________  ___  _________   
|\  _____\\  \|\  \|\_____  \|\_____  \|\  \|\___   ___\ 
\ \  \__/\ \  \\\  \\|___/  /|\|___/  /\ \  \|___ \  \_| 
 \ \   __\\ \  \\\  \   /  / /    /  / /\ \  \   \ \  \  
  \ \  \_| \ \  \\\  \ /  /_/__  /  /_/__\ \  \   \ \  \ 
   \ \__\   \ \_______\\________\\________\ \__\   \ \__\
    \|__|    \|_______|\|_______|\|_______|\|__|    \|__|
"#;

        println!("{ascii_art}");
    }

    if git_data.is_empty() {
        println!("Could not find any git repos from provided path");
        println!(
            "Make sure to add FUZZIT_BASE_PATH to your environment (ex: ~/.zshrc) or use FUZZIT_PATH before command"
        );

        return Ok(());
    }

    println!("Iterating git repos from {base_path}\n");
    for (repo_path, git_data) in git_data {
        if git_data.status.contains("nothing to commit")
            && !git_data.status.contains("branch is ahead")
        {
            if git_data.diff.is_empty() {
                println!("{repo_path} .. {}", "CLEAN".green().italic());
            } else {
                // edge case: no upstream branch, but there are local commits not pushed
                println!(
                    "{repo_path} .. {}",
                    "DIRTY (changes committed, not pushed)".red().bold()
                );
            }
        } else if git_data.status.contains("no changes added to commit")
            || git_data.status.contains("untracked")
        {
            println!(
                "{repo_path} .. {}",
                "DIRTY (changes not added)".fg_rgb::<255, 184, 108>() // orange
            );
        } else if git_data.status.contains("Changes to be committed") {
            println!(
                "{repo_path} .. {}",
                "DIRTY (changes added, not committed)".red()
            );
        } else if git_data.status.contains("branch is ahead")
            || git_data.status.contains("diverged")
        {
            println!(
                "{repo_path} .. {}",
                "DIRTY (changes committed, not pushed)".red().bold()
            );
        } else {
            println!("{repo_path} .. {}", "UNKNOWN".yellow());
        }
    }

    Ok(())
}

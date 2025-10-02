use std::path::PathBuf;

use clap::Parser;
use color_eyre::eyre::Result;

pub mod diff;
pub mod git_data;
pub mod status;

use git_data::collect_git_data;

#[derive(Parser, Debug)]
#[command(
    version,
    name = "fuzzit",
    about = "Fuzzy nested git repo finder with status and diff previews"
)]
struct Args {
    /// Simple list of one-line git status summaries (diff TUI by default)
    #[arg(short, long, default_value = "false")]
    status: bool,
    /// Disable ascii art from displaying (false by default)
    #[arg(short, long, default_value = "false")]
    disable_ascii: bool,
    /// Path to start searching from, takes priority over FUZZIT_BASE_PATH
    #[arg(env)]
    fuzzit_path: Option<PathBuf>,
    /// Your default base path to start searching from, please add to your environment (ex: ~/.zshrc)
    #[arg(env)]
    fuzzit_base_path: Option<PathBuf>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let (base_path, git_data) = collect_git_data(args.fuzzit_path, args.fuzzit_base_path)?;

    if args.status {
        status::display(base_path, args.disable_ascii, git_data)?;
    }
    // } else {
    //     let mut terminal = ratatui::init();
    //     let res = diff::app::new(base_path, git_data).run(&mut terminal);
    //     ratatui::restore();
    //     res?
    // }

    Ok(())
}

use std::path::PathBuf;

use clap::{Command::subcommand, Parser, arg, command};
use color_eyre::eyre::Result;
use utils::collect_git_data;

pub mod diff;
pub mod status;
pub mod utils;

#[derive(Parser, Debug)]
#[command(
    version,
    name = "fuzzit",
    about = "Fuzzy nested git repo finder with status and diff previews"
)]
struct Args {
    /// Simple list of one-line git status summaries
    #[command(subcommand, default_value = "false")]
    status: bool,
    #[arg(env)]
    fuzzit_path: Option<PathBuf>,
    #[arg(env)]
    fuzzit_base_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();

    let (base_path, git_data) = collect_git_data(args.fuzzit_path, args.fuzzit_base_path).await?;

    if args.status {
        status::display(base_path, git_data).await?;
    } else {
        let terminal = ratatui::init();
        diff::App::new(base_path, git_data).run(terminal);
    }

    Ok(())
}

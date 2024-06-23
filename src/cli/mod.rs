use clap::Parser;

mod commands;
use commands::Commands;

#[derive(Parser, Debug)]
#[command(author = "0x56696B")]
#[command(about = "A git extension to make `git worktree` easier to use", long_about = None)]
#[command(version)]
#[command(next_line_help = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}

impl Cli {
  pub fn new() -> Self {
    Cli::parse()
  }
}

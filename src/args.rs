use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "file_organizer", author = "vcnt")]
pub struct Args {
    /// The directory to organize
    #[arg(short, long)]
    pub directory: Option<PathBuf>,

    /// optional log file to restore the state from
    #[arg(short, long)]
    pub restore: Option<PathBuf>,
}

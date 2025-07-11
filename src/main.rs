use clap::Parser;

mod args;
mod log;
mod organizer;
mod restore;

use args::Args;
use log::{Log, write_log};
use organizer::{organize_files, remove_empty_directories};
use restore::restore_state;

fn main() {
    let args = Args::parse();

    // validate args
    match !args.restore.is_none() {
        true => {
            let log_file = args.restore.unwrap();
            // restore file must exist
            if !log_file.exists() {
                eprintln!("Log file does not exist: {}", log_file.display());
                return;
            }
            let abs_log_file = log_file.canonicalize().unwrap();
            restore_state(&abs_log_file);
        }
        false => {
            // directory must exist and be a directory
            if args.directory.is_none() {
                eprintln!("No directory specified. Use --directory to specify a directory.");
                return;
            }

            let args_directory = args.directory.unwrap();

            if !args_directory.exists() || !args_directory.is_dir() {
                eprintln!(
                    "Directory does not exist or is not a directory: {}",
                    args_directory.display()
                );
                return;
            }
            let abs_directory = args_directory.canonicalize().unwrap();
            let mut log = Log::new(abs_directory.clone());
            organize_files(&abs_directory, &mut log);
            remove_empty_directories(&abs_directory, &mut log);
            write_log(&mut log);
        }
    }
}

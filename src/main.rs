use clap::Parser;

mod args;
mod confs;
mod log;
mod logging;
mod organizer;
mod restore;

use args::Args;
use confs::set_verbose;
use log::{Log, write_log};
use logging::log_error;
use organizer::{organize_files, remove_empty_directories};
use restore::restore_state;
use std::sync::{Arc, Mutex};

fn main() {
    let args = Args::parse();
    set_verbose(!args.quiet);
    // start the logging system
    logging::init();

    // validate args
    match !args.restore.is_none() {
        true => {
            let log_file = args.restore.unwrap();
            // restore file must exist
            if !log_file.exists() {
                log_error(&format!("Log file does not exist: {}", log_file.display()));
                return;
            }
            let abs_log_file = log_file.canonicalize().unwrap();
            restore_state(&abs_log_file);
        }
        false => {
            // directory must exist and be a directory
            if args.directory.is_none() {
                log_error(&format!(
                    "No directory specified. Use --directory to specify a directory."
                ));
                return;
            }

            let args_directory = args.directory.unwrap();

            if !args_directory.exists() || !args_directory.is_dir() {
                log_error(&format!(
                    "Directory does not exist or is not a directory: {}",
                    args_directory.display()
                ));
                return;
            }
            let abs_directory = args_directory.canonicalize().unwrap();

            // log file inside a mutex for parallel processing
            let log = Arc::new(Mutex::new(Log::new(abs_directory.clone())));

            organize_files(&abs_directory, log.clone());

            let mut log = Arc::try_unwrap(log)
                .expect("Log still has multiple owners")
                .into_inner()
                .unwrap();
            remove_empty_directories(&abs_directory, &mut log);
            write_log(&mut log);
        }
    }
}

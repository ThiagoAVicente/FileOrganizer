use crate::log::{Log, log_from_file};
use crate::organizer::remove_empty_directories;
use std::fs;
use std::path::PathBuf;

/// restore the directory structure based on the log file
/// # Arguments
/// * 'log_file' - the path to the log file
pub fn restore_state(log_file: &PathBuf) {
    let state = log_from_file(log_file);

    // validate state
    if !state.base_directory.exists() || !state.base_directory.is_dir() {
        eprintln!(
            "Base directory does not exist or is not a directory: {}",
            state.base_directory.display()
        );
        return;
    }

    // restore removed directories if any
    if !state.removed_directories.is_empty() {
        // restore removed directories
        for dir in &state.removed_directories {
            match fs::create_dir_all(dir) {
                Ok(_) => println!("Restored directory: {}", dir.display()),
                Err(e) => eprintln!("Failed to restore directory {}: {}", dir.display(), e),
            }
        }
    } else {
        println!("No directories to restore.");
    }

    // restore moved files
    for entry in &state.moves {
        let old_path = &entry.old_path;
        let new_path = &entry.new_path;

        // check if the old path exists
        if !new_path.exists() {
            eprintln!("Path does not exist: {}", new_path.display());
            continue;
        }

        // create parent directory for the new path if it doesn't exist
        if let Some(parent) = old_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }

        // move the file back to its original location
        match fs::rename(new_path, old_path) {
            Ok(_) => println!(
                "Restored file: {} to {}",
                new_path.display(),
                old_path.display()
            ),
            Err(e) => eprintln!("Failed to restore file {}: {}", new_path.display(), e),
        }
    }

    // remove empty directories after restoring
    remove_empty_directories(
        &state.base_directory,
        &mut Log::new(state.base_directory.clone()),
    );
    // remove the write log
    if log_file.exists() {
        match fs::remove_file(log_file) {
            Ok(_) => println!("Removed log file: {}", log_file.display()),
            Err(e) => eprintln!("Failed to remove log file {}: {}", log_file.display(), e),
        }
    } else {
        println!("Log file does not exist, nothing to remove.");
    }
}

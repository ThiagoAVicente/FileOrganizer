use crate::log::{Log, log_from_file};
use crate::logging::{log_error, log_info};
use crate::organizer::{move_file, remove_empty_directories};
use std::fs;
use std::path::PathBuf;

/// restore the directory structure based on the log file
/// # Arguments
/// * 'log_file' - the path to the log file
pub fn restore_state(log_file: &PathBuf) {
    let state = log_from_file(log_file);

    // validate state
    if !state.base_directory().exists() || !state.base_directory().is_dir() {
        log_error(&format!(
            "Base directory does not exist or is not a directory: {}",
            state.base_directory().display()
        ));
        return;
    }

    // restore removed directories if any
    if !state.removed_directories().is_empty() {
        // restore removed directories
        for dir in state.removed_directories() {
            match fs::create_dir_all(dir) {
                Ok(_) => log_info(&format!("Restored directory: {}", dir.display())),
                Err(e) => log_error(&format!(
                    "Failed to restore directory {}: {}",
                    dir.display(),
                    e
                )),
            }
        }
    } else {
        log_info("No directories to restore.");
    }

    let mut dummy_log = Log::new(state.base_directory().clone()); // just so I can use the move_file function

    // restore moved files
    for entry in state.moves() {
        let old_path = &entry.old_path();
        let new_path = &entry.new_path();

        // check if the old path exists
        if !new_path.exists() {
            log_error(&format!("Path does not exist: {}", new_path.display()));
            continue;
        }

        // create parent directory for the new path if it doesn't exist
        if let Some(parent) = old_path.parent() {
            fs::create_dir_all(parent).expect("Failed to create parent directory");
        }

        // move the file back to its original location
        move_file(new_path, old_path, &mut dummy_log);
    }

    // remove empty directories after restoring
    remove_empty_directories(
        &state.base_directory(),
        &mut Log::new(state.base_directory().clone()),
    );
    // remove the write log
    if log_file.exists() {
        match fs::remove_file(log_file) {
            Ok(_) => log_info(&format!("Removed log file: {}", log_file.display())),
            Err(e) => log_error(&format!(
                "Failed to remove log file {}: {}",
                log_file.display(),
                e
            )),
        }
    } else {
        log_info("Log file does not exist, nothing to remove.");
    }
}

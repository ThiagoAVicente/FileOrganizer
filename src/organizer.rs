use crate::log::{Log};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use walkdir::WalkDir;

/// remove empty directories recursively
/// # Arguments
/// * 'directory' - the directory to remove empty directories from
pub fn remove_empty_directories(directory: &PathBuf, log: &mut Log) {
    // iterate recursively through the directory

    let mut dirs: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
        .collect();

    dirs.reverse();

    for entry in dirs {
        let path = entry.path();
        // check if the path is a directory
        if path.is_dir() {
            // check if the directory is empty
            if fs::read_dir(path).unwrap().next().is_none() {
                // remove the empty directory
                match fs::remove_dir(path) {
                    Ok(_) => {
                        println!("Removed empty directory: {}", path.display());
                        // push the removed directory to the log
                        log.remove_directory(path.to_path_buf());
                    }
                    Err(e) => eprintln!("Failed to remove directory {}: {}", path.display(), e),
                }
                continue;
            }
        }
    }
}

/// organize files in the directory by their extensions
/// # Arguments
/// * 'directory' - the directory to organize files in
/// * 'log' - the log to write the changes to
pub fn organize_files(directory: &PathBuf, log_mut: Arc<Mutex<Log>>) {
    // iterate recursively through the directory

    // find all entries 
    let entries: Vec<_> = WalkDir::new(directory)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .collect();

    // use parallel iterator to process files concurrently
    entries.par_iter().for_each(|entry| {
        let path = entry.path();

        // check if path is a file and
        if path.is_file() {
            // check if the extension directory was already created
            let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");
            let new_path;
            let to_create;

            if extension.is_empty() {
                // place files without an extension in a "no_extension" directory
                to_create = directory.join("no_extension");
            } else {
                // create the extension directory if it doesn't exist
                to_create = directory.join(extension);
            }
            
            match fs::create_dir_all(&to_create) {
                Ok(_) => {
                    // move the file to the no_extension directory
                    new_path = to_create.join(path.file_name().unwrap());
                    let mut log = log_mut.lock().unwrap();
                    log.create_directory(to_create);
                    move_file(path, new_path.as_path(), &mut *log);
                    
                }
                Err(e) => {
                    eprintln!("Failed to create directory {}: {}", to_create.display(), e);
                    // do nothing
                }
            }
            
            // lock the log for writting
        }
    });
}

/// move a file to a new path, renaming it if the new path already exists
/// # Arguments
/// * 'old_path' - the path to the file to move
/// * 'new_path' - the path to move the file to
/// * 'log' - the log to write the changes to
pub fn move_file(old_path: &Path, new_path: &Path, log: &mut Log) {
    let old_path_save = old_path.to_path_buf();
    let mut candidate = new_path.to_path_buf();
    let mut counter = 1;

    // find a unique file name to avoid overwriting existing files
    while candidate.exists() {
        let file_stem = new_path.file_stem().unwrap().to_str().unwrap();
        let extension = new_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let new_file_name = if extension.is_empty() {
            format!("{} ({})", file_stem, counter)
        } else {
            format!("{} ({}).{}", file_stem, counter, extension)
        };
        candidate = new_path.with_file_name(new_file_name);
        counter += 1;
    }

    match fs::rename(old_path, &candidate) {
        Ok(_) => log.move_file(old_path_save, candidate),
        Err(e) => eprintln!("Failed to move file {}: {}", old_path.display(), e),
    }
}

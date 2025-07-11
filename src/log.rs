use std::fs;
use crate::logging::{log_info, log_error};
use std::path::PathBuf;

#[derive(Debug)]
pub struct LogEntry {
    old_path: PathBuf,
    new_path: PathBuf,
}
impl LogEntry {
    pub fn new(old_path: PathBuf, new_path: PathBuf) -> Self {
        LogEntry { old_path, new_path }
    }
    pub fn to_string(&self) -> String {
        format!("{} -> {}", self.old_path.display(), self.new_path.display())
    }
    pub fn old_path(&self) -> &PathBuf {
        &self.old_path
    }
    pub fn new_path(&self) -> &PathBuf {
        &self.new_path
    }
}

#[derive(Debug)]
pub struct Log {
    base_directory: PathBuf,
    created_directories: Vec<PathBuf>,
    removed_directories: Vec<PathBuf>,
    moves: Vec<LogEntry>,
}
impl Log {
    pub fn new(base_directory: PathBuf) -> Self {
        Log {
            base_directory,
            created_directories: Vec::new(),
            removed_directories: Vec::new(),
            moves: Vec::new(),
        }
    }
    pub fn create_directory(&mut self, path: PathBuf) {
        if !self.created_directories.contains(&path) {
            self.created_directories.push(path);
        }
    }
    pub fn remove_directory(&mut self, path: PathBuf) {
        if !self.removed_directories.contains(&path) {
            self.removed_directories.push(path);
        }
    }
    pub fn move_file(&mut self, old_path: PathBuf, new_path: PathBuf) {
        self.moves.push(LogEntry::new(old_path, new_path));
    }

    pub fn base_directory(&self) -> &PathBuf {
        &self.base_directory
    }
    pub fn removed_directories(&self) -> &Vec<PathBuf> {
        &self.removed_directories
    }
    pub fn moves(&self) -> &Vec<LogEntry> {
        &self.moves
    }
    pub fn created_directories(&self) -> &Vec<PathBuf> {
        &self.created_directories
    }
    
    pub fn to_string(&self) -> String {
        let mut log_content = String::new();
        log_content.push_str(&self.base_directory.to_string_lossy());
        for dir in &self.created_directories {
            log_content.push_str(&format!("\n+ {}", dir.display()));
        }
        for entry in &self.removed_directories {
            log_content.push_str(&format!("\n{}", entry.display()));
        }
        for entry in &self.moves {
            log_content.push_str(&format!("\n{}", entry.to_string()));
        }
        log_content
    }
}

/// read the log file and return a Log struct
/// # Arguments
/// * 'log_file' - the path to the log file
pub fn log_from_file(log_file: &PathBuf) -> Log {
    let content = fs::read_to_string(log_file).expect("Failed to read log file");
    let mut lines = content.lines();

    // process file content
    let base = PathBuf::from(lines.next().expect("Log file is empty"));
    let mut removed_directories = Vec::new();
    let mut created_directories = Vec::new();
    
    let mut moves = Vec::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        // check if the line is a move entry or a removed directory
        if let Some((old_path, new_path)) = line.split_once(" -> ") {
            moves.push(LogEntry::new(
                PathBuf::from(old_path),
                PathBuf::from(new_path),
            ));
        } else if line.starts_with("+ "){
           created_directories.push(PathBuf::from(line.trim_start_matches("+ "))); 
        } else {
            removed_directories.push(PathBuf::from(line));
        }
    }

    // create log data structure
    let response = Log {
        base_directory: base,
        created_directories:created_directories,
        removed_directories:removed_directories,
        moves:moves,
    };

    return response;
}

/// write logs file containing the changes made to the directory
/// # Arguments
/// * 'log' - the log to write to a file
pub fn write_log(log: &mut Log) {
    let log_file = log.base_directory.join(".file_organizer_log");
    let log_content = log.to_string();
    // write to the file
    match fs::write(&log_file, log_content) {
        Ok(_) =>  log_info(&format!("Log written to {}", log_file.display())),
        Err(e) => log_error(&format!("Failed to write log file {}: {}", log_file.display(), e)),
    }
}

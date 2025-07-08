use chrono::Local;
use std::fs;
use std::path::PathBuf;

pub struct LogEntry {
    pub old_path: PathBuf,
    pub new_path: PathBuf,
}
impl LogEntry {
    pub fn new(old_path: PathBuf, new_path: PathBuf) -> Self {
        LogEntry { old_path, new_path }
    }
    pub fn to_string(&self) -> String {
        format!("{} -> {}", self.old_path.display(), self.new_path.display())
    }
}

pub struct Log {
    pub base_directory: PathBuf,
    pub removed_directories: Vec<PathBuf>,
    pub moves: Vec<LogEntry>,
}
impl Log {
    pub fn new(base_directory: PathBuf) -> Self {
        Log {
            base_directory,
            removed_directories: Vec::new(),
            moves: Vec::new(),
        }
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
        } else {
            removed_directories.push(PathBuf::from(line));
        }
    }

    // create log data structure
    let response = Log {
        base_directory: base,
        removed_directories,
        moves,
    };

    return response;
}

/// write logs file containing the changes made to the directory
/// # Arguments
/// * 'log' - the log to write to a file
pub fn write_log(log: &mut Log) {
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let log_file = log.base_directory.join(format!("log_{}.txt", timestamp));
    let mut log_content = String::new();

    log_content.push_str(log.base_directory.to_str().unwrap());
    for entry in &log.removed_directories {
        log_content.push_str(&format!("\n{}", entry.display()));
    }
    for entry in &log.moves {
        log_content.push_str(&format!("\n{}", entry.to_string()));
    }

    // write to the file
    match fs::write(&log_file, log_content) {
        Ok(_) => println!("Log written to {}", log_file.display()),
        Err(e) => eprintln!("Failed to write log file {}: {}", log_file.display(), e),
    }
}

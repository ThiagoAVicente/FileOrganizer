use std::sync::atomic::{AtomicBool, Ordering};

static VERBOSE: AtomicBool = AtomicBool::new(true);
pub const LOG_FILE_NAME: &str = ".file_organizer_log";

pub fn is_verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}
pub fn set_verbose(value: bool) {
    VERBOSE.store(value, Ordering::Relaxed);
}

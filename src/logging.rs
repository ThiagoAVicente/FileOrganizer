use crate::confs::is_verbose;

pub fn log_info(message: &str) {
    if is_verbose() {
        log::info!("{}", message);
    }
}
pub fn log_error(message: &str) {
    if is_verbose() {
        log::error!("{}", message);
    }
}
pub fn init() {
    use env_logger::Env;
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
}

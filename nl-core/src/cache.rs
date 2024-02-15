use std::path::PathBuf;

pub fn mkdir_if_not_exists(path: &PathBuf) {
    if !path.exists() {
        log::info!("Creating directory: {:?}", &path);
        std::fs::create_dir_all(path).expect("Failed to create directory");
    }
}
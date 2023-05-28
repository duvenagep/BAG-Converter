use std::env;
use std::fs;
use std::path::PathBuf;

pub fn new_folder(new_folder_name: &str) -> Result<PathBuf, String> {
    let current_dir = match env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            return Err(format!("Failed to get current directory: {}", e));
        }
    };

    let new_dir = current_dir.join(new_folder_name);
    match fs::create_dir(&new_dir) {
        Ok(_) => Ok(new_dir),
        Err(e) => Err(format!("Failed to create new directory: {}", e)),
    }
}

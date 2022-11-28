pub mod channel;
pub mod crypt;
pub mod id;
pub mod message;
pub mod user;

#[cfg(target_os = "windows")]
pub fn get_config_dir() -> String {
    let mut path = std::env::var("APPDATA").unwrap();
    path.push_str("\\yuttari");

    // create the directory if it doesn't exist
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path
}

#[cfg(target_os = "linux")]
pub fn get_config_dir() -> String {
    let mut path = std::env::var("HOME").unwrap();
    path.push_str("/.config/yuttari");

    // create the directory if it doesn't exist
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path
}

#[cfg(target_os = "macos")]
pub fn get_config_dir() -> String {
    let mut path = std::env::var("HOME").unwrap();
    path.push_str("/Library/Application Support/yuttari");

    // create the directory if it doesn't exist
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(&path).unwrap();
    }

    path
}

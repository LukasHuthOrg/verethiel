use std::{fs::Permissions, io::Write, os::unix::fs::PermissionsExt, path::PathBuf, str::FromStr};

use tempfile::NamedTempFile;

use super::open_file;

#[test]
#[should_panic(expected = "'nonexistent_file.txt' is not a file.")]
fn test_panic_on_missing_file() {
    _ = open_file(&PathBuf::from_str("nonexistent_file.txt").unwrap()).unwrap();
}

#[test]
#[should_panic(expected = "Failed to open file")]
fn test_panic_on_failed_read() {
    #[cfg(target_os = "windows")]
    panic!("This cannot be tested on Windows, therefore 'Failed to open file' is thrown here");
    let tf = NamedTempFile::new().unwrap();
    #[cfg(target_os = "linux")]
    tf.as_file()
        .set_permissions(Permissions::from_mode(0o000))
        .unwrap();
    _ = open_file(&tf.path().to_path_buf()).unwrap();
}

#[test]
#[should_panic(expected = "Failed to parse file")]
fn test_panic_on_invalid_json() {
    let tf = NamedTempFile::new().unwrap();
    tf.as_file().write_all("<invalid json>".as_bytes()).unwrap();
    _ = open_file(&tf.path().to_path_buf()).unwrap();
}

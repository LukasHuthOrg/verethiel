use std::path::PathBuf;

use tempfile::NamedTempFile;

use crate::utility::Translation;

use super::{sort, sort_file, sort_directory};

const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
const FILE_CONTENT_1: &str = r#"{"c":{"b":"b","a":"a"},"b":"b","a":"a"}"#;
fn setup_correct() -> (NamedTempFile, NamedTempFile) {
    let t0 = NamedTempFile::new().unwrap();
    std::fs::write(t0.path(), FILE_CONTENT_0).unwrap();
    let t1 = NamedTempFile::new().unwrap();
    std::fs::write(t1.path(), FILE_CONTENT_1).unwrap();
    (t0, t1)
}

#[test]
fn test_file_content_validity() {
    let _: Translation = serde_json::from_str(FILE_CONTENT_0).unwrap();
    let _: Translation = serde_json::from_str(FILE_CONTENT_1).unwrap();
}
#[test]
fn test_correct_order_file_file_none() {
    let (t0, t1) = setup_correct();
    sort(t0.path().to_path_buf(), t1.path().to_path_buf(), false, None);
    let result = String::from_utf8(std::fs::read(t1.path()).unwrap()).unwrap();
    assert_eq!(result, FILE_CONTENT_0);
}

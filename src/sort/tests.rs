use tempfile::{NamedTempFile, TempDir};

use crate::utility::Translation;

use super::{sort, sort_directory, sort_file};

const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
const FILE_CONTENT_1: &str = r#"{"c":{"b":"b","a":"a"},"b":"b","a":"a"}"#;
const FILE_CONTENT_2: &str = r#"{"b":"b","c":{"b":"b","a":"a"},"a":"a"}"#;
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
    sort(
        t0.path().to_path_buf(),
        t1.path().to_path_buf(),
        false,
        None,
        false,
    );
    let result = String::from_utf8(std::fs::read(t1.path()).unwrap()).unwrap();
    assert_eq!(result, FILE_CONTENT_0);
}
#[test]
fn test_sort_file() {
    let (_, t1) = setup_correct();
    let base: Translation = serde_json::from_str(FILE_CONTENT_0).unwrap();
    sort_file(t1.path().to_path_buf(), &base, None, false).unwrap();
    let result = String::from_utf8(std::fs::read(t1.path()).unwrap()).unwrap();
    assert_eq!(result, FILE_CONTENT_0);
}
#[test]
fn test_sort_directory() {
    let d0 = TempDir::new().unwrap();
    let t0 = NamedTempFile::new_in(d0.path()).unwrap();
    let t1 = NamedTempFile::new_in(d0.path()).unwrap();
    let t2 = NamedTempFile::new_in(d0.path()).unwrap();
    std::fs::write(t0.path(), FILE_CONTENT_1).unwrap();
    std::fs::write(t1.path(), FILE_CONTENT_2).unwrap();
    let base: Translation = serde_json::from_str(FILE_CONTENT_0).unwrap();
    sort_directory(
        d0.path().to_path_buf(),
        &base,
        None,
        false,
        &t2.path().to_path_buf(),
        false,
    )
    .unwrap();
    let result0 = String::from_utf8(std::fs::read(t0.path()).unwrap()).unwrap();
    let result1 = String::from_utf8(std::fs::read(t1.path()).unwrap()).unwrap();
    assert_eq!(result0, FILE_CONTENT_0);
    assert_eq!(result1, FILE_CONTENT_0);
}
#[test]
#[should_panic(expected = "Failed to open directory")]
fn test_sort_directory_file_as_dir() {
    let base: Translation = serde_json::from_str(FILE_CONTENT_0).unwrap();
    let t = NamedTempFile::new().unwrap();
    sort_directory(
        t.path().to_path_buf(),
        &base,
        None,
        false,
        &t.path().to_path_buf(),
        false,
    )
    .unwrap();
}

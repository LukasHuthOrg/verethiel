use std::io::{Read, Write};

use regex::Regex;
use tempfile::NamedTempFile;

#[test]
fn test_diff_file_nothing_to_do() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut t1 = NamedTempFile::new().unwrap();
    t1.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut t2 = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        t1.path().to_path_buf(),
        false,
        false,
        Some(t2.path().to_path_buf()),
    );
    let mut result = String::new();
    t2.read_to_string(&mut result).unwrap();
    let regex = Regex::new(r"^Diff for /tmp/\.tmp[a-zA-Z0-9]{6}\nnothing\.\n\n$").unwrap();
    assert!(
        regex.is_match(&result),
        "result is: '{result:?}', expected format: {regex:?}"
    );
}
#[test]
fn test_diff_file_missing() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
    const FILE_CONTENT_1: &str = r#"{"a":"a","b":"b","c":{"a":"a"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut t1 = NamedTempFile::new().unwrap();
    t1.write_all(FILE_CONTENT_1.as_bytes()).unwrap();
    let mut t2 = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        t1.path().to_path_buf(),
        false,
        false,
        Some(t2.path().to_path_buf()),
    );
    let mut result = String::new();
    t2.read_to_string(&mut result).unwrap();
    let regex =
        Regex::new(r"^Diff for /tmp/\.tmp[a-zA-Z0-9]{6}\nmissing keys:\nMissing key: c\.b\n\n$")
            .unwrap();
    assert!(
        regex.is_match(&result),
        "result is: '{result:?}', expected format: {regex:?}"
    );
}
#[test]
fn test_diff_file_extra() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a"}}"#;
    const FILE_CONTENT_1: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut t1 = NamedTempFile::new().unwrap();
    t1.write_all(FILE_CONTENT_1.as_bytes()).unwrap();
    let mut t2 = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        t1.path().to_path_buf(),
        false,
        false,
        Some(t2.path().to_path_buf()),
    );
    let mut result = String::new();
    t2.read_to_string(&mut result).unwrap();
    let regex =
        Regex::new(r"^Diff for /tmp/\.tmp[a-zA-Z0-9]{6}\nextra keys:\nExtra key: c\.b\n\n$")
            .unwrap();
    assert!(
        regex.is_match(&result),
        "result is: '{result:?}', expected format: {regex:?}"
    );
}
#[test]
fn test_diff_file_both() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a"}}"#;
    const FILE_CONTENT_1: &str = r#"{"a":"a","b":"b","c":{"b":"b"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut t1 = NamedTempFile::new().unwrap();
    t1.write_all(FILE_CONTENT_1.as_bytes()).unwrap();
    let mut t2 = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        t1.path().to_path_buf(),
        false,
        false,
        Some(t2.path().to_path_buf()),
    );
    let mut result = String::new();
    t2.read_to_string(&mut result).unwrap();
    let regex = Regex::new(r"^Diff for /tmp/\.tmp[a-zA-Z0-9]{6}\nmissing keys:\nMissing key: c\.a\nextra keys:\nExtra key: c\.b\n\n$").unwrap();
    assert!(
        regex.is_match(&result),
        "result is: '{result:?}', expected format: {regex:?}"
    );
}
#[test]
fn test_diff_dir_nothing_to_do() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let d0 = tempfile::tempdir().unwrap();
    let mut t1 = NamedTempFile::new_in(d0.path()).unwrap();
    t1.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut t2 = NamedTempFile::new_in(d0.path()).unwrap();
    t2.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut output = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        d0.path().to_path_buf(),
        false,
        false,
        Some(output.path().to_path_buf()),
    );
    let mut result = String::new();
    output.read_to_string(&mut result).unwrap();
    let temp_path_pattern = r"/tmp(/.tmp[A-Za-z0-9]{6})+";
    let pattern = format!(
        r"^Diff for {}\nnothing\.\n\nDiff for {}\nnothing\.\n\n$",
        temp_path_pattern, temp_path_pattern,
    );
    let regex = Regex::new(&pattern).unwrap();
    assert!(
        regex.is_match(&result),
        "result is: {result:?}, expected format: /{regex}/"
    );
}
#[test]
fn test_diff_dir_nest_no_recurse_nothing_to_do() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let d0 = tempfile::tempdir().unwrap();
    let mut t1 = NamedTempFile::new_in(d0.path()).unwrap();
    t1.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let d1 = tempfile::tempdir_in(d0.path()).unwrap();
    let mut t2 = NamedTempFile::new_in(d1.path()).unwrap();
    t2.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut output = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        d0.path().to_path_buf(),
        false,
        false,
        Some(output.path().to_path_buf()),
    );
    let mut result = String::new();
    output.read_to_string(&mut result).unwrap();
    let temp_path_pattern = r"/tmp(/.tmp[A-Za-z0-9]{6})+";
    let pattern = format!(r"^Diff for {}\nnothing\.\n\n$", temp_path_pattern,);
    let regex = Regex::new(&pattern).unwrap();
    assert!(
        regex.is_match(&result),
        "result is: {result:?}, expected format: /{regex}/"
    );
}
#[test]
fn test_diff_dir_nest_recurse_nothing_to_do() {
    const FILE_CONTENT_0: &str = r#"{"a":"a","b":"b","c":{"a":"a","b":"b"}}"#;
    let mut t0 = NamedTempFile::new().unwrap();
    t0.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let d0 = tempfile::tempdir().unwrap();
    let mut t1 = NamedTempFile::new_in(d0.path()).unwrap();
    t1.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let d1 = tempfile::tempdir_in(d0.path()).unwrap();
    let mut t2 = NamedTempFile::new_in(d1.path()).unwrap();
    t2.write_all(FILE_CONTENT_0.as_bytes()).unwrap();
    let mut output = NamedTempFile::new().unwrap();
    super::diff(
        t0.path().to_path_buf(),
        d0.path().to_path_buf(),
        true,
        false,
        Some(output.path().to_path_buf()),
    );
    let mut result = String::new();
    output.read_to_string(&mut result).unwrap();
    let temp_path_pattern = r"/tmp(/.tmp[A-Za-z0-9]{6})+";
    let pattern = format!(
        r"^Diff for {}\nnothing\.\n\nDiff for {}\nnothing\.\n\n$",
        temp_path_pattern, temp_path_pattern,
    );
    let regex = Regex::new(&pattern).unwrap();
    assert!(
        regex.is_match(&result),
        "result is: {result:?}, expected format: /{regex}/"
    );
}

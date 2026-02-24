use std::path::PathBuf;

use crate::utility::Translation;

pub(crate) fn verify(base_file: PathBuf, source: PathBuf, recursive: bool) {
    let Ok(base) = open_file(base_file) else { return; };
    if source.is_dir() {
        validate_directory(source, &base, recursive)
    } else if source.is_file() {
        validate_file(source, &base)
    } else {
        Ok(())
    }
    .expect("Failed to verify files")
}
fn validate_directory(path: PathBuf, base: &Translation, recursive: bool) -> Result<(), ()> {
    let Ok(dir) = std::fs::read_dir(&path) else {
        eprint!("Failed to open source dir: '{source}'", source = path.display());
        return Err(());
    };
    for entry in dir {
        let Ok(entry) = entry else { continue; };
        let entry_path = entry.path();
        if entry_path.is_dir() {
            if validate_directory(entry_path, base, recursive).is_err() {
                return Err(());
            }
        } else if entry_path.is_file() {
            validate_file(entry_path, base)?;
        }
    }
    Ok(())
}
fn validate_file(path: PathBuf, base: &Translation) -> Result<(), ()> {
    let mut translation = open_file(path.clone())?;
    if let Err(key) = translation.visit_translation(base) {
        eprintln!("Failed to find '{key}' in '{}'", path.display());
        return Err(());
    }
    if !translation.everything_visited() {
        let mut base_translation: Translation = base.clone();
        if let Err(key) = base_translation.visit_translation(&translation) {
            eprintln!(
                "Found '{key}' in '{}' which is not present in base",
                path.display()
            );
            return Err(());
        }
    }
    Ok(())
}
fn open_file(path: PathBuf) -> Result<Translation, ()> {
    if !path.is_file() {
        eprintln!("'{path}' is not a file.", path = path.display());
        return Err(());
    }
    let Ok(file_content) = std::fs::read_to_string(&path) else {
        eprint!("Failed to open file '{base}'", base = path.display());
        return Err(());
    };
    let Ok(result): Result<Translation, serde_json::Error> = serde_json::from_str(&file_content) else {
        eprint!("Failed to parse file '{path}'", path = path.display());
        return Err(());
    };
    Ok(result)
}

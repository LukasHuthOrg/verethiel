use std::{path::PathBuf, process::exit};

use crate::utility::{Translation, open_file};

pub(crate) fn verify(base_file: PathBuf, source: PathBuf, recursive: bool, strict: bool) {
    let base = open_file(&base_file).unwrap();

    if let Err(err) = if source.is_dir() {
        validate_directory(source, &base, recursive, strict)
    } else if source.is_file() {
        validate_file(source, &base, strict)
    } else {
        Ok(())
    } {
        eprintln!("Failed to veify files: {err}");
        exit(1);
    }
}
fn validate_directory(
    path: PathBuf,
    base: &Translation,
    recursive: bool,
    strict: bool,
) -> Result<(), String> {
    let Ok(dir) = std::fs::read_dir(&path) else {
        return Err(format!(
            "Failed to open source dir: '{source}'",
            source = path.display()
        ));
    };
    for entry in dir {
        let Ok(entry) = entry else {
            continue;
        };
        let entry_path = entry.path();
        if entry_path.is_dir() {
            validate_directory(entry_path, base, recursive, strict)?;
        } else if entry_path.is_file() {
            validate_file(entry_path, base, strict)?;
        }
    }
    Ok(())
}
pub(crate) fn validate_file(path: PathBuf, base: &Translation, strict: bool) -> Result<(), String> {
    let mut translation = open_file(&path)?;
    let visit_fn = if strict {
        Translation::visit_ordered_translation::<true>
    } else {
        Translation::visit_translation
    };
    if let Err(key) = visit_fn(&mut translation, base) {
        return Err(format!("Failed to find '{key}' in '{}'", path.display()));
    }
    if !translation.everything_visited() {
        let mut base_translation: Translation = base.clone();
        if let Err(key) = base_translation.visit_translation(&translation) {
            return Err(format!(
                "Found '{key}' in '{}' which is not present in base",
                path.display()
            ));
        }
    }
    Ok(())
}

#[cfg(test)]
mod test;

use std::path::PathBuf;

use crate::utility::{Translation, open_file};

pub(crate) fn sort(base_path: PathBuf, source: PathBuf, recursive: bool, output: Option<PathBuf>) {
    if let Some(output) = &output {
        let both_dir = source.is_dir() && output.is_dir();
        let both_file = source.is_file() && output.is_file();
        if !both_file && !both_dir {
            panic!(
                "When using output, output and source must either be a directory or file, they can't be different"
            );
        }
    }
    let base = open_file(base_path.clone()).unwrap();
    if source.is_dir() {
        sort_directory(source, &base, output, recursive, &base_path)
    } else if source.is_file() {
        sort_file(source, &base, output, &base_path)
    } else {
        panic!("source is neither and existing file nor an existing directory")
    }
    .unwrap()
}
fn sort_directory(
    path: PathBuf,
    base: &Translation,
    output: Option<PathBuf>,
    recursive: bool,
    base_path: &PathBuf,
) -> Result<(), String> {
    for entry in std::fs::read_dir(&path)
        .map_err(|_| format!("Failed to open directory '{}'", path.display()))?
    {
        let Ok(entry) = entry else {
            continue;
        };
        let new_output = output.clone().map(|path| path.join(entry.path()));
        if recursive && entry.path().is_dir() {
            sort_directory(entry.path(), base, new_output, recursive, base_path)?;
        } else if entry.path().is_file() {
            sort_file(entry.path(), base, new_output, base_path)?;
        }
    }
    Ok(())
}
fn sort_file(
    path: PathBuf,
    base: &Translation,
    output: Option<PathBuf>,
    base_path: &PathBuf,
) -> Result<(), String> {
    if &path == base_path {
        return Ok(());
    }
    let output_path = output.unwrap_or_else(|| path.clone());
    let mut translation = open_file(path)?;
    translation.apply_translation_order(base)?;
    std::fs::write(output_path, translation.to_string())
        .map_err(|err| format!("Failed to write to file: {err}"))
}

#[cfg(test)]
mod tests;

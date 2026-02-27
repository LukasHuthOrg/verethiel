use std::{path::PathBuf, process::exit};

use crate::utility::{Translation, open_file};

pub(crate) fn diff(
    base_path: PathBuf,
    source: PathBuf,
    recursive: bool,
    fix: bool,
    output: Option<PathBuf>,
) {
    let base = match open_file(&base_path) {
        Ok(base) => base,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };
}
struct Diff {
    /// Index into keys
    missing: Vec<usize>,
    /// Index into keys
    extra: Vec<usize>,
    // placeholder_missmatch: Vec<PlaceholderDiff<'a>>
    translation: Translation,
    file_path: PathBuf,
}
fn diff_directory(
    path: &PathBuf,
    base: &Translation,
    recursive: bool,
    fix: bool,
    base_path: &PathBuf,
) -> (Vec<Diff>, Vec<String>) {
    let _: (Vec<Diff>, Vec<String>) = std::fs::read_dir(&path)
        .map_err(|_| format!("Failed to open directory '{}'", path.display()))?
        .filter_map(Result::ok)
        .map(|entry| {
            if recursive && entry.path().is_dir() {
                diff_directory(&entry.path(), base, recursive, fix, base_path)
            } else if entry.path().is_file() && base_path != &entry.path() {
                diff_file(&entry.path(), base, fix)
                    .map(|v| (v, Vec::new()))
                    .unwrap_or_else(|e| (Vec::new(), vec![e]))
            } else {
                (Vec::new(), Vec::new())
            }
        })
        .fold((Vec::new(), Vec::new()), agg_res_and_errs);
    todo!()

    // for entry in std::fs::read_dir(&path)
    //     .map_err(|_| format!("Failed to open directory '{}'", path.display()))?
    // {
    //     let Ok(entry) = entry else {
    //         continue;
    //     };
    //     let new_output = output.clone().map(|path| path.join(entry.path()));
    //     if recursive && entry.path().is_dir() {
    //         sort_directory(entry.path(), base, new_output, recursive, base_path, strict)?;
    //     } else if entry.path().is_file() && base_path != &entry.path() {
    //         println!("'{}' '{}'", base_path.display(), entry.path().display());
    //         sort_file(entry.path(), base, new_output, strict)?;
    //     }
    // }
    // Ok(())
}
type Aggregate = (Vec<Diff>, Vec<String>);
fn agg_res_and_errs(mut agg: Aggregate, mut value: Aggregate) -> Aggregate {
    agg.0.append(&mut value.0);
    agg.1.append(&mut value.1);
    agg
}
fn diff_file(path: &PathBuf, base: &Translation, fix: bool) -> Result<Vec<Diff>, String> {
    todo!()
}

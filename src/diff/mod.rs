use std::{
    collections::{vec_deque, VecDeque},
    fmt::{Display, Write},
    fs::DirEntry,
    io::Write as _,
    path::{Path, PathBuf},
    process::exit,
};

use crate::utility::{
    open_file,
    translation::{KeyToString as _, ToKeyArray},
    ToSliceArr, Translation,
};

#[cfg(test)]
mod tests;

pub(crate) fn diff(
    base_path: PathBuf,
    source: PathBuf,
    recursive: bool,
    fix: bool,
    output: Option<PathBuf>,
) {
    if fix {
        unimplemented!("The fix flag is currently not implemented");
    }
    let base = match open_file(&base_path) {
        Ok(base) => base,
        Err(err) => {
            eprintln!("{err}");
            exit(1);
        }
    };
    let base_keys = base.get_keys().keys();
    let base_keys = base_keys.to_slice();
    if source.is_dir() {
        let result =
            diff_directory(&source, &base, recursive, fix, &base_path, &base_keys, 0).base(base);
        print_result(result, output);
    } else if source.is_file() {
        let result = DiffResults::new(diff_file(&source, &base, &base_keys, fix)).base(base);
        print_result(result, output);
    }
}
fn print_result(diffresults: DiffResults, output: Option<PathBuf>) {
    if let Some(output) = output {
        let mut file = std::fs::OpenOptions::new()
            .read(false)
            .write(true)
            .create(true)
            .truncate(true)
            .open(output)
            .expect("file could not be opened or created");
        file.write_fmt(format_args!("{diffresults}"))
            .expect("Failed to write to file");
    } else {
        std::io::stdout()
            .write_fmt(format_args!("{diffresults}"))
            .expect("Failed to write to stdout");
    }
}
const MAX_DEPTH: usize = 10;
#[derive(Debug, Default)]
struct Diff {
    /// Index into keys
    missing: Vec<usize>,
    /// Index into keys
    extra: Vec<usize>,
    // placeholder_mismatch: Vec<PlaceholderDiff<'a>>
    translation: Translation,
    file_path: PathBuf,
}
fn diff_directory(
    path: &Path,
    base: &Translation,
    recursive: bool,
    fix: bool,
    base_path: &Path,
    base_keys: &Vec<&[&String]>,
    depth: usize,
) -> DiffResults {
    if depth >= MAX_DEPTH {
        return DiffResults::new_err(format!(
            "Recursion too deep, MAX_DEPTH is {MAX_DEPTH} tried with: {}",
            path.display()
        ));
    }
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return DiffResults::new_err(format!("Failed to open directory '{}'", path.display()));
    };
    read_dir
        .map(|entry| {
            process_entry_with_errors(entry, recursive, base_path, fix, base, base_keys, depth)
        })
        .collect()
}
fn process_entry_with_errors(
    maybe_entry: Result<DirEntry, std::io::Error>,
    recursive: bool,
    base_path: &Path,
    fix: bool,
    base: &Translation,
    base_keys: &Vec<&[&String]>,
    depth: usize,
) -> DiffResults {
    match maybe_entry {
        Ok(entry) => process_entry(entry, recursive, base_path, fix, base, base_keys, depth),
        Err(err) => DiffResults::new_err(format!("Failed to read dir entry: {err}")),
    }
}
fn process_entry(
    entry: DirEntry,
    recursive: bool,
    base_path: &Path,
    fix: bool,
    base: &Translation,
    base_keys: &Vec<&[&String]>,
    depth: usize,
) -> DiffResults {
    let path = entry.path();
    if recursive && path.is_dir() {
        diff_directory(&path, base, recursive, fix, base_path, base_keys, depth + 1)
    } else if path.is_file() && base_path != path {
        DiffResults::new(diff_file(&path, base, base_keys, fix))
    } else {
        DiffResults::default()
    }
}
type DiffResult = Result<Diff, String>;
#[derive(Debug, Default)]
struct DiffResults {
    diffs: Vec<DiffResult>,
    base: Option<Translation>,
}
impl DiffResults {
    pub fn new(value: DiffResult) -> Self {
        Self {
            diffs: vec![value],
            base: None,
        }
    }
    pub fn new_err(err: String) -> Self {
        Self {
            diffs: vec![Err(err)],
            base: None,
        }
    }
    pub fn base(mut self, base: Translation) -> Self {
        self.base = Some(base);
        self
    }
}
impl FromIterator<DiffResults> for DiffResults {
    fn from_iter<T: IntoIterator<Item = DiffResults>>(iter: T) -> Self {
        Self {
            diffs: iter.into_iter().flat_map(|d| d.diffs).collect(),
            base: None,
        }
    }
}
impl Display for DiffResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = self
            .base
            .as_ref()
            .expect("There should always be a base attached when printing");
        let base_keys = base.get_keys();
        let base_keys: Vec<Vec<&String>> = base_keys
            .into_iter()
            .map(VecDeque::into_iter)
            .map(vec_deque::IntoIter::collect)
            .collect();
        let base_keys = base_keys.iter().map(Vec::as_slice).collect::<Vec<_>>();
        for (i, diff) in self.diffs.iter().enumerate() {
            match diff {
                Ok(diff) => f.write_fmt(format_args!("{}", DiffWithBase::new(diff, &base_keys)))?,
                Err(err) => f.write_fmt(format_args!("An error occured: {err}\n"))?,
            }
            if self.diffs.len() > i {
                f.write_char('\n')?;
            }
        }
        Ok(())
    }
}
struct DiffWithBase<'a> {
    diff: &'a Diff,
    base_keys: &'a Vec<&'a [&'a String]>,
}
impl<'a> DiffWithBase<'a> {
    pub fn new(diff: &'a Diff, base_keys: &'a Vec<&'a [&'a String]>) -> Self {
        Self { diff, base_keys }
    }
}
impl<'a> Display for DiffWithBase<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let path = &self.diff.file_path;
        f.write_fmt(format_args!("Diff for {}\n", path.display()))?;
        if self.diff.missing.is_empty() && self.diff.extra.is_empty() {
            return f.write_str("nothing.\n");
        }
        if !self.diff.missing.is_empty() {
            f.write_str("missing keys:\n")?;
            for &missing in self.diff.missing.iter() {
                let key = self
                    .base_keys
                    .get(missing)
                    .expect("Any used index should be in the key list")
                    .to_string();
                f.write_fmt(format_args!("Missing key: {key}\n"))?;
            }
        }
        if !self.diff.extra.is_empty() {
            let keys = self.diff.translation.get_keys();
            let keys: Vec<Vec<&String>> = keys
                .into_iter()
                .map(VecDeque::into_iter)
                .map(vec_deque::IntoIter::collect)
                .collect();
            let keys = keys.iter().map(Vec::as_slice).collect::<Vec<_>>();
            f.write_str("extra keys:\n")?;
            for &missing in self.diff.extra.iter() {
                let key = keys
                    .get(missing)
                    .expect("Any used index should be in the key list")
                    .to_string();
                f.write_fmt(format_args!("Extra key: {key}\n"))?;
            }
        }
        Ok(())
    }
}
fn diff_file(
    path: &Path,
    base: &Translation,
    base_keys: &Vec<&[&String]>,
    _fix: bool,
) -> DiffResult {
    let mut extra = Vec::new();
    let mut missing = Vec::new();
    let translation = open_file(path)?;
    let keys = translation.get_keys().keys();
    let keys = keys.to_slice();
    for (i, key) in keys.into_iter().enumerate() {
        if base.contains_key(key) {
            continue;
        }
        extra.push(i);
    }
    for (i, key) in base_keys.iter().enumerate() {
        if translation.contains_key(key) {
            continue;
        }
        missing.push(i);
    }
    Ok(Diff {
        missing,
        extra,
        translation,
        file_path: path.to_path_buf(),
    })
}

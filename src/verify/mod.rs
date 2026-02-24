use std::path::PathBuf;

use crate::utility::Translation;

pub(crate) fn verify(base_file: PathBuf, source: PathBuf) {
    let Ok(base) = std::fs::read_to_string(&base_file) else {
        eprint!("Failed to open file '{base}'", base = base_file.display());
        return;
    };
    let Ok(base): Result<Translation, serde_json::Error> = serde_json::from_str(&base) else {
        eprint!("Failed to parse file '{base}'", base = base_file.display());
        return;
    };
    if source.is_dir() {
        let Ok(dir) = std::fs::read_dir(&source) else {
            eprint!("Failed to open source dir: '{source}'", source = source.display());
            return;
        };
        for entry in dir {
            let Ok(entry) = entry else { continue; };
            if !entry.path().is_file() { continue; };
            todo!();
        }
    } else if source.is_file() {
        todo!();
    }
}

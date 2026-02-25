use std::path::PathBuf;

use crate::utility::Translation;

pub fn open_file(path: PathBuf) -> Result<Translation, String> {
    if !path.is_file() {
        return Err(format!("'{path}' is not a file.", path = path.display()));
    }
    let Ok(file_content) = std::fs::read_to_string(&path) else {
        return Err(format!(
            "Failed to open file '{base}'",
            base = path.display()
        ));
    };
    let Ok(result): Result<Translation, serde_json::Error> = serde_json::from_str(&file_content)
    else {
        return Err(format!(
            "Failed to parse file '{path}'",
            path = path.display()
        ));
    };
    Ok(result)
}

#[cfg(test)]
mod tests;


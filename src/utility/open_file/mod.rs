use std::path::Path;

use crate::utility::Translation;

pub fn open_file(path: &Path) -> Result<Translation, String> {
    if !path.is_file() {
        return Err(format!("'{path}' is not a file.", path = path.display()));
    }
    let Ok(file_content) = std::fs::read_to_string(&path) else {
        return Err(format!(
            "Failed to open file '{base}'",
            base = path.display()
        ));
    };
    match serde_json::from_str(&file_content) {
        Ok(result) => Ok(result),
        Err(err) => Err(format!(
            "Failed to parse file '{path}': {err} '{file_content}'",
            path = path.display()
        )),
    }
}

#[cfg(test)]
mod tests;

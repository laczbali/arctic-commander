use std::path::{Path, PathBuf};

pub fn list_dir(path: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();

    if !path.is_dir() {
        return result;
    }

    let dir = path.read_dir();
    let entries = match dir {
        Ok(entries) => entries,
        Err(_) => return result,
    };

    for entry in entries {
        match entry {
            Ok(entry) => {
                result.push(entry.path());
            }
            Err(_) => continue,
        };
    }

    return result;
}

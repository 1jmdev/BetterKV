use std::fs;
use std::path::{Path, PathBuf};

pub fn discover_rtest_files(root: &Path) -> Result<Vec<PathBuf>, String> {
    if !root.exists() {
        return Err(format!("path does not exist: {}", root.display()));
    }

    let mut found = Vec::new();
    visit(root, &mut found)?;
    found.sort();
    Ok(found)
}

fn visit(path: &Path, found: &mut Vec<PathBuf>) -> Result<(), String> {
    let metadata = fs::metadata(path)
        .map_err(|err| format!("failed to read metadata for {}: {err}", path.display()))?;

    if metadata.is_file() {
        if path.extension().and_then(|value| value.to_str()) == Some("rtest") {
            found.push(path.to_path_buf());
        }
        return Ok(());
    }

    let entries = fs::read_dir(path)
        .map_err(|err| format!("failed to read directory {}: {err}", path.display()))?;

    for entry in entries {
        let entry = entry.map_err(|err| format!("failed to read directory entry: {err}"))?;
        visit(&entry.path(), found)?;
    }

    Ok(())
}

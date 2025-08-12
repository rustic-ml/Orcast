use crate::error::{OrcastError, Result};
use std::fs;
use std::path::Path;

pub fn prepare_result_dir(dir: &str) -> Result<()> {
    let p = Path::new(dir);
    if p.exists() {
        for entry in fs::read_dir(p).map_err(|e| OrcastError::Config(format!("read_dir: {e}")))? {
            let entry = entry.map_err(|e| OrcastError::Config(format!("dir entry: {e}")))?;
            let path = entry.path();
            if path.is_file() {
                fs::remove_file(&path).map_err(|e| OrcastError::Config(format!("remove_file: {e}")))?;
            } else if path.is_dir() {
                fs::remove_dir_all(&path).map_err(|e| OrcastError::Config(format!("remove_dir_all: {e}")))?;
            }
        }
    } else {
        fs::create_dir_all(p).map_err(|e| OrcastError::Config(format!("create_dir_all: {e}")))?;
    }
    Ok(())
}



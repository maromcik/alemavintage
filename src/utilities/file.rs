use crate::error::AppError;

pub fn remove_file(path: &str) -> Result<(), AppError> {
    let fs_path = format!(".{path}");
    if !path.is_empty() && std::path::Path::new(&fs_path).exists() {
        std::fs::remove_file(&fs_path)?;
    }
    Ok(())
}
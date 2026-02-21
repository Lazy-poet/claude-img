use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub path: PathBuf,
}

/// Validate that the file is a real image and return its canonical path.
///
/// Uses header-only reads via `imagesize` to reject non-images.
pub fn extract(path: &Path) -> Result<ImageMetadata> {
    let canonical_path = path.canonicalize().context("Failed to resolve path")?;
    imagesize::size(&canonical_path).context("Not a valid image file")?;
    Ok(ImageMetadata {
        path: canonical_path,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_nonexistent_file() {
        let result = extract(Path::new("/nonexistent/image.png"));
        assert!(result.is_err());
    }
}

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct ImageMetadata {
    pub path: PathBuf,
    pub file_name: String,
    pub size_bytes: u64,
    pub size_display: String,
    pub width: u32,
    pub height: u32,
    pub format: Option<String>,
}

/// Extract metadata from a single image file.
///
/// Reads file headers for dimensions.
/// Dimension/format failures degrade to None rather than failing the whole file.
pub fn extract(path: &Path) -> Result<ImageMetadata> {
    let canonical_path = path.canonicalize().context("Failed to resolve path")?;

    let fs_meta = fs::metadata(&canonical_path)
        .with_context(|| format!("Failed to read file metadata: {}", canonical_path.display()))?;

    let file_name = canonical_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| "unknown".into());

    let (width, height, format) =
        read_image_info(&canonical_path).context("Not a valid image file")?;

    Ok(ImageMetadata {
        path: canonical_path,
        file_name,
        size_bytes: fs_meta.len(),
        size_display: format_size(fs_meta.len()),
        width,
        height,
        format
    })
}

/// Read image dimensions and format from file headers.
/// Returns Err if the file is not a valid image.
fn read_image_info(path: &Path) -> Result<(u32, u32, Option<String>)> {
    let reader = image::ImageReader::open(path).context("Failed to open image")?;
    let reader = reader.with_guessed_format().context("Failed to detect format")?;
    let format = reader.format().map(|f| format!("{:?}", f));
    let (w, h) = reader.into_dimensions().context("Failed to read image dimensions")?;
    Ok((w, h, format))
}

pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;

    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn format_size_kb() {
        assert_eq!(format_size(1024), "1.0 KB");
        assert_eq!(format_size(1536), "1.5 KB");
    }

    #[test]
    fn format_size_mb() {
        assert_eq!(format_size(1_048_576), "1.0 MB");
        assert_eq!(format_size(1_572_864), "1.5 MB");
    }

    #[test]
    fn format_size_gb() {
        assert_eq!(format_size(1_073_741_824), "1.0 GB");
        assert_eq!(format_size(1_610_612_736), "1.5 GB");
    }

    #[test]
    fn extract_nonexistent_file() {
        let result = extract(Path::new("/nonexistent/image.png"));
        assert!(result.is_err());
    }
}

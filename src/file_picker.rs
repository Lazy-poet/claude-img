use std::path::{Path, PathBuf};
use rfd::FileDialog;
use std::fmt;
pub const ALLOWED_IMG_EXTENSIONS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "webp", "bmp", "tiff", "tif",
];


#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    FileNotFound,
    NotAFile,
    UnsupportedFmt,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(f, "File not found"),
            Self::NotAFile => write!(f, "Not a file"),
            Self::UnsupportedFmt => write!(f, "Unsupported file format"),
        }
    }
}


/// Opens the user's native file picker dialog for selecting images.
pub fn pick_images(directory: Option<&Path>) -> Vec<PathBuf> {
    let mut dialog = FileDialog::new()
        .set_title("Select context images for Claude")
        .add_filter("Images", ALLOWED_IMG_EXTENSIONS);

    if let Some(dir) = directory {
        dialog = dialog.set_directory(dir);
    }

    dialog.pick_files().unwrap_or_default()
}

/// Validates a list of paths that they exist and have a valid extension & returns a tuple (valid, errors).
pub fn validate_paths(paths: &[PathBuf]) -> (Vec<&PathBuf>, Vec<(&PathBuf, ValidationError)>) {
    let mut valid = Vec::new();
    let mut errors = Vec::new();

    for path in paths {
        if !path.exists() {
            errors.push((path, ValidationError::FileNotFound));
            continue;
        }
        if !path.is_file() {
            errors.push((path, ValidationError::NotAFile));
            continue;
        }
        let file_ext_valid = path
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|ext| {
                ALLOWED_IMG_EXTENSIONS.contains(&ext)

            });
        if !file_ext_valid {
            errors.push((path, ValidationError::UnsupportedFmt));
            continue;
        }
        valid.push(path);
    }

    (valid, errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn validate_nonexistent_file() {
        let paths = vec![PathBuf::from("/nonexistent/agghvsimg.png")];
        let (valid, errors) = validate_paths(&paths);
        assert!(valid.is_empty());
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].1, ValidationError::FileNotFound);
    }

    #[test]
    fn validate_directory_rejected() {
        let dir = std::env::temp_dir();
        let paths = vec![dir.clone()];
        let (valid, errors) = validate_paths(&paths);
        assert!(valid.is_empty());
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn validate_unsupported_extension() {
        let tmp = std::env::temp_dir().join("test_validate.txt");
        fs::write(&tmp, "i'm a text file pls").unwrap();
        let paths = vec![tmp.clone()];
        let (valid, errors) = validate_paths(&paths);
        assert!(valid.is_empty());
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].1, ValidationError::UnsupportedFmt);
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn validate_valid_image_extension() {
        let tmp = std::env::temp_dir().join("test_validate.png");
        fs::write(&tmp, "cosplaying png").unwrap();
        let paths = vec![tmp.clone()];
        let (valid, errors) = validate_paths(&paths);
        assert_eq!(valid.len(), 1);
        assert!(errors.is_empty());
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn validate_case_insensitive_extension() {
        let tmp = std::env::temp_dir().join("test_validate.PNG");
        fs::write(&tmp, "cosplaying png").unwrap();
        let paths = vec![tmp.clone()];
        let (valid, errors) = validate_paths(&paths);
        assert_eq!(valid.len(), 1);
        assert!(errors.is_empty());
        let _ = fs::remove_file(&tmp);
    }

    #[test]
    fn validate_mixed_paths() {
        let good = std::env::temp_dir().join("test_mixed_good.jpg");
        let bad = PathBuf::from("/nonexistent/bad.png");
        fs::write(&good, "fake jpg").unwrap();
        let paths = vec![good.clone(), bad];
        let (valid, errors) = validate_paths(&paths);
        assert_eq!(valid.len(), 1);
        assert_eq!(errors.len(), 1);
        let _ = fs::remove_file(&good);
    }
}

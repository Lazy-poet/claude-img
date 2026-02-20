use crate::metadata::ImageMetadata;
use anyhow::Result;
use std::io::{self, Write};

/// Write @path references to stdout for Claude Code to capture.
pub fn write_output(images: &[ImageMetadata]) -> Result<()> {
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for img in images {
        writeln!(out, "@{}", img.path.display())?;
    }
    Ok(())
}
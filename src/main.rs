mod file_picker;
mod metadata;

use anyhow::{bail, Result};
use std::io::{self, Write};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let selected_files = file_picker::pick_images();
    if selected_files.is_empty() {
        eprintln!("No files selected.");
        return Ok(());
    }

    let (valid, errors) = file_picker::validate_paths(&selected_files);

    if !errors.is_empty() {
        for (path, reason) in &errors {
            eprintln!("Skipped: {} ({})", path.display(), reason);
        }
    }

    if valid.is_empty() {
        bail!("No valid image files to process");
    }
    // temporary logging
    let stdout = io::stdout();
    let mut out = stdout.lock();
    for path in &valid {
        let meta = metadata::extract(&path)?;
        writeln!(out, "{:?}", meta)?;
    }

    Ok(())
}

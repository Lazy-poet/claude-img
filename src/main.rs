mod file_picker;
mod metadata;
mod writer;

use std::io::{self, Write};
use anyhow::Result;
use writer::write_output;

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
        let stderr = io::stderr();
        let mut err = stderr.lock();
        for (path, reason) in &errors {
            let _ = writeln!(err, "Skipped: {} ({})", path.display(), reason);
        }
    }

    if valid.is_empty() {
        eprintln!("No valid image files to process.");
        return Ok(());
    }

    let mut images = Vec::new();
    for path in &valid {
        match metadata::extract(path) {
            Ok(meta) => images.push(meta),
            Err(_) => {
                let name = path.file_name().map(|n| n.to_string_lossy()).unwrap_or_default();
                eprintln!("Skipped: {} (not a valid image)", name);
            }
        }
    }

    if images.is_empty() {
        eprintln!("No valid images selected.");
        return Ok(());
    }

    write_output(&images)?;

    Ok(())
}

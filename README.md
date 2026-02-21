# claude-img

Native file picker for uploading image context to Claude Code conversations.

Select one or multiple images from disk with a single `/img` command.

## Install

**macOS / Linux:**
```bash
curl -fsSL https://claude-img-releases.s3.eu-north-1.amazonaws.com/install.sh | bash
```

**Windows (PowerShell):**
```powershell
irm https://claude-img-releases.s3.eu-north-1.amazonaws.com/install.ps1 | iex
```

No dependencies. Downloads a pre-built binary and sets up the `/img` skill for Claude Code.

**Supports:** macOS (Apple Silicon & Intel), Linux (x86_64), Windows (x86_64)

## Usage

In any Claude Code session:

```
/img review these mockups for accessibility
/img what's different between these screenshots?
/img
```

A native file picker opens. Select one or more images (PNG, JPG, GIF, WebP, BMP, TIFF). The images are injected into the conversation. Everything after `/img` is added as additional context to the images.

## Why

Claude Code's `Ctrl+V` only allows pasting a single image from your clipboard, and it's stressful to manually type paths to images. `claude-img` lets you select multiple easily.

## How it works

1. `/img` triggers the Claude Code skill
2. `claude-img` opens your OS native file picker via [rfd](https://crates.io/crates/rfd)
3. Selected files are validated as images
4. `@/path/to/image.png` references go to stdout
5. Claude Code captures them and injects the images into context

## Building from source

```bash
cargo install --git https://github.com/lazy-poet/claude-img
```

## Uninstall

**macOS / Linux:**
```bash
rm ~/.local/bin/claude-img
rm -rf ~/.claude/skills/img
```

**Windows (PowerShell):**
```powershell
Remove-Item "$env:LOCALAPPDATA\Programs\claude-img" -Recurse
Remove-Item "$env:USERPROFILE\.claude\skills\img" -Recurse
```

## License

MIT

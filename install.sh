#!/bin/bash
set -euo pipefail

REPO="lazy-poet/claude-img"
SKILL_DIR="$HOME/.claude/skills/img"

# --- Safety checks ---
if [ "$(id -u)" -eq 0 ]; then
    echo "Error: Do not run this script as root." >&2
    exit 1
fi

if [ -z "${HOME:-}" ] || [ ! -d "$HOME" ]; then
    echo "Error: \$HOME is not set or does not exist." >&2
    exit 1
fi

trap 'echo "Installation failed." >&2' ERR

# --- Detect platform ---
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS-$ARCH" in
    Darwin-arm64)  BINARY="claude-img-macos-arm64" ;;
    Darwin-x86_64) BINARY="claude-img-macos-x86_64" ;;
    Linux-x86_64)  BINARY="claude-img-linux-x86_64" ;;
    *)
        echo "Error: Unsupported platform $OS-$ARCH" >&2
        echo "Build from source: cargo install --git https://github.com/$REPO" >&2
        exit 1
        ;;
esac

# --- Install directory ---
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

# --- Download binary ---
echo "Installing claude-img for $OS ($ARCH)..."
LATEST_URL="https://github.com/$REPO/releases/latest/download/$BINARY"
curl --proto '=https' --tlsv1.2 -fsSL "$LATEST_URL" -o "$INSTALL_DIR/claude-img"
chmod +x "$INSTALL_DIR/claude-img"

# --- Install Claude Code skill ---
echo "Installing Claude Code skill..."
mkdir -p "$SKILL_DIR"
if [ -f "$SKILL_DIR/SKILL.md" ]; then
    echo "Existing /img skill found. Backing up to $SKILL_DIR/SKILL.md.bak"
    cp "$SKILL_DIR/SKILL.md" "$SKILL_DIR/SKILL.md.bak"
fi
cat > "$SKILL_DIR/SKILL.md" << 'SKILL_EOF'
---
name: img
description: Upload images to this conversation through your native file picker
allowed-tools: Bash(claude-img)
---

Run the file picker and attach the selected images:

!`claude-img`

If images were attached above (lines starting with @), analyze them. If no images were attached or the output shows "Skipped" or "No valid images", let the user know and suggest they try again with valid image files. $ARGUMENTS
SKILL_EOF

# --- Check PATH ---
if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
    echo ""
    echo "Note: Add $INSTALL_DIR to your PATH:"
    case "$SHELL" in
        */zsh)  echo "  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.zshrc" ;;
        */bash) echo "  echo 'export PATH=\"$INSTALL_DIR:\$PATH\"' >> ~/.bashrc" ;;
        */fish) echo "  fish_add_path $INSTALL_DIR" ;;
        *)      echo "  export PATH=\"$INSTALL_DIR:\$PATH\"" ;;
    esac
fi

echo ""
echo "Done! Type /img in Claude Code to open the image picker."

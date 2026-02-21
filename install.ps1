$ErrorActionPreference = "Stop"

$Repo = "lazy-poet/claude-img"
$InstallDir = "$env:LOCALAPPDATA\Programs\claude-img"
$SkillDir = "$env:USERPROFILE\.claude\skills\img"
$Binary = "claude-img-windows-x86_64.exe"

Write-Host "Installing claude-img..."

# Download binary
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$S3Base = "https://claude-img-releases.s3.eu-north-1.amazonaws.com"
Invoke-WebRequest -Uri "$S3Base/latest/$Binary" -OutFile "$InstallDir\claude-img.exe"

# Install skill
New-Item -ItemType Directory -Force -Path $SkillDir | Out-Null
if (Test-Path "$SkillDir\SKILL.md") {
    Write-Host "Existing /img skill found. Backing up to SKILL.md.bak"
    Copy-Item "$SkillDir\SKILL.md" "$SkillDir\SKILL.md.bak"
}
$SkillContent = @'
---
name: img
description: Upload images to this conversation through your native file picker
allowed-tools: Bash(claude-img)
---

Run the file picker and attach the selected images:

!`claude-img`

If images were attached above (lines starting with @), analyze them. If no images were attached or the output shows "Skipped" or "No valid images", let the user know and suggest they try again with valid image files. $ARGUMENTS
'@
Set-Content -Path "$SkillDir\SKILL.md" -Value $SkillContent -Encoding UTF8

# Add to PATH if needed
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to PATH. Restart your terminal for it to take effect."
}

Write-Host ""
Write-Host "Done! Type /img in Claude Code to open the image picker."

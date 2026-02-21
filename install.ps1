$ErrorActionPreference = "Stop"

$Repo = "lazy-poet/claude-img"
$InstallDir = "$env:LOCALAPPDATA\Programs\claude-img"
$SkillDir = "$env:USERPROFILE\.claude\skills\img"
$Binary = "claude-img-windows-x86_64.exe"

Write-Host "Installing claude-img..."

# Download binary
New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
$Url = "https://github.com/$Repo/releases/latest/download/$Binary"
Invoke-WebRequest -Uri $Url -OutFile "$InstallDir\claude-img.exe"

# Install skill
New-Item -ItemType Directory -Force -Path $SkillDir | Out-Null
if (Test-Path "$SkillDir\SKILL.md") {
    Write-Host "Existing /img skill found. Backing up to SKILL.md.bak"
    Copy-Item "$SkillDir\SKILL.md" "$SkillDir\SKILL.md.bak"
}
$SkillUrl = "https://raw.githubusercontent.com/$Repo/main/skill/SKILL.md"
Invoke-WebRequest -Uri $SkillUrl -OutFile "$SkillDir\SKILL.md"

# Add to PATH if needed
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Added $InstallDir to PATH. Restart your terminal for it to take effect."
}

Write-Host ""
Write-Host "Done! Type /img in Claude Code to open the image picker."

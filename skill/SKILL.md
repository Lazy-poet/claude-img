---
name: img
description: Upload images to this conversation through your native file picker
allowed-tools: Bash(claude-img)
---

Run the file picker and attach the selected images:

!`claude-img`

If images were attached above (lines starting with @), analyze them. If no images were attached or the output shows "Skipped" or "No valid images", let the user know and suggest they try again with valid image files. $ARGUMENTS

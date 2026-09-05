# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
[CmdletBinding()]
param(
    [ValidateSet('Smoke', 'Matrix', 'All')][string]$Mode = 'Smoke',
    [string]$OutputDirectory,
    [ValidateRange(0, 7)][int]$Page = 0,
    [ValidateRange(0, 2)][int]$Preview = 1,
    [switch]$ReuseExisting
)
$ErrorActionPreference = 'Stop'
$captureArgs = @((Join-Path $PSScriptRoot 'capture_gallery_baseline.py'), '--mode', $Mode, '--page', [string]$Page, '--preview', [string]$Preview)
if ($OutputDirectory) { $captureArgs += @('--output-directory', $OutputDirectory) }
if ($ReuseExisting) { $captureArgs += '--reuse-existing' }
& python @captureArgs
if ($LASTEXITCODE -ne 0) { throw "Gallery capture failed with exit code $LASTEXITCODE" }

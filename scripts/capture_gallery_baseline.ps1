# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
[CmdletBinding()]
param(
    [ValidateSet('Smoke', 'Matrix', 'All', 'Navigation', 'Catalog')][string]$Mode = 'Smoke',
    [string]$OutputDirectory,
    [ValidateRange(0, 7)][int]$Page,
    [string]$Destination,
    [ValidateRange(0, 2)][int]$Preview = 1,
    [switch]$ReuseExisting
)
$ErrorActionPreference = 'Stop'
if ($PSBoundParameters.ContainsKey('Page') -and $PSBoundParameters.ContainsKey('Destination')) { throw 'Page and Destination cannot be supplied together' }
$captureArgs = @((Join-Path $PSScriptRoot 'capture_gallery_baseline.py'), '--mode', $Mode, '--preview', [string]$Preview)
if ($PSBoundParameters.ContainsKey('Page')) { $captureArgs += @('--page', [string]$Page) }
if ($PSBoundParameters.ContainsKey('Destination')) { $captureArgs += @('--destination', $Destination) }
if ($OutputDirectory) { $captureArgs += @('--output-directory', $OutputDirectory) }
if ($ReuseExisting) { $captureArgs += '--reuse-existing' }
& python @captureArgs
if ($LASTEXITCODE -ne 0) { throw "Gallery capture failed with exit code $LASTEXITCODE" }

# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
# Inspect only a newly launched verification host, never another application.
param(
    [Parameter(Mandatory=$true)][string]$Executable,
    [Parameter(Mandatory=$true)][string]$OutputDirectory
)
$ErrorActionPreference = 'Stop'
Add-Type -AssemblyName UIAutomationClient
Add-Type -AssemblyName UIAutomationTypes
$binary = (Resolve-Path -LiteralPath $Executable).Path
$outputPath = [System.IO.Path]::GetFullPath($OutputDirectory)
[System.IO.Directory]::CreateDirectory($outputPath) | Out-Null
$env:SLINT_BACKEND = 'winit-software'
$env:SLINT_SCALE_FACTOR = '1'
$probeProcess = $null
$report = @{ status = 'IN_PROGRESS'; scope = 'Actual Windows UIA tree and native Invoke; no screen-reader claim'; elements = @() }
try {
    $probeProcess = Start-Process -FilePath $binary -PassThru -WindowStyle Hidden
    $condition = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $probeProcess.Id)
    $window = $null
    for ($attempt=0; $attempt -lt 100 -and $null -eq $window; $attempt++) {
        if ($probeProcess.HasExited) { throw 'Verification host exited before UIA attachment' }
        Start-Sleep -Milliseconds 100
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Children, $condition)
    }
    if ($null -eq $window) { throw 'Own process window not found' }
    # AccessKit may publish the semantic tree after UIA first requests it.
    $buttons = @()
    for ($attempt=0; $attempt -lt 30 -and $buttons.Count -eq 0; $attempt++) {
        $elements = $window.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
        $rows = @()
        $buttons = @()
        foreach ($element in $elements) {
            $current = $element.Current
            $rows += @{ name=$current.Name; type=$current.ControlType.ProgrammaticName; enabled=$current.IsEnabled; focusable=$current.IsKeyboardFocusable }
            if ($current.ControlType -eq [System.Windows.Automation.ControlType]::Button -and $current.IsEnabled) { $buttons += $element }
        }
        if ($buttons.Count -eq 0) { Start-Sleep -Milliseconds 100 }
    }
    $report.elements = $rows
    if ($buttons.Count -eq 0) { throw 'No enabled native Button exposed through Windows UIA' }
    $named = @($buttons | Where-Object { $_.Current.Name.Length -gt 0 })
    if ($named.Count -eq 0) { throw 'Native buttons have no accessible names' }
    $kitButtons = @($named | Where-Object { $_.Current.Name -eq 'Kit command' })
    if ($kitButtons.Count -ne 1) { throw 'Expected exactly one current Kit command input owner in UIA' }
    $button = $kitButtons[0]
    $report.invoked_name = $button.Current.Name
    $button.SetFocus()
    Start-Sleep -Milliseconds 150
    $report.focus_observed = $button.Current.HasKeyboardFocus
    if (-not $report.focus_observed) { throw 'UIA focus was not observed on the native Kit action' }
    $pattern = $null
    if (-not $button.TryGetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern, [ref]$pattern)) { throw 'Native Button has no InvokePattern' }
    $pattern.Invoke()
    $report.invoke_returned = $true
    $counterText = ''
    for ($attempt=0; $attempt -lt 30 -and $counterText -notmatch 'Kit actions: 1\b'; $attempt++) {
        Start-Sleep -Milliseconds 100
        $currentElements = $window.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
        foreach ($element in $currentElements) {
            if ($element.Current.Name -like 'Native actions:*') { $counterText = $element.Current.Name }
        }
    }
    if ($counterText -notmatch 'Kit actions: 1\b') { throw 'UIA Invoke did not produce exactly one observed Kit command' }
    $report.command_counter = $counterText
    $checkbox = $null
    foreach ($element in $currentElements) {
        if ($element.Current.Name -eq 'Buttons enabled' -and $element.Current.ControlType -eq [System.Windows.Automation.ControlType]::CheckBox) { $checkbox = $element }
    }
    if ($null -eq $checkbox) { throw 'Enabled-state fixture missing from UIA' }
    $toggle = $checkbox.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    $toggle.Toggle()
    Start-Sleep -Milliseconds 200
    $report.disabled_observed = -not $button.Current.IsEnabled
    if (-not $report.disabled_observed) { throw 'Disabled state did not reach UIA' }
    $report.status = 'PASS'
} catch {
    $report.status = 'FAIL'
    $report.error = $_.Exception.Message
} finally {
    if ($null -ne $probeProcess -and -not $probeProcess.HasExited) { Stop-Process -Id $probeProcess.Id }
    $report | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $outputPath 'result.json') -Encoding UTF8
}
$report | ConvertTo-Json -Depth 8
if ($report.status -ne 'PASS') { exit 1 }

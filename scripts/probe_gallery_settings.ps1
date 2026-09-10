# SPDX-FileCopyrightText: Copyright (c) 2026 Quadrant contributors
# SPDX-License-Identifier: GPL-3.0-only
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
$savedEnvironment = @{}
Get-ChildItem Env: | Where-Object { $_.Name -like 'QUADRANT_GALLERY_*' -or $_.Name -like 'SLINT_*' } | ForEach-Object {
    $savedEnvironment[$_.Name] = $_.Value
    [Environment]::SetEnvironmentVariable($_.Name, $null, 'Process')
}
$env:QUADRANT_GALLERY_DESTINATION = 'home'
$env:QUADRANT_GALLERY_THEME = 'light'
$env:SLINT_BACKEND = 'winit-software'
$env:SLINT_SCALE_FACTOR = '1'

Add-Type -AssemblyName System.Drawing
Add-Type -AssemblyName System.Windows.Forms
Add-Type @'
using System;
using System.Runtime.InteropServices;
public static class CaptionGeometry {
 [StructLayout(LayoutKind.Sequential)] public struct RECT { public int Left,Top,Right,Bottom; }
 [StructLayout(LayoutKind.Sequential)] public struct INFO {
   public uint Size; public RECT Title;
   [MarshalAs(UnmanagedType.ByValArray,SizeConst=6)] public uint[] State;
   [MarshalAs(UnmanagedType.ByValArray,SizeConst=6)] public RECT[] Rects;
 }
 [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h,out RECT r);
 [DllImport("user32.dll")] public static extern bool PrintWindow(IntPtr h,IntPtr dc,uint flags);
 [DllImport("user32.dll")] public static extern IntPtr SendMessageW(IntPtr h,uint m,IntPtr w,IntPtr l);
 [DllImport("user32.dll",EntryPoint="SendMessageW")] public static extern IntPtr QueryTitle(IntPtr h,uint m,IntPtr w,ref INFO l);
 [DllImport("user32.dll")] public static extern bool IsZoomed(IntPtr h);
 [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
}
'@
function Capture-OwnWindow($name) {
 $r=New-Object CaptionGeometry+RECT
 if (-not [CaptionGeometry]::GetWindowRect($handle,[ref]$r)) { throw 'Window rectangle unavailable' }
 $bmp=New-Object System.Drawing.Bitmap(($r.Right-$r.Left),($r.Bottom-$r.Top))
 $g=[System.Drawing.Graphics]::FromImage($bmp);$dc=$g.GetHdc()
 try { $ok=[CaptionGeometry]::PrintWindow($handle,$dc,2) } finally { $g.ReleaseHdc($dc);$g.Dispose() }
 try { if (-not $ok) {throw 'PrintWindow failed'}; $bmp.Save((Join-Path $outputPath ($name+'.png')),[System.Drawing.Imaging.ImageFormat]::Png) } finally {$bmp.Dispose()}
}
function Check-Chrome($name) {
 $info=New-Object CaptionGeometry+INFO
 $info.Size=[Runtime.InteropServices.Marshal]::SizeOf($info)
 $info.State=New-Object uint32[] 6;$info.Rects=New-Object CaptionGeometry+RECT[] 6
 $null=[CaptionGeometry]::QueryTitle($handle,0x033F,[IntPtr]::Zero,[ref]$info)
 $backControl=Find-Control 'Back' 'ControlType.Button'
 $backRect=$backControl.Current.BoundingRectangle
 $row=@{state=$name;back=$backRect.ToString();native_buttons=@()}
 foreach($index in @(2,3,5)) {
   $r=$info.Rects[$index];$center=($r.Top+$r.Bottom)/2.0
   $delta=[Math]::Abs($center-($backRect.Top+$backRect.Height/2.0))
   $row.native_buttons+=@{index=$index;left=$r.Left;right=$r.Right;top=$r.Top;bottom=$r.Bottom;width=($r.Right-$r.Left);height=($r.Bottom-$r.Top);center_delta=$delta}
   if ($r.Bottom -le $r.Top -or $delta -ne 0 -or [Math]::Abs($backRect.Height - ($r.Bottom-$r.Top)) -gt 0) {throw "Caption vertical geometry differs: $delta"}
   if ($index -eq 2 -and $backRect.Width -ne ($r.Right-$r.Left)) {throw "Caption Back width differs from native minimize"}
   $cx=[int](($r.Left+$r.Right)/2);$cy=[int]$center
   $nativePoint=($cx -band 65535) -bor (($cy -band 65535) -shl 16)
   $nativeHit=[CaptionGeometry]::SendMessageW($handle,0x0084,[IntPtr]::Zero,[IntPtr]$nativePoint).ToInt32()
   $expectedHit=@{2=8;3=9;5=20}[$index]
   if($nativeHit -ne $expectedHit) {throw "Caption button hit mismatch: index $index, hit $nativeHit"}
 }
 $win=New-Object CaptionGeometry+RECT;$null=[CaptionGeometry]::GetWindowRect($handle,[ref]$win)
 foreach($case in @(@(200,16,2),@(200,40,1),@(24,16,1))) {
   $sx=$win.Left+$case[0];$sy=$win.Top+$case[1];$packed=($sx -band 65535) -bor (($sy -band 65535) -shl 16)
   $hit=[CaptionGeometry]::SendMessageW($handle,0x0084,[IntPtr]::Zero,[IntPtr]$packed).ToInt32()
   if($hit -ne $case[2]) {throw "Wrong caption/client hit at $($case[0]),$($case[1]): $hit"}
 }
 $report.geometry+=,$row
 Capture-OwnWindow $name
}

$probeProcess = $null
$report = @{ status='IN_PROGRESS'; scope='Own Gallery Windows UIA route/focus/control integration; no reader claim'; checks=@() }
$report.executable = $binary
$report.binary_sha256 = (Get-FileHash -LiteralPath $binary -Algorithm SHA256).Hash.ToLowerInvariant()
function Find-Control($Name, $Type) {
    $foundControls = @()
    for ($attempt=0; $attempt -lt 40; $attempt++) {
        $elements = $window.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
        $foundControls = @($elements | Where-Object { $_.Current.Name -eq $Name -and $_.Current.ControlType.ProgrammaticName -eq $Type })
        if ($foundControls.Count -gt 0) { break }
        Start-Sleep -Milliseconds 100
    }
    if ($foundControls.Count -ne 1) { throw "Expected one $Type '$Name', got $($foundControls.Count)" }
    return $foundControls[0]
}
function Invoke-Control($Control) {
    $pattern = $null
    if ($Control.TryGetCurrentPattern([System.Windows.Automation.InvokePattern]::Pattern, [ref]$pattern)) {
        $pattern.Invoke()
    } elseif ($Control.TryGetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern, [ref]$pattern)) {
        # Slint exposes a checked button through TogglePattern.
        $report.checks += "Checked navigation action uses UIA TogglePattern: $($Control.Current.Name)"
        $pattern.Toggle()
    } else { throw "No supported UIA action for $($Control.Current.Name)" }
    Start-Sleep -Milliseconds 200
}
try {
    $probeProcess = Start-Process -FilePath $binary -PassThru -WindowStyle Hidden
    $processCondition = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::ProcessIdProperty, $probeProcess.Id)
    $titleCondition = New-Object System.Windows.Automation.PropertyCondition([System.Windows.Automation.AutomationElement]::NameProperty, 'Quadrant Kit Gallery')
    $condition = New-Object System.Windows.Automation.AndCondition($processCondition, $titleCondition)
    $window = $null
    for ($attempt=0; $attempt -lt 100 -and $null -eq $window; $attempt++) {
        if ($probeProcess.HasExited) { throw 'Gallery exited before UIA attachment' }
        Start-Sleep -Milliseconds 100
        $window = [System.Windows.Automation.AutomationElement]::RootElement.FindFirst([System.Windows.Automation.TreeScope]::Children, $condition)
    }
    if ($null -eq $window) { throw 'Own Gallery window not found' }
    $handle=[IntPtr]$window.Current.NativeWindowHandle
    $report.geometry=@()
    Check-Chrome 'restored'
    $null=[CaptionGeometry]::SendMessageW($handle,0x0112,[IntPtr]0xF030,[IntPtr]::Zero)
    Start-Sleep -Milliseconds 500
    if(-not [CaptionGeometry]::IsZoomed($handle)) {throw 'Native maximize failed'}
    Check-Chrome 'maximized'
    $null=[CaptionGeometry]::SendMessageW($handle,0x0112,[IntPtr]0xF120,[IntPtr]::Zero)
    Start-Sleep -Milliseconds 500
    if([CaptionGeometry]::IsZoomed($handle)) {throw 'Native restore failed'}
    $report.checks+='Native restored/maximized alignment and caption/client hit testing'
    $brand = Find-Control 'Kit Gallery' 'ControlType.Text'
    $backAction = Find-Control 'Back' 'ControlType.Button'
    $paneAction = Find-Control 'Toggle navigation pane' 'ControlType.Button'
    $backBounds = $backAction.Current.BoundingRectangle
    $paneBounds = $paneAction.Current.BoundingRectangle
    $brandBounds = $brand.Current.BoundingRectangle
    if ($paneBounds.Top -lt $backBounds.Bottom -or $paneBounds.Right -gt $brandBounds.Left -or [Math]::Abs(($paneBounds.Top + $paneBounds.Height / 2) - ($brandBounds.Top + $brandBounds.Height / 2)) -gt 1) { throw 'Navigation toggle must sit left of the name on the same navigation header row' }
    if ($brandBounds.Top -lt $backBounds.Bottom -or $brandBounds.Left -gt $window.Current.BoundingRectangle.Left + 304) { throw 'Gallery name is not in the navigation header' }
    $report.checks += 'Navigation toggle and Gallery name share the header below caption Back'
    $report.back_bounds = $backBounds.ToString()
    $report.pane_toggle_bounds = $paneBounds.ToString()
    $report.navigation_title_bounds = $brandBounds.ToString()
    $settings = Find-Control 'Settings' 'ControlType.Button'
    $settings.SetFocus()
    Start-Sleep -Milliseconds 150
    if (-not $settings.Current.HasKeyboardFocus) { throw 'Settings navigation focus not observed' }
    $report.checks += 'Unique Settings target exposes keyboard focus'
    Invoke-Control $settings
    $selectedPattern = $settings.GetCurrentPattern([System.Windows.Automation.TogglePattern]::Pattern)
    if ($selectedPattern.Current.ToggleState -ne [System.Windows.Automation.ToggleState]::On) { throw 'Settings selection is not exposed to UIA' }
    $report.checks += 'Host-controlled Settings selection is exposed through UIA'
    $theme = Find-Control 'App theme' 'ControlType.ComboBox'
    $preview = Find-Control 'Preview width' 'ControlType.ComboBox'
    if (-not $theme.Current.IsEnabled -or -not $preview.Current.IsEnabled) { throw 'Settings controls disabled' }
    $report.checks += 'Settings activation opens both named enabled native ComboBoxes'
    $report.settings_bounds = $settings.Current.BoundingRectangle.ToString()
    $report.theme_bounds = $theme.Current.BoundingRectangle.ToString()
    $report.preview_bounds = $preview.Current.BoundingRectangle.ToString()
    Capture-OwnWindow 'settings'
    $homeAction = Find-Control 'Home' 'ControlType.Button'
    Invoke-Control $homeAction
    $null = Find-Control 'Browse all components' 'ControlType.Button'
    $report.checks += 'Home activation returns to the actual Home page'
    $back = Find-Control 'Back' 'ControlType.Button'
    Invoke-Control $back
    $null = Find-Control 'App theme' 'ControlType.ComboBox'
    $report.checks += 'Caption Back returns to Settings through Gallery history'
    $elements = $window.FindAll([System.Windows.Automation.TreeScope]::Descendants, [System.Windows.Automation.Condition]::TrueCondition)
    $report.elements = @($elements | ForEach-Object { @{name=$_.Current.Name; type=$_.Current.ControlType.ProgrammaticName; enabled=$_.Current.IsEnabled} })
    $oldToolbar = @($elements | Where-Object { $_.Current.ControlType -eq [System.Windows.Automation.ControlType]::Button -and $_.Current.Name -cin @('System','Light','Dark','C','M','W') })
    if ($oldToolbar.Count -ne 0) { throw 'Old theme/preview toolbar buttons remain exposed' }
    $report.checks += 'Old theme and C/M/W toolbar buttons are absent'
    $toggleAction=Find-Control 'Toggle navigation pane' 'ControlType.Button'
    $toggleAction.SetFocus()
    if([CaptionGeometry]::GetForegroundWindow() -ne $handle) {throw 'Keyboard target is not the owned foreground window'}
    [System.Windows.Forms.SendKeys]::SendWait(' ')
    Start-Sleep -Milliseconds 200
    $currentElements=$window.FindAll([System.Windows.Automation.TreeScope]::Descendants,[System.Windows.Automation.Condition]::TrueCondition)
    $titles=@($currentElements | Where-Object {$_.Current.Name -eq 'Kit Gallery'})
    if($titles.Count -ne 0) {throw 'Compact navigation retains the expanded name'}
    $toggles=@($currentElements | Where-Object {$_.Current.Name -eq 'Toggle navigation pane'})
    $extra=@($currentElements | Where-Object {$_.Current.Name -eq 'Expand navigation to search' -or $_.Current.Name -match '^(Expand|Collapse) '})
    if($toggles.Count -ne 1 -or $extra.Count -ne 0) {throw 'Compact rail exposes duplicate toggle or separate chevrons'}
    $toggleRect=$toggleAction.Current.BoundingRectangle
    $railCenter=$toggleRect.Left+$toggleRect.Width/2
    $railActions=@($currentElements | Where-Object {$_.Current.ControlType -eq [System.Windows.Automation.ControlType]::Button -and $_.Current.BoundingRectangle.Top -ge $toggleRect.Bottom -and $_.Current.BoundingRectangle.Left -lt $railCenter})
    if($railActions.Count -lt 3) {throw 'Compact primary/footer targets unavailable'}
    foreach($action in $railActions) {
        $bounds=$action.Current.BoundingRectangle
        if([Math]::Abs(($bounds.Left+$bounds.Width/2)-$railCenter) -gt 1) {throw "Off-center compact target: $($action.Current.Name)"}
    }
    $editors=@($currentElements | Where-Object {$_.Current.ControlType -eq [System.Windows.Automation.ControlType]::Edit -and $_.Current.BoundingRectangle.Left -lt $railCenter+27})
    if($editors.Count -ne 0) {throw 'Compact navigation retains a search editor'}
    $report.checks+='Compact has one toggle, no search/chevrons, and centered primary/footer action bounds'
    Capture-OwnWindow 'compact'
    $toggleAction.SetFocus()
    if([CaptionGeometry]::GetForegroundWindow() -ne $handle) {throw 'Keyboard target is not the owned foreground window'}
    [System.Windows.Forms.SendKeys]::SendWait(' ')
    Start-Sleep -Milliseconds 200
    $null=Find-Control 'Kit Gallery' 'ControlType.Text'
    $null=Find-Control 'Collapse Design guidance' 'ControlType.Button'
    $report.checks+='Compact hides navigation title; expanding restores it'
    $report.status = 'PASS' 
} catch {
    $report.status = 'FAIL'
    $report.error = $_.Exception.Message
} finally {
    if ($null -ne $probeProcess -and -not $probeProcess.HasExited) { Stop-Process -Id $probeProcess.Id }
    Get-ChildItem Env: | Where-Object { $_.Name -like 'QUADRANT_GALLERY_*' -or $_.Name -like 'SLINT_*' } | ForEach-Object { [Environment]::SetEnvironmentVariable($_.Name, $null, 'Process') }
    foreach ($name in $savedEnvironment.Keys) { [Environment]::SetEnvironmentVariable($name, $savedEnvironment[$name], 'Process') }
    $report | ConvertTo-Json -Depth 8 | Set-Content -LiteralPath (Join-Path $outputPath 'result.json') -Encoding UTF8
}
$report | ConvertTo-Json -Depth 8
if ($report.status -ne 'PASS') { exit 1 }

# PySide6 and shiboken6 for the bundled Python, the release built for the Qt the app runs. PySide6 requires its exact
# shiboken6, so both are installed here, together, at the Qt version.
#   tools\python\fetch-pyside.ps1 -Python <python.exe> -Version <qt-version> -Dest <site-packages>
param(
    [Parameter(Mandatory = $true)][string]$Python,
    [Parameter(Mandatory = $true)][string]$Version,
    [Parameter(Mandatory = $true)][string]$Dest
)

function Have-Pyside {
    $p = Join-Path $Dest "PySide6\_config.py"
    $s = Join-Path $Dest "shiboken6\_config.py"
    if (-not (Test-Path $p) -or -not (Test-Path $s)) { return $false }
    return (Select-String -Path $p -Pattern "^version = `"$Version`"" -Quiet) -and
           (Select-String -Path $s -Pattern "^version = `"$Version`"" -Quiet)
}
if (Have-Pyside) { exit 0 }

New-Item -ItemType Directory -Force $Dest | Out-Null
foreach ($name in "PySide6", "shiboken6") {
    Remove-Item -Recurse -Force (Join-Path $Dest $name) -ErrorAction SilentlyContinue
}
Get-ChildItem $Dest -Filter "PySide6_Essentials-*.dist-info" -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force
Get-ChildItem $Dest -Filter "shiboken6-*.dist-info" -ErrorAction SilentlyContinue | Remove-Item -Recurse -Force
& $Python -m pip install --quiet --no-deps --target $Dest "PySide6-Essentials==$Version" "shiboken6==$Version"
if ($LASTEXITCODE) { throw "PySide6 and shiboken6 $Version could not be installed" }

$savedPath = $env:PYTHONPATH
$env:PYTHONPATH = $Dest
$got = & $Python -c "import PySide6, shiboken6; print(PySide6.__version__, shiboken6.__version__)"
if ($null -eq $savedPath) { Remove-Item Env:\PYTHONPATH -ErrorAction SilentlyContinue } else { $env:PYTHONPATH = $savedPath }
if ("$got" -ne "$Version $Version") {
    throw "PySide6 and shiboken6 must both be $Version; got $got"
}

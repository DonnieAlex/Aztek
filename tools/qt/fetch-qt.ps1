# Qt for the build, from Qt's own kit, fetched with aqtinstall: the kit PySide6 is built for, so a build never links a
# Qt from the machine. A kit already in <dest> is reused, so later builds download nothing.
#   tools\qt\fetch-qt.ps1 -Version <version> -Dest <dir> [-Python <python.exe>]
# Prints <dest>\<version>\msvc2022_64.
param(
    [Parameter(Mandatory = $true)][string]$Version,
    [Parameter(Mandatory = $true)][string]$Dest,
    [string]$Python = ""
)

$arch = "win64_msvc2022_64"
$kit = "msvc2022_64"
$qt = Join-Path $Dest "$Version\$kit"
if (Test-Path (Join-Path $qt "lib\cmake\Qt6")) { Write-Output $qt; exit 0 }

if (-not $Python) {
    $Python = (Get-Command python -ErrorAction SilentlyContinue).Source
    if (-not $Python) { throw "Python is needed once to fetch Qt" }
}
New-Item -ItemType Directory -Force $Dest | Out-Null
# aqtinstall in a venv of its own, so neither the build's Python nor the machine's pip is changed.
$venv = Join-Path $Dest ".aqt"
& $Python -m venv $venv
if ($LASTEXITCODE) { throw "aqtinstall's venv could not be made" }
$aqt = Join-Path $venv "Scripts\python.exe"
# Pinned with hashes, so a changed or new release on PyPI never runs here.
& $aqt -m pip install -q --retries 10 --timeout 120 --require-hashes -r (Join-Path $PSScriptRoot "aqtinstall-requirements.txt")
if ($LASTEXITCODE) { throw "aqtinstall could not be installed" }
# Qt's download servers drop connections now and then: a few tries, each starting clean.
for ($i = 1; $i -le 5; $i++) {
    & $aqt -m aqt install-qt windows desktop $Version $arch --outputdir $Dest
    if (-not $LASTEXITCODE) { break }
    if ($i -eq 5) { throw "Qt $Version : download failed 5 times" }
    Remove-Item -Recurse -Force (Join-Path $Dest $Version) -ErrorAction SilentlyContinue
    Start-Sleep -Seconds ($i * 60)
}
Remove-Item -Recurse -Force $venv
if (-not (Test-Path (Join-Path $qt "lib\cmake\Qt6"))) { throw "Qt $Version : the kit did not arrive in $qt" }
Write-Output $qt

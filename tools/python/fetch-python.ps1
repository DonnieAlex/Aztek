# Python 3.13, the version Aztek embeds (the VFX Reference Platform CY2027 version), for machines that have no 3.13: a
# relocatable build from python-build-standalone, checked against its published SHA-256.
#   tools\python\fetch-python.ps1 <folder>
# Unpacks into <folder> (python.exe, python313.dll, Lib, DLLs) and returns <folder>. Nothing is downloaded when it is
# already there.
param([Parameter(Mandatory = $true)][string]$Dest)
$ErrorActionPreference = "Stop"

$release = "20261110"
$version = "3.13.18"
$sum = "7e93813bd362c6413f30788b248590e2f169df2d17dcac81aafbcc7a6eeb542c"
$stamp = Join-Path $Dest ".aztek-python-$version-$release"
if (Test-Path $stamp) { return (Resolve-Path $Dest).Path }

$name = "cpython-$version+$release-x86_64-pc-windows-msvc-install_only.tar.gz"
$url = "https://github.com/astral-sh/python-build-standalone/releases/download/$release/$name"
$tmp = Join-Path ([System.IO.Path]::GetTempPath()) ("aztek-python-" + [guid]::NewGuid())
New-Item -ItemType Directory $tmp | Out-Null
try {
    Write-Host "Downloading Python ${version}: $url"
    Invoke-WebRequest -Uri $url -OutFile "$tmp\python.tar.gz" -UseBasicParsing
    $got = (Get-FileHash "$tmp\python.tar.gz" -Algorithm SHA256).Hash.ToLower()
    if ($got -ne $sum) { throw "Python download: checksum mismatch" }
    # tar.exe ships with Windows 10 1803 and later.
    tar -xzf "$tmp\python.tar.gz" -C $tmp
    if ($LASTEXITCODE) { throw "unpacking Python failed" }
    if (Test-Path $Dest) { Remove-Item $Dest -Recurse -Force }
    New-Item -ItemType Directory -Force (Split-Path $Dest -Parent) | Out-Null
    Move-Item "$tmp\python" $Dest
    New-Item -ItemType File $stamp | Out-Null
} finally {
    Remove-Item $tmp -Recurse -Force -ErrorAction SilentlyContinue
}
return (Resolve-Path $Dest).Path

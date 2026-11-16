# Python 3.12, the version Aztek embeds (as the reference's own Python is 3.12), for machines that have no 3.12: a
# relocatable build from python-build-standalone, checked against its published SHA-256.
#   tools\python\fetch-python.ps1 <folder>
# Unpacks into <folder> (python.exe, python312.dll, Lib, DLLs) and returns <folder>. Nothing is downloaded when it is
# already there.
param([Parameter(Mandatory = $true)][string]$Dest)
$ErrorActionPreference = "Stop"

$release = "20260924"
$version = "3.12.14"
$sum = "c5303174bc29f5205decf6721ac549d4eb41c448f9b8c46cbc562d00348865bb"
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

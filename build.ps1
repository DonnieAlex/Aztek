# Builds Aztek on Windows: .\build.ps1 [-Debug] [-Run]
# Every build is self-contained: Qt, Python and PySide6/shiboken6 are fetched into build\ when they are not there, the
# program is built against those and never against a Qt or Python from the machine, and build\dist runs with nothing else
# installed. tools\bundle\versions.ps1 holds the versions. Needs Visual Studio 2022 (or Build Tools) with the C++ workload,
# Rust (rustup) and CMake; Ninja ships with Visual Studio's CMake tools.
param(
    [switch]$Debug,
    [switch]$Run
)
$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot
. "$PSScriptRoot\tools\bundle\versions.ps1"

$type = if ($Debug) { "Debug" } else { "Release" }
$dir = if ($Debug) { "build-debug" } else { "build" }
$cache = Join-Path $PSScriptRoot "build"

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    throw "Rust is missing: install it from https://rustup.rs"
}

# The MSVC environment, from Visual Studio's own locator.
if (-not $env:VCINSTALLDIR) {
    $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (-not (Test-Path $vswhere)) { throw "Visual Studio 2022 or its Build Tools with the C++ workload are missing" }
    $vs = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
    if (-not $vs) { throw "Visual Studio has no C++ tools: add the 'Desktop development with C++' workload" }
    $vcvars = Join-Path $vs "VC\Auxiliary\Build\vcvars64.bat"
    cmd /c "`"$vcvars`" >nul && set" | ForEach-Object {
        if ($_ -match "^(.*?)=(.*)$") { Set-Item -Path "env:$($matches[1])" -Value $matches[2] }
    }
}

# Python 3.13, the only version Aztek embeds, fetched once into build\python: the exe links the one copied below.
$bundle = & "$PSScriptRoot\tools\python\fetch-python.ps1" (Join-Path $cache "python")
$py = Join-Path $bundle "python.exe"
Write-Host "Python: $py"

# Qt, from Qt's own kit, and PySide6/shiboken6 built for it and installed into the Python that is copied below.
$Qt = (& "$PSScriptRoot\tools\qt\fetch-qt.ps1" -Version $AztekQtVersion -Dest (Join-Path $cache "qt") -Python $py | Select-Object -Last 1)
if (-not $Qt -or -not (Test-Path "$Qt\lib\cmake\Qt6")) { throw "Qt $AztekQtVersion could not be fetched" }
Write-Host "Qt: $Qt"
& "$PSScriptRoot\tools\python\fetch-pyside.ps1" -Python $py -Version $AztekQtVersion -Dest (Join-Path $bundle "Lib\site-packages")
if ($LASTEXITCODE) { throw "PySide6 and shiboken6 $AztekQtVersion could not be installed" }

$env:PYO3_PYTHON = $py
cmake -B $dir -G Ninja "-DCMAKE_BUILD_TYPE=$type" "-DCMAKE_PREFIX_PATH=$Qt" "-DPython3_EXECUTABLE=$py" `
    "-DPython3_ROOT_DIR=$bundle" "-DPython3_FIND_REGISTRY=NEVER" `
    "-DPython3_INCLUDE_DIR=$bundle\include" "-DPython3_LIBRARY=$bundle\libs\python313.lib"
if ($LASTEXITCODE) { throw "configure failed" }
cmake --build $dir
if ($LASTEXITCODE) { throw "build failed" }

# A runnable folder that needs nothing else installed: the exe, Qt's DLLs and plugins, the VC runtime, Python's DLL and
# standard library with PySide6 and shiboken6 (Python finds Lib\os.py beside the exe), and the example scenes.
$dist = "$dir\dist"
New-Item -ItemType Directory -Force $dist | Out-Null
Copy-Item "$dir\aztek.exe" $dist -Force
# aztekpy, the reference's mayapy, beside it.
if (Test-Path "$dir\aztekpy.exe") { Copy-Item "$dir\aztekpy.exe" $dist -Force }
# Harold's command-line renderer, for .hss scene files.
Copy-Item "$dir\harold.exe" $dist -Force
& "$Qt\bin\windeployqt.exe" --no-translations --no-compiler-runtime "$dist\aztek.exe" | Out-Null
if ($LASTEXITCODE) { throw "windeployqt failed" }
if ($env:VCToolsRedistDir) {
    Get-ChildItem "$env:VCToolsRedistDir\x64\Microsoft.VC*.CRT\*.dll" | Copy-Item -Destination $dist -Force
}
Get-ChildItem $bundle -Filter "python3*.dll" | Copy-Item -Destination $dist -Force
robocopy "$bundle\Lib" "$dist\Lib" /E /NFL /NDL /NJH /NJS /NP /XD __pycache__ site-packages test tests idlelib tkinter turtledemo | Out-Null
if ($LASTEXITCODE -ge 8) { throw "copying Python's Lib failed" }
robocopy "$bundle\DLLs" "$dist\DLLs" /E /NFL /NDL /NJH /NJS /NP /XD __pycache__ /XF tcl*.dll tk*.dll _tkinter.pyd | Out-Null
if ($LASTEXITCODE -ge 8) { throw "copying Python's DLLs failed" }
# PySide6 and shiboken6, the ones the build linked: the wheel's Qt DLLs stay inside them, beside the exe's own Qt.
foreach ($name in "PySide6", "shiboken6") {
    robocopy "$bundle\Lib\site-packages\$name" "$dist\Lib\site-packages\$name" /E /NFL /NDL /NJH /NJS /NP /XD __pycache__ | Out-Null
    if ($LASTEXITCODE -ge 8) { throw "copying $name failed" }
}
$global:LASTEXITCODE = 0
# pip in Aztek's own Python, as the reference's mayapy has it: `aztekpy -m pip install ...`. Not fatal: the program runs
# without it.
if (Test-Path "$dist\aztekpy.exe") {
    # Windows PowerShell turns a native program's stderr into errors, which "Stop" would make fatal.
    $ErrorActionPreference = "Continue"
    & "$dist\aztekpy.exe" -m ensurepip --default-pip 2>&1 | Out-Host
    $ErrorActionPreference = "Stop"
    if ($LASTEXITCODE -ne 0) { Write-Warning "pip was not added to the bundled Python" }
    $global:LASTEXITCODE = 0
}
if (Test-Path examples) { Copy-Item examples "$dist\examples" -Recurse -Force }
Write-Host "Built $dist\aztek.exe"

if ($Run) { & "$dist\aztek.exe" }

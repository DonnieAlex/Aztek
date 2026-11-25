# Packaging

Everything is built by `.github/workflows/build.yml` and attached to the `latest` pre-release (each push to main) and to
versioned releases (`v*` tags). The version comes from `Cargo.toml`, `[workspace.package]`. The scripts below also run
by hand.

The workflow trades speed for builds that finish (from 14/11/2026):

- Two jobs at a time everywhere (`CARGO_BUILD_JOBS`, `CMAKE_BUILD_PARALLEL_LEVEL`, `RUST_TEST_THREADS`), no
  incremental builds and no debug info in test builds, so a build stays within the runner's memory.
- Every package download (apt, dnf, Homebrew, pip, Qt, crates) is tried again before it fails.
- What the build downloads and runs is pinned and checked: Python against its SHA-256 (`tools/python/fetch-python.sh`),
  appimagetool 1.9.1 against its SHA-256 (`tools/appimage/fetch-appimagetool.sh`), and aqtinstall with everything it
  needs at exact versions with hashes (`tools/qt/aqtinstall-requirements.txt`, installed with `--require-hashes`).
- Build, test, package and smoke-test steps go through `tools/ci/retry.sh`: up to three tries, and after a failure one
  job at a time, the least memory there is. Windows does the same in its build step.
- The Linux jobs first remove the runner's unused SDKs (`tools/ci/free-disk.sh`), about 25 GB, through volumes in the
  container jobs, so the two target directories, Qt and the packages fit.
- Each job may take up to 340 minutes.

## VFX Reference Platform

The builds follow the VFX Reference Platform for CY2027 (vfxplatform.com, final 30/08/2026) where Aztek uses what it
lists, with the versions in `tools/bundle/versions.sh` and `.ps1`:

- Python 3.13.x, the only version Aztek embeds (3.13.18, `tools/python/fetch-python.sh`).
- Qt and PySide6/shiboken6 6.8.x: 6.8.3, the last open source 6.8 release, on every platform. The ARM AppImage is the
  exception: it uses Rocky 9's Qt, as there is no arm64 Linux kit or wheel for 6.8.
- C++20 for the shell (`CMAKE_CXX_STANDARD`). The plug-in kit compiles as C++14 and C++20 (`tools/check_kit.sh`).
- gcc 14 at most: `gcc-toolset-14` on Rocky 8, the system's gcc 11 on Rocky 9.
- glibc: what is built on Rocky 8 needs glibc 2.28 or newer, older than the platform's 2.34, so it runs on EL8 as well
  as EL9.
- macOS 14.0 deployment target, Visual Studio 2022 (toolset v14.44 on the windows-2022 runner).

OpenEXR, OpenUSD and the rest of the platform's libraries are not linked: `.exr` and USD are read and written by Rust
crates (`exr`, `openusd`) with no C++ dependency.

| Package | Built on | Script | Qt | Python |
| --- | --- | --- | --- | --- |
| `aztek-<version>-1.el8.x86_64.rpm` | `rockylinux:8` container | `packaging/rpm/build-rpm.sh` | 6.8.3 bundled | 3.13 bundled |
| `aztek-<version>-1.el9.x86_64.rpm` | `rockylinux:9` container | `packaging/rpm/build-rpm.sh` | 6.8.3 bundled | 3.13 bundled |
| `aztek_<version>-1_ubuntu24.04_amd64.deb` | ubuntu-24.04 | `packaging/deb/build-deb.sh` | 6.8.3 bundled | 3.13 bundled |
| `aztek_<version>-1_ubuntu26.04_amd64.deb` | `ubuntu:26.04` container | `packaging/deb/build-deb.sh` | 6.8.3 bundled | 3.13 bundled |
| `aztek-<version>-setup-x64.exe` | windows-2022 | `packaging/windows/build-installer.ps1` | 6.8.3 bundled | 3.13 bundled |
| `aztek-windows-x86_64.zip` | windows-2022 | `build.ps1` | 6.8.3 bundled | 3.13 bundled |
| `aztek-linux-x86_64.tar.gz` | ubuntu-24.04 | `build.sh` | 6.8.3 bundled | 3.13 bundled |
| `aztek-<version>-x86_64.AppImage` | `rockylinux:8` container | `build.sh`, `packaging/appimage/pack-appimage.sh` | 6.8.3 bundled | 3.13 bundled |
| `aztek-<version>-aarch64.AppImage` | `rockylinux:9` container on ubuntu-24.04-arm | `packaging/appimage/build-appimage.sh` | Rocky 9's, bundled | 3.13 bundled |

Every package except the ARM AppImage carries Python 3.13 and Qt and PySide6/shiboken6 6.8.3. The ARM AppImage is the
exception: it uses Rocky 9's Qt and goes without PySide6, as there is no arm64 Linux kit or wheel.

## Rocky Linux

Both Rocky 8 and 9 install the same self-contained build. The RPM carries Python 3.13, Qt and PySide6/shiboken6 6.8.3 in
`/opt/aztek`, and
`/usr/bin/aztek` starts it. Nothing else has to be installed.

    dnf install ./aztek-*.rpm

Building one needs EPEL, for `xcb-util-cursor`; the bundle carries that library, so a machine only installing the RPM
does not. Rocky 8 builds with `gcc-toolset-14` because Qt's headers want more than gcc 8.

To build by hand, as root in a container, with the checkout mounted:

    packaging/rpm/install-build-deps.sh
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal && . ~/.cargo/env
    packaging/rpm/build-rpm.sh build dist

## Vulkan at run time

The viewport loads `libvulkan.so.1` with dlopen, so it is declared by hand: `vulkan-loader` and `libvulkan1`. It also
needs a driver:

- NVIDIA workstations: the proprietary driver has its own Vulkan driver (`nvidia_icd.json`). Nothing else is needed.
  Aztek picks a discrete GPU over Mesa's when both are there.
- `mesa-vulkan-drivers` is a weak dependency, for AMD, Intel and software rendering. It is harmless beside NVIDIA's.
- Rocky 8's Mesa software driver (lavapipe) fails to start, seen in `vulkaninfo` as well as Aztek. Without a GPU on Rocky
  8 the viewport is black with a Vulkan error in the status line; the rest of the application works.
- The NVIDIA driver crashes in `libnvidia-eglcore` when asked for a surface on an X server without GLX, which takes
  Aztek with it. Seen on Rocky 8's Xvfb, and a VNC server without GLX should behave the same. Either use a server with
  GLX, or select Mesa with `VK_DRIVER_FILES=/usr/share/vulkan/icd.d/lvp_icd.x86_64.json` where lavapipe works.

Aztek runs through X11 only (`QT_QPA_PLATFORM=xcb`, XWayland on a Wayland session).

## AppImage

    chmod +x aztek-*-x86_64.AppImage
    ./aztek-*-x86_64.AppImage

On 64-bit ARM, `aztek-*-aarch64.AppImage`, built the same way on GitHub's ARM runner, but in Rocky 9 with its own Qt 6
bundled: Qt's ARM kit needs a newer C library than Rocky 8's. So it needs glibc 2.34 or newer (Ubuntu 22.04, Debian 12,
Raspberry Pi OS 12, Fedora 35 and later), and a Vulkan driver, as the viewport draws only with Vulkan. Its build and
check can't fail a run or hold a release back.

For any other x86_64 Linux, and Ubuntu releases the `.deb` files aren't built for. One file with Qt, Python and its
standard library, and the libraries a distribution may not have, inside; built on Rocky 8 so that its C library
(glibc 2.28) is the newest it needs. What it takes from the system is what has to match the machine or every desktop
has: the C library, GL and Vulkan, X11, fontconfig and freetype, glib and D-Bus (the list is in the script, after the
AppImage project's excludelist). It needs FUSE to mount itself; without it, `--appimage-extract-and-run` unpacks and
runs. Help > Check for Updates replaces the file with the new one.

PySide6 and shiboken6 come inside every package, the release built for the bundled Qt, for Python tools with windows of
their own. PySide only works with the very Qt it was built for, so the two are installed together at the Qt version
(`tools/python/fetch-pyside.sh`) and the build fails rather than pairing versions that differ. Python stops a PySide
built for another Qt at import with an ImportError naming the one to install, rather than letting it bring the program
down.

CI starts it unpacked in clean `ubuntu:24.04` and `ubuntu:rolling` containers (`verify-appimage`). A build or check
that fails leaves that release without an AppImage rather than holding the release back.

The program itself must not need an OpenGL library. Qt's `Gui` target puts `libOpenGL` and `libGLX` on the link line
of whatever links it, and with Qt's own kit the linker kept both in the program even with `--as-needed`, so it would
not start in the clean containers, which have no `libOpenGL.so.0`. `CMakeLists.txt` takes them off the link line, and
`build-appimage.sh` stops if the program needs one again. Its log lists what the AppImage takes from the system.

## Ubuntu

    sudo apt install ./aztek_*.deb

Depends come from `dpkg-shlibdeps` on the bundled binary, plus what it cannot see from `packaging/deb/control.in`:
`libvulkan1` and `hicolor-icon-theme`. The package installs the bundle in `/opt/aztek`, with `/usr/bin/aztek`,
`/usr/bin/aztekpy` and `/usr/bin/harold` starting it. It needs no Qt, Python or PySide6 from Ubuntu; only the desktop
libraries the bundle still takes (X11, fontconfig, glib, freetype) come from the system.

## Windows

`build.ps1` makes `build\dist`, which runs on a machine with nothing else installed: Qt 6.8.3 from `windeployqt`, the
VC runtime copied beside the exe, Python 3.13's DLL with its standard library in `Lib` and its extension modules in
`DLLs`, and PySide6/shiboken6 6.8.3 in `Lib\site-packages`. Qt and Python are fetched into `build\` by
`tools\qt\fetch-qt.ps1` and `tools\python\fetch-python.ps1`, and PySide6 by `tools\python\fetch-pyside.ps1`; none is
taken from the machine. The installer carries the folder, so it needs no Qt or Python where it installs.

`packaging/windows/aztek.iss` is the Inno Setup script: Start Menu entry, uninstaller, and two optional tasks, off by
default, for a desktop shortcut and for opening `.ma` files with Aztek. It installs for all users or the current one.

The installer is not signed, so SmartScreen warns about it until someone signs it with a code signing certificate.

Inno Setup's compiler prints "Non-commercial use only". Commercial use of Aztek's installer build needs a licence from
its author, or a move to NSIS.

## Checks

`packaging/linux/smoke-test.sh` and `packaging/windows/smoke-test.ps1` run in the workflow on the installed package, in a
clean container on Linux: libraries resolve, the application starts, a Python script runs through `aztek.cmds`, and a
screenshot is saved. A failure stops the release. They do not check that the viewport drew, since the runners have no
GPU.

## Examples

`examples/` in the repository, if it exists, is installed to `usr/share/aztek/examples` in the bundle (that is
`/opt/aztek/usr/share/aztek/examples` in the packages) and `examples` beside the exe on Windows.

## macOS

Early. The `macos` job builds `aztek-macos-arm64.zip` on a `macos-15` runner (Apple silicon), for macOS 14 and later. It isn't a condition of a
release yet: when it fails, the other packages still go out.

- `Aztek.app` carries Qt (`macdeployqt`), `libpython3.13.dylib` and `libMoltenVK.dylib` in `Contents/Frameworks`, and
  Python's standard library with PySide6/shiboken6 in `Contents/Resources/python`, and the examples in
  `Contents/Resources/examples`, where File > Examples and the Home window look.
- Signing: `packaging/macos/sign.sh` signs ad hoc, or, with `AZTEK_SIGN_IDENTITY` naming a Developer ID Application
  certificate, signs every dylib, module, plug-in, framework and program inside, deepest first, with the hardened
  runtime, a timestamp and `packaging/macos/aztek.entitlements` on the app. `packaging/macos/notarize.sh` then submits it
  with an App Store Connect API key (`AZTEK_NOTARY_KEY`, `AZTEK_NOTARY_KEY_ID`, `AZTEK_NOTARY_ISSUER`) and staples the
  ticket; `--package` runs it when both are set.
- The `macos` job takes the certificate and key from the repository secrets `MACOS_CERTIFICATE` (the .p12, base64),
  `MACOS_CERTIFICATE_PASSWORD`, `APPLE_API_KEY` (the .p8, base64), `APPLE_API_KEY_ID` and `APPLE_API_ISSUER`. Without
  them the app is signed ad hoc and Gatekeeper asks first: System Settings > Privacy & Security, Open Anyway, or
  `xattr -dr com.apple.quarantine Aztek.app`.
- Every build fetches Qt 6.8.3, Python 3.13 and PySide6 6.8.3 into `build/`; nothing comes from Homebrew but the build
  tools.
- `gpu.rs` loads MoltenVK from the bundle first, then a Vulkan loader on the library path, then Homebrew's folders.
  The viewport draws through `AZTEK_APPKIT`: the widget's `NSView`, with a Metal layer from `ash_window`.
- `./build-macos.sh` builds it, installing what is missing first (Xcode's command line tools, Homebrew, CMake, Ninja, MoltenVK, Rust; `--deps` updates them); `--package` makes the same `Aztek.app` and zip as the release build, which runs that script.
- Not checked yet: the viewport on a real Mac, and whether Ctrl and Alt in mouse and key handling should be Cmd and Option.

## Flatpak and snap

- There is no Flatpak any more (from 20/11/2026). A Flatpak installed before is pointed at the AppImage by Check for
  Updates.
- There is no snap any more (from 14/11/2026). A snap installed before is pointed at the AppImage by Check for Updates.

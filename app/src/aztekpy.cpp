/* aztekpy, the reference's mayapy: runs the aztek program beside it as `aztek --python ...`, which is Python's own
 * command line with Aztek's modules and no window. A program of its own so it can sit on PATH, be what a tool's
 * sys.executable names, and keep a console on Windows, where aztek itself is a windowed program. */
#ifdef _WIN32
#include <windows.h>
#include <cstdio>
#include <cstdlib>
#include <cwchar>

/* One argument quoted as CommandLineToArgvW reads it back. */
static void quote(const wchar_t* a, wchar_t** out) {
    wchar_t* o = *out;
    *o++ = L'"';
    for (;;) {
        unsigned slashes = 0;
        while (*a == L'\\') {
            ++a;
            ++slashes;
        }
        if (!*a) {
            for (unsigned i = 0; i < slashes * 2; ++i) *o++ = L'\\';
            break;
        }
        if (*a == L'"') {
            for (unsigned i = 0; i < slashes * 2 + 1; ++i) *o++ = L'\\';
        } else {
            for (unsigned i = 0; i < slashes; ++i) *o++ = L'\\';
        }
        *o++ = *a++;
    }
    *o++ = L'"';
    *o++ = L' ';
    *out = o;
}

int wmain(int argc, wchar_t** argv) {
    wchar_t exe[32768];
    DWORD n = GetModuleFileNameW(NULL, exe, 32768);
    if (n == 0 || n >= 32768) return 1;
    wchar_t* slash = wcsrchr(exe, L'\\');
    if (!slash) return 1;
    wcscpy(slash + 1, L"aztek.exe");
    size_t len = wcslen(exe) * 2 + 16;
    for (int i = 1; i < argc; ++i) len += wcslen(argv[i]) * 2 + 4;
    wchar_t* cmd = static_cast<wchar_t*>(malloc(len * sizeof(wchar_t)));
    if (!cmd) return 1;
    wchar_t* o = cmd;
    quote(exe, &o);
    wcscpy(o, L"--python ");
    o += 9;
    for (int i = 1; i < argc; ++i) quote(argv[i], &o);
    *o = 0;
    STARTUPINFOW si;
    PROCESS_INFORMATION pi;
    ZeroMemory(&si, sizeof si);
    si.cb = sizeof si;
    si.dwFlags = STARTF_USESTDHANDLES;
    si.hStdInput = GetStdHandle(STD_INPUT_HANDLE);
    si.hStdOutput = GetStdHandle(STD_OUTPUT_HANDLE);
    si.hStdError = GetStdHandle(STD_ERROR_HANDLE);
    if (!CreateProcessW(exe, cmd, NULL, NULL, TRUE, 0, NULL, NULL, &si, &pi)) {
        fwprintf(stderr, L"aztekpy: can't start %ls\n", exe);
        return 1;
    }
    WaitForSingleObject(pi.hProcess, INFINITE);
    DWORD rc = 1;
    GetExitCodeProcess(pi.hProcess, &rc);
    CloseHandle(pi.hThread);
    CloseHandle(pi.hProcess);
    free(cmd);
    return (int)rc;
}
#else
#include <climits>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <unistd.h>
#ifdef __APPLE__
#include <mach-o/dyld.h>
#endif

int main(int argc, char** argv) {
    char exe[PATH_MAX];
#ifdef __APPLE__
    uint32_t size = sizeof exe;
    char raw[PATH_MAX];
    if (_NSGetExecutablePath(raw, &size) != 0 || !realpath(raw, exe)) return 1;
#else
    ssize_t n = readlink("/proc/self/exe", exe, sizeof exe - 1);
    if (n <= 0) return 1;
    exe[n] = 0;
#endif
    char* slash = strrchr(exe, '/');
    if (!slash || (size_t)(slash - exe) + 7 >= sizeof exe) return 1;
    strcpy(slash + 1, "aztek");
    char** args = static_cast<char**>(calloc(static_cast<size_t>(argc) + 2, sizeof(char*)));
    if (!args) return 1;
    args[0] = exe;
    args[1] = const_cast<char*>("--python");
    for (int i = 1; i < argc; ++i) args[i + 1] = argv[i];
    execv(exe, args);
    fprintf(stderr, "aztekpy: can't start %s\n", exe);
    return 1;
}
#endif

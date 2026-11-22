//! about: the application, its build and the system it runs on, one answer per flag as in the reference. With no flag
//! it gives the version line.

use std::path::PathBuf;

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, FlagDef};
use aztek_scene::Session;
use serde_json::{json, Value};

/// Every flag, long and short, in the reference's order.
const FLAGS: &[(&str, &str, &str)] = &[
    ("apiVersion", "api", "The API version."),
    ("application", "a", "The application's name."),
    ("arm64", "a64", "Whether this build runs on an arm64 processor."),
    ("armHost", "ah", "Whether the machine's processor is arm64, whatever this build is."),
    ("batch", "b", "Whether this is a batch session."),
    ("buildDirectory", "bd", "The folder the application runs from."),
    ("buildVariant", "bv", "The build variant: release or debug."),
    ("codeset", "cs", "The locale's code set."),
    ("compositingManager", "cm", "On Linux, whether a compositing manager runs; true elsewhere."),
    ("connected", "cnt", "Whether the machine has a route to the Internet."),
    ("creativeVersion", "cre", "Whether this is the reference's Creative edition: false."),
    ("ctime", "cti", "The local time as ctime writes it."),
    ("currentDate", "cd", "Today's date, yyyy/mm/dd."),
    ("currentTime", "ct", "The local time, hh:mm:ss."),
    ("customVersion", "cv", "Whether this is a custom version: false."),
    ("customVersionClient", "cvc", "The custom version's client: empty."),
    ("customVersionMajor", "cvm", "The custom version's major number: 0."),
    ("customVersionMinor", "cvn", "The custom version's minor number: 0."),
    ("customVersionString", "cvs", "The custom version's string: empty."),
    ("cutIdentifier", "c", "The build's cut: when it was built."),
    ("date", "d", "The build date, yyyy/mm/dd."),
    ("environmentFile", "env", "The environment file read at start, or where it would be."),
    ("evalVersion", "ev", "Obsolete: false."),
    ("file", "f", "The scene file version this writes."),
    ("fontInfo", "foi", "The fonts asked for and the ones in use."),
    ("helpDataDirectory", "hdd", "The folder of the help pages."),
    ("installedVersion", "iv", "The product and its version."),
    ("ioVersion", "io", "Whether this is the reference's IO edition: false."),
    ("irix", "ir", "Obsolete: false."),
    ("is64", "x64", "Whether the application is 64 bit."),
    ("languageResources", "lr", "The installed languages, each locale:name:ISO 639-2 code."),
    ("linux", "li", "Whether the system is Linux."),
    ("linux64", "l64", "Whether the system is 64 bit Linux."),
    ("liveUpdate", "lu", "Obsolete: the product and version."),
    ("localizedResourceLocation", "lrl", "Where the language's resources are when it isn't the default one: empty."),
    ("ltVersion", "lt", "Obsolete: false."),
    ("macOS", "mac", "Whether the system is macOS."),
    ("macOSASi", "asi", "Whether the system is macOS on Apple silicon."),
    ("macOSppc", "ppc", "Whether the system is macOS on PowerPC: false."),
    ("macOSx86", "x86", "Whether the system is macOS on Intel."),
    ("majorVersion", "mjv", "The major version."),
    ("minorVersion", "mnv", "The minor version."),
    ("ntOS", "nt", "Whether the system is Windows."),
    ("operatingSystem", "os", "nt, win64, mac, linux or linux64."),
    ("operatingSystemVersion", "osv", "The system's version; on Linux as uname -srvm gives it."),
    ("patchVersion", "pv", "The patch version."),
    ("preferences", "pd", "The preferences folder."),
    ("product", "p", "The product's name."),
    ("qtVersion", "qt", "The Qt version; empty in batch."),
    ("tablet", "tab", "Whether the machine is a Tablet PC: Windows only."),
    ("tabletMode", "tm", "tablet or laptop mode: Windows only."),
    ("uiLanguage", "uil", "The interface's language."),
    ("uiLanguageForStartup", "uis", "The interface language the next start uses."),
    ("uiLanguageIsLocalized", "uii", "Whether the interface runs in a language other than the default."),
    ("uiLocaleLanguage", "ull", "The system locale's language."),
    ("version", "v", "The version."),
    ("win64", "w64", "Whether the system is 64 bit Windows."),
    ("windowManager", "wm", "The window manager's name."),
    ("windows", "win", "Whether the system is Windows."),
];

fn version() -> &'static str {
    aztek_core::REFERENCE_VERSION
}

fn installed() -> String {
    format!("Aztek {}", version())
}

const MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Now in local time: year, month, day, hour, minute, second, day of the week (0 Sunday).
struct Clock {
    y: i64,
    mo: u32,
    d: u32,
    h: u32,
    mi: u32,
    s: u32,
    wd: u32,
}

#[cfg(unix)]
fn now() -> Clock {
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as libc::time_t).unwrap_or(0);
    let mut tm: libc::tm = unsafe { std::mem::zeroed() };
    unsafe { libc::localtime_r(&t, &mut tm) };
    Clock { y: tm.tm_year as i64 + 1900, mo: tm.tm_mon as u32 + 1, d: tm.tm_mday as u32, h: tm.tm_hour as u32, mi: tm.tm_min as u32, s: tm.tm_sec as u32, wd: tm.tm_wday as u32 }
}

/// Windows: universal time, there being no local time zone call here.
#[cfg(not(unix))]
fn now() -> Clock {
    let secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs() as i64).unwrap_or(0);
    let days = secs.div_euclid(86_400);
    let (y, mo, d) = civil(days);
    let r = secs.rem_euclid(86_400);
    Clock { y, mo, d, h: (r / 3600) as u32, mi: (r / 60 % 60) as u32, s: (r % 60) as u32, wd: (days + 4).rem_euclid(7) as u32 }
}

#[cfg(not(unix))]
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (yoe + era * 400 + if m <= 2 { 1 } else { 0 }, m, d)
}

/// The locale the system gives: LC_ALL, LC_MESSAGES, then LANG; en_US.UTF-8 when none says.
fn locale() -> String {
    ["LC_ALL", "LC_MESSAGES", "LANG"]
        .iter()
        .filter_map(|k| std::env::var(k).ok())
        .find(|v| !v.is_empty() && v != "C" && v != "POSIX")
        .unwrap_or_else(|| "en_US.UTF-8".to_string())
}

fn codeset() -> String {
    if cfg!(windows) {
        return "1252".to_string();
    }
    locale().split('.').nth(1).map(|c| c.split('@').next().unwrap_or(c).to_string()).unwrap_or_else(|| "UTF-8".to_string())
}

fn locale_language() -> String {
    let l = locale();
    let l = l.split(['.', '@']).next().unwrap_or("en_US").to_string();
    if l.is_empty() {
        "en_US".to_string()
    } else {
        l
    }
}

fn os_version() -> String {
    if cfg!(target_os = "linux") {
        let read = |f: &str| std::fs::read_to_string(format!("/proc/sys/kernel/{f}")).map(|t| t.trim().to_string()).unwrap_or_default();
        let machine = match std::env::consts::ARCH {
            "x86_64" => "x86_64",
            "aarch64" => "aarch64",
            a => a,
        };
        let parts = [read("ostype"), read("osrelease"), read("version"), machine.to_string()];
        return parts.iter().filter(|p| !p.is_empty()).cloned().collect::<Vec<_>>().join(" ");
    }
    if cfg!(target_os = "macos") {
        return format!("macOS {}", std::env::consts::ARCH);
    }
    format!("Windows {}", std::env::consts::ARCH)
}

/// Whether there is a route out: a UDP socket "connected" to a public address picks a route without sending anything.
fn connected() -> bool {
    let Ok(sock) = std::net::UdpSocket::bind("0.0.0.0:0") else { return false };
    if sock.connect("192.0.2.1:9").is_err() && sock.connect("8.8.8.8:53").is_err() {
        return false;
    }
    sock.local_addr().is_ok_and(|a| !a.ip().is_loopback() && !a.ip().is_unspecified())
}

fn compositing() -> bool {
    if !cfg!(target_os = "linux") {
        return true;
    }
    if aztek_core::batch_mode() {
        return false;
    }
    // Wayland composites always; on X the desktops that ship a compositor.
    if std::env::var("AZTEK_QT_PLATFORM").is_ok_and(|p| p.starts_with("wayland")) || std::env::var_os("WAYLAND_DISPLAY").is_some() {
        return true;
    }
    let desk = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
    ["gnome", "kde", "cinnamon", "unity", "budgie", "pantheon", "deepin", "xfce"].iter().any(|d| desk.contains(d))
}

fn window_manager() -> String {
    if cfg!(windows) {
        return "Windows".to_string();
    }
    if cfg!(target_os = "macos") {
        return "Aqua".to_string();
    }
    for k in ["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "XDG_SESSION_DESKTOP"] {
        if let Ok(v) = std::env::var(k) {
            if !v.is_empty() {
                return v;
            }
        }
    }
    String::new()
}

fn build_date() -> &'static str {
    option_env!("AZTEK_BUILD_DATE").unwrap_or("")
}

fn cut() -> String {
    let d = option_env!("AZTEK_BUILD_STAMP").unwrap_or("");
    format!("{d}-{}", env!("CARGO_PKG_VERSION"))
}

fn env_file() -> PathBuf {
    let ap = crate::user_folder::applied();
    ap.env_file.clone().unwrap_or_else(|| {
        let base = if ap.version.is_empty() { ap.root.clone() } else { ap.root.join(&ap.version) };
        base.join("Aztek.env")
    })
}

fn path_text(p: PathBuf) -> String {
    aztek_core::paths::to_utf8(&p)
}

fn about(_s: &mut Session, a: &Args) -> CmdResult {
    let (win, linux, mac) = (cfg!(windows), cfg!(target_os = "linux"), cfg!(target_os = "macos"));
    let arm = cfg!(target_arch = "aarch64");
    let wide = cfg!(target_pointer_width = "64");
    let Some(&(flag, _, _)) = FLAGS.iter().find(|(name, _, _)| a.bool(name)) else {
        if let Some(k) = a.flags.keys().find(|k| !matches!(k.as_str(), "queried")) {
            fail!("about: invalid flag -{k}");
        }
        // No flag: the version line.
        return Ok(json!(format!("{} ({})", installed(), cut())));
    };
    let c = || now();
    Ok(match flag {
        "apiVersion" => json!(version().parse::<i64>().unwrap_or(0) * 10_000),
        "application" => json!("aztek"),
        "arm64" => json!(arm),
        // A build for the machine's own processor: emulation isn't asked about here.
        "armHost" => json!(arm),
        "batch" => json!(aztek_core::batch_mode()),
        "buildDirectory" => json!(path_text(aztek_core::paths::executable_dir())),
        "buildVariant" => json!(if cfg!(debug_assertions) { "debug" } else { "release" }),
        "codeset" => json!(codeset()),
        "compositingManager" => json!(compositing()),
        "connected" => json!(connected()),
        "creativeVersion" | "customVersion" | "evalVersion" | "ioVersion" | "irix" | "ltVersion" | "macOSppc" | "uiLanguageIsLocalized" => json!(false),
        "tablet" => json!(false),
        "ctime" => {
            let c = c();
            json!(format!("{} {} {:>2} {:02}:{:02}:{:02} {}", DAYS[c.wd as usize % 7], MONTHS[(c.mo as usize + 11) % 12], c.d, c.h, c.mi, c.s, c.y))
        }
        "currentDate" => {
            let c = c();
            json!(format!("{:04}/{:02}/{:02}", c.y, c.mo, c.d))
        }
        "currentTime" => {
            let c = c();
            json!(format!("{:02}:{:02}:{:02}", c.h, c.mi, c.s))
        }
        "customVersionClient" | "customVersionString" | "localizedResourceLocation" => json!(""),
        "customVersionMajor" | "customVersionMinor" => json!(0),
        "cutIdentifier" => json!(cut()),
        "date" => json!(build_date()),
        "environmentFile" => json!(path_text(env_file())),
        "file" | "version" | "majorVersion" => json!(version()),
        "minorVersion" | "patchVersion" => json!("0"),
        "fontInfo" => {
            let used = std::env::var("AZTEK_FONT_INFO").unwrap_or_default();
            json!(if used.is_empty() { "No interface fonts in batch mode.".to_string() } else { format!("Requested: the system's interface font. In use: {used}.") })
        }
        "helpDataDirectory" => json!(path_text(aztek_core::paths::resources_dir().parent().map(|p| p.join("docs")).unwrap_or_default())),
        "installedVersion" | "liveUpdate" => json!(installed()),
        "is64" => json!(wide),
        "languageResources" => json!(["en_US:English:eng"]),
        "linux" => json!(linux),
        "linux64" => json!(linux && wide),
        "macOS" => json!(mac),
        "macOSASi" => json!(mac && arm),
        "macOSx86" => json!(mac && cfg!(target_arch = "x86_64")),
        "ntOS" | "windows" => json!(win),
        "win64" => json!(win && wide),
        "operatingSystem" => json!(if win { "win64" } else if mac { "mac" } else { "linux64" }),
        "operatingSystemVersion" => json!(os_version()),
        "preferences" => json!(path_text(crate::user_folder::user_pref_dir())),
        "product" => json!("Aztek"),
        "qtVersion" => json!(std::env::var("AZTEK_QT_VERSION").unwrap_or_default()),
        "tabletMode" => json!("laptop"),
        "uiLanguage" | "uiLanguageForStartup" => json!("en_US"),
        "uiLocaleLanguage" => json!(locale_language()),
        "windowManager" => json!(window_manager()),
        _ => fail!("about: invalid flag -{flag}"),
    })
}

pub fn register(s: &mut Session) {
    let mut c = CommandDef::new("about", "The application, its build and the system: one answer per flag; with none, the version line.", about).undoable(false);
    for (name, short, doc) in FLAGS {
        c = c.flag(FlagDef::new(name, short, ArgType::Bool, Value::Null, doc));
    }
    s.register(c);
}

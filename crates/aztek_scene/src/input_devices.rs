//! Input devices: the devices scripts define (data servers, with defineDataServer) and what they are named by. Each
//! graph has its own list, by its serial, so undo can take a device away and give it back.
//!
//! A data server device holds its connection open: a local socket (`name` is `/tmp/name`, a name starting with `/` a
//! full path) or a UDP address (`host:service`, the service a port or a UDP service name). The server's axes are kept as
//! it describes them; a server that sends none leaves the device without axes.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use aztek_ops::{fail, ArgType, Args, CmdResult, CommandDef, Commands};
use serde_json::Value;

use crate::ref_core::f;
use crate::session::Session;

type Cmd = CommandDef<Session>;

/// An open connection to a data server.
#[derive(Debug)]
pub enum Link {
    #[cfg(unix)]
    Socket(std::os::unix::net::UnixStream),
    Udp(std::net::UdpSocket),
}

#[derive(Clone, Debug)]
pub struct Device {
    pub name: String,
    /// What made it: "dataServer" for defineDataServer.
    pub kind: String,
    /// The server as given.
    pub server: String,
    /// Its axes: channelName.usage for scalar channels, channelName.component for compound ones.
    pub axes: Vec<String>,
    pub link: Option<Arc<Link>>,
}

static DEVICES: Mutex<BTreeMap<u64, BTreeMap<String, Device>>> = Mutex::new(BTreeMap::new());

fn with<R>(serial: u64, f: impl FnOnce(&mut BTreeMap<String, Device>) -> R) -> R {
    let mut d = DEVICES.lock().unwrap_or_else(|e| e.into_inner());
    f(d.entry(serial).or_default())
}

/// The devices of a session's graph, by name.
pub fn devices(s: &Session) -> Vec<Device> {
    with(s.graph.serial(), |d| d.values().cloned().collect())
}

/// A device by name.
pub fn device(s: &Session, name: &str) -> Option<Device> {
    with(s.graph.serial(), |d| d.get(name).cloned())
}

/// Adds a device, undoably.
pub fn add(s: &mut Session, dev: Device) {
    let serial = s.graph.serial();
    let name = dev.name.clone();
    with(serial, |d| d.insert(name.clone(), dev.clone()));
    let n2 = name.clone();
    s.graph.record(move |_| with(serial, |d| { d.remove(&n2); }), move |_| with(serial, |d| { d.insert(name.clone(), dev.clone()); }));
}

/// Removes a device, undoably: its connection closes once the removal can no longer be undone.
pub fn remove(s: &mut Session, name: &str) -> Option<Device> {
    let serial = s.graph.serial();
    let dev = with(serial, |d| d.remove(name))?;
    let (n1, keep) = (name.to_string(), dev.clone());
    s.graph.record(move |_| with(serial, |d| { d.insert(n1.clone(), keep.clone()); }), {
        let n2 = name.to_string();
        move |_| with(serial, |d| { d.remove(&n2); })
    });
    Some(dev)
}

/// A UDP service's port: a number, or a name from the services list.
fn udp_port(service: &str) -> Option<u16> {
    if let Ok(p) = service.parse() {
        return Some(p);
    }
    let list = std::fs::read_to_string("/etc/services").unwrap_or_default();
    list.lines().filter_map(|l| {
        let l = l.split('#').next()?.trim();
        let mut w = l.split_whitespace();
        let name = w.next()?;
        let (port, proto) = w.next()?.split_once('/')?;
        let aliases: Vec<&str> = w.collect();
        ((name == service || aliases.contains(&service)) && proto == "udp").then(|| port.parse().ok()).flatten()
    }).next()
}

fn connect(server: &str) -> Result<Link, String> {
    if let Some((host, service)) = server.rsplit_once(':') {
        let host = if host.is_empty() { "localhost" } else { host };
        let port = udp_port(service).ok_or_else(|| format!("no UDP service {service}"))?;
        use std::net::ToSocketAddrs;
        let addr = (host, port).to_socket_addrs().map_err(|e| format!("{host}: {e}"))?.next().ok_or_else(|| format!("{host}: no address"))?;
        let local = if addr.is_ipv4() { "0.0.0.0:0" } else { "[::]:0" };
        let sock = std::net::UdpSocket::bind(local).map_err(|e| e.to_string())?;
        sock.connect(addr).map_err(|e| format!("{server}: {e}"))?;
        return Ok(Link::Udp(sock));
    }
    #[cfg(unix)]
    {
        let path = if server.starts_with('/') { server.to_string() } else { format!("/tmp/{server}") };
        let s = std::os::unix::net::UnixStream::connect(&path).map_err(|e| format!("{path}: {e}"))?;
        Ok(Link::Socket(s))
    }
    #[cfg(not(unix))]
    Err(format!("{server}: local socket servers need a Unix system; give host:service"))
}

/// What a server says its axes are, if it says so straight away: lines of axis names.
#[cfg(unix)]
fn read_axes(link: &Link) -> Vec<String> {
    use std::io::Read;
    let Link::Socket(s) = link else { return Vec::new() };
    let mut s = match s.try_clone() {
        Ok(s) => s,
        Err(_) => return Vec::new(),
    };
    let _ = s.set_read_timeout(Some(std::time::Duration::from_millis(100)));
    let mut buf = [0u8; 4096];
    let n = s.read(&mut buf).unwrap_or(0);
    String::from_utf8_lossy(&buf[..n]).split_whitespace().filter(|w| w.contains('.')).map(str::to_string).collect()
}

#[cfg(not(unix))]
fn read_axes(_link: &Link) -> Vec<String> {
    Vec::new()
}

fn define_data_server(s: &mut Session, a: &Args) -> CmdResult {
    let Some(name) = a.get("device").and_then(Value::as_str).filter(|n| !n.is_empty()).map(str::to_string) else { fail!("defineDataServer: -device is required") };
    if a.bool("undefine") {
        if device(s, &name).is_none_or(|d| d.kind != "dataServer") {
            fail!("defineDataServer: no data server device {name}");
        }
        remove(s, &name);
        return Ok(Value::Null);
    }
    let Some(server) = a.get("server").and_then(Value::as_str).filter(|n| !n.is_empty()).map(str::to_string) else { fail!("defineDataServer: -server is required") };
    if device(s, &name).is_some() {
        fail!("defineDataServer: a device named {name} exists already");
    }
    let link = connect(&server).map_err(|e| aztek_ops::CommandError::new(format!("defineDataServer: can't reach the server {e}")))?;
    let axes = read_axes(&link);
    add(s, Device { name, kind: "dataServer".into(), server, axes, link: Some(Arc::new(link)) });
    Ok(Value::Null)
}

pub fn register(c: &mut Commands<Session>) {
    c.register(
        Cmd::new(
            "defineDataServer",
            "Connect to a data server and make a device of it: -s a local socket name (/tmp/name, or a full path) or a UDP \
             host:service, -d the device's name, which must be new. -u -d closes the connection and removes the device.",
            define_data_server,
        )
        .flag(f("device", "d", ArgType::String, "The device's name."))
        .flag(f("server", "s", ArgType::String, "The server: a socket name, or host:service for UDP (host defaults to this machine)."))
        .flag(f("undefine", "u", ArgType::Bool, "Remove the device and close its connection.")),
    );
}

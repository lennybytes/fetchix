use local_ip_address::local_ip;
use std::env;
use std::fs;

fn get_kernel() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease")
        .unwrap_or_else(|_| "Unknown".to_string())
        .trim()
        .to_string()
}

fn get_local_ip() -> String {
    local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "Not connected".to_string())
}

fn get_distro() -> String {
    fs::read_to_string("/etc/os-release")
        .ok()
        .and_then(|info| {
            info.lines()
                .find(|line| line.starts_with("PRETTY_NAME="))
                .map(|line| {
                    line.trim_start_matches("PRETTY_NAME=")
                        .trim_matches('"')
                        .to_string()
                })
        })
        .unwrap_or_else(|| "Unknown".to_string())
}

// KORREKTUR: Rückgabetyp -> String hinzugefügt und unwrap_or_else genutzt
fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|e| format!("Couldn't read SHELL env var: {}", e))
}

fn main() {
    println!("OS:       {}", get_distro());
    println!("Kernel:   {}", get_kernel());
    println!("Local IP: {}", get_local_ip());
    println!("Shell:    {}", get_shell()); // KORREKTUR: Hier aufgerufen
}

use crate::utils::process;
use std::path::{Path, PathBuf};

pub fn detect() -> Option<String> {
    let proc_info = process::get_info(1);

    if proc_info.is_ok() {
        let proc_info = proc_info.unwrap();
        return Some(
            (match proc_info.command.trim() {
                "systemd" => "SystemD",
                "openrc-init" | "init-openrc" => "OpenRC",
                "runit" => "Runit",
                "init" => {
                    if Path::new("/run/dinit").exists() {
                        "Dinit"
                    } else if Path::new("/usr/share/sysvinit/inittab").exists()
                        || Path::new("/etc/inittab").exists()
                    {
                        "SysVinit"
                    } else if std::fs::read_link(proc_info.cmdline.split("\0").next().unwrap())
                        .unwrap_or(PathBuf::from("".to_string()))
                        .to_str()
                        == Some("openrc-init")
                    {
                        "OpenRC"
                    } else {
                        "Unknown"
                    }
                }
                "s6-svscan" => "S6",
                "upstart" => "Upstart",
                s => s,
            })
            .into(),
        );
    } else {
        return None;
    }
}

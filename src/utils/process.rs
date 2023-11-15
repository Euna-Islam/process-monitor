extern crate sysinfo;

use sysinfo::{ProcessExt, System, SystemExt};

pub struct ProcessInfo {
    pub name: String,
    pub id: String,
    pub cpu_usage: String,
    pub status: String
}

pub fn get_process_info() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let process_list: Vec<ProcessInfo> = sys.processes().iter()
    .map(|(pid, process)| {
        ProcessInfo {
            name: process.name().to_string(),
            id: pid.to_string(),
            cpu_usage: process.cpu_usage().to_string(),
            status: process.status().to_string(),
        }
    })
    .collect();

    process_list
}
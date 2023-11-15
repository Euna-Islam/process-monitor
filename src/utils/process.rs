extern crate sysinfo;

use sysinfo::{ProcessExt, System, SystemExt};
use process_monitor::models::process_info::ProcessInfo;

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
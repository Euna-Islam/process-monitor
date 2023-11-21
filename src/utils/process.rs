extern crate sysinfo;

use process_monitor::models::process_info::ProcessInfo;
use sysinfo::{ProcessExt, System, SystemExt};

pub fn get_process_info() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.refresh_all();
    let process_list: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .map(|(pid, process)| ProcessInfo {
            name: process.name().to_string(),
            id: pid.to_string(),
            cpu_usage: process.cpu_usage().to_string(),
            status: process.status().to_string(),
            memory_usage: process.memory().to_string(),
        })
        .collect();

    process_list
}

pub fn sort_by_memory() -> Vec<ProcessInfo> {
    let mut process_list: Vec<ProcessInfo> = get_process_info();

    process_list.sort_by(|a, b| {
        let a_mem = a.memory_usage.parse::<u64>().unwrap_or(0);
        let b_mem = b.memory_usage.parse::<u64>().unwrap_or(0);
        b_mem.cmp(&a_mem)
    });

    process_list
}

pub fn sort_by_cpu_usage() -> Vec<ProcessInfo> {
    let mut process_list: Vec<ProcessInfo> = get_process_info();

    process_list.sort_by(|a, b| {
        let a_cpu = a.cpu_usage.parse::<u64>().unwrap_or(0);
        let b_cpu = b.cpu_usage.parse::<u64>().unwrap_or(0);
        b_cpu.cmp(&a_cpu)
    });

    process_list
}

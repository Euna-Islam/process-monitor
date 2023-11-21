extern crate sysinfo;

use sysinfo::{ProcessExt, System, SystemExt};
use process_monitor::models::process_info::ProcessInfo;

pub fn get_process_info() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut process_list: Vec<ProcessInfo> = sys.processes().iter()
    .map(|(pid, process)| {
        ProcessInfo {
            name: process.name().to_string(),
            id: pid.to_string(),
            cpu_usage: process.cpu_usage().to_string(),
            status: process.status().to_string(),
            memory_usage: process.memory().to_string()
        }
    })
    .collect();

    process_list.sort_by(|a, b| {
        let a_mem = a.memory_usage.parse::<u64>().unwrap_or(0);
        let b_mem = b.memory_usage.parse::<u64>().unwrap_or(0);
        a_mem.cmp(&b_mem)
    });

    process_list
}

pub fn get_expensive_processes() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut process_list: Vec<ProcessInfo> = sys.processes().iter()
    .map(|(pid, process)| {
        ProcessInfo {
            name: process.name().to_string(),
            id: pid.to_string(),
            cpu_usage: process.cpu_usage().to_string(),
            status: process.status().to_string(),
            memory_usage: process.memory().to_string()
        }
    })
    .collect();

    process_list.sort_by(|a, b| {
        let a_mem = a.memory_usage.parse::<u64>().unwrap_or(0);
        let b_mem = b.memory_usage.parse::<u64>().unwrap_or(0);
        a_mem.cmp(&b_mem)
    });

    let (a, _b) = process_list.split_at(5);

    // Print the first five elements
    println!("Testing {:?}", a.len());

    process_list
}
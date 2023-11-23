/* SPDX-License-Identifier: Apache-2.0 */

//import external crate for system information
extern crate sysinfo;

//import custom struct ProcessInfo 
use process_monitor::models::process_info::ProcessInfo;

//import needed traits from external crate sysinfo
use sysinfo::{ProcessExt, System, SystemExt};

/*
 *Get process information
 *Map it to Vector of ProcessInfo
 */
pub fn get_process_info() -> Vec<ProcessInfo> {
    let mut sys = System::new_all();
    //refreshing twice for accurate data about CPU
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

/*
 * get process info
 * sort them by memory usage
 */
pub fn sort_by_memory() -> Vec<ProcessInfo> {
    let mut process_list: Vec<ProcessInfo> = get_process_info();

    process_list.sort_by(|a, b| {
        let mem1 = a.memory_usage.parse::<u64>().unwrap_or(0);
        let mem2 = b.memory_usage.parse::<u64>().unwrap_or(0);
        mem2.cmp(&mem1)
    });

    process_list
}

/*
 * get process info
 * sort them by cpu usage
 */
pub fn sort_by_cpu_usage() -> Vec<ProcessInfo> {
    let mut process_list: Vec<ProcessInfo> = get_process_info();

    process_list.sort_by(|a, b| {
        let cpu1 = a.cpu_usage.parse::<u64>().unwrap_or(0);
        let cpu2 = b.cpu_usage.parse::<u64>().unwrap_or(0);
        cpu2.cmp(&cpu1)
    });

    process_list
}

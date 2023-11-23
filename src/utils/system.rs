/* SPDX-License-Identifier: Apache-2.0 */

//importing external crate
extern crate sysinfo;

//import custom struct SystemInfo
use process_monitor::models::system_info::SystemInfo;
//import needed traits from external crate
use sysinfo::{System, SystemExt};

/*
 * Get system info
 */
pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let sys_name = sys.name().expect("Could not get system name").to_string();
    let sys_host = sys
        .host_name()
        .expect("Could not get host name")
        .to_string();
    SystemInfo {
        host: sys_host,
        name: sys_name,
    }
}

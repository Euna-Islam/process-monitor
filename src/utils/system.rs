extern crate sysinfo;

use sysinfo::{System, SystemExt};
use process_monitor::models::system_info::SystemInfo;

pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let sys_name = sys.name().expect("Could not get system name").to_string();
    let sys_host = sys.host_name().expect("Could not get host name").to_string();
    SystemInfo {
        host: sys_host,
        name: sys_name
    }
}

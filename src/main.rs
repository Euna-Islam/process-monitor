extern crate sysinfo;
extern crate warp;

use sysinfo::{ProcessExt, System, SystemExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET disk_info_route => 200 OK with body "Hello, warp!"
    let disk_info_route = warp::path!("process-monitor")
        .map(|| warp::reply::html(get_system_info()));

    // Start the warp server
    warp::serve(disk_info_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

// Function to get disk information
fn get_system_info() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();

    let sys_name = sys.name().expect("Could not get system name").to_string();
    let sys_host = sys.host_name().expect("Could not get host name").to_string();

    let mut sys_info = "System Information<br>.....................".to_string();

    sys_info.push_str("<br>System Name : ");
    sys_info.push_str(&sys_name);

    sys_info.push_str("<br>Host Name : ");
    sys_info.push_str(&sys_host);

    sys_info
}

// fn get_system_data() {
//     let mut sys = System::new_all();

//     // First we update all information of our `System` struct.
//     sys.refresh_all();

//     // We display all disks' information:
//     println!("=> disks:");
//     for disk in sys.disks() {
//         println!("{:?}", disk);
//     }

//     println!("=> system:");
//     // RAM and swap information:
//     println!("total memory: {} bytes", sys.total_memory());
//     println!("used memory : {} bytes", sys.used_memory());
//     println!("total swap  : {} bytes", sys.total_swap());
//     println!("used swap   : {} bytes", sys.used_swap());

//     // Display system information:
//     println!("System name:             {:?}", sys.name());
//     println!("System kernel version:   {:?}", sys.kernel_version());
//     println!("System OS version:       {:?}", sys.os_version());
//     println!("System host name:        {:?}", sys.host_name());

//     // Number of CPUs:
//     println!("NB CPUs: {}", sys.cpus().len());

//     // Display processes ID, name na disk usage:
//     for (pid, process) in sys.processes() {
//         println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
//     }
// }

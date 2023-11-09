extern crate sysinfo;
extern crate warp;

//use sysinfo::{ProcessExt, System, SystemExt};
use warp::Filter;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
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

extern crate sysinfo;
extern crate warp;

use sysinfo::{ProcessExt, System, SystemExt};
use warp::Filter;

#[tokio::main]
async fn main() {
   // GET process_monitor_route => 200 OK with body containing system info
   let process_monitor_route = warp::path!("process-monitor")
   .map(|| warp::reply::html(get_system_info()));

    // GET other_page_route => 200 OK with body containing other info
    let other_page_route = warp::path!("cpu")
    .map(|| warp::reply::html(get_other_info()));

    // Combine routes
    let routes = process_monitor_route.or(other_page_route);

    // Start the warp server
    warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
}

// Function to get system information
fn get_system_info() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();

    let sys_name = sys.name().expect("Could not get system name").to_string();
    let sys_host = sys.host_name().expect("Could not get host name").to_string();


    format!(
        r#"
        <html>
        <body>
            <!-- Link to other_page -->
            <a href="/cpu" target="_blank">CPU</a>
            <h1>System Information</h1>
            <p>System Name: {sys_name}</p>
            <p>Host Name: {sys_host}</p>  
        </body>
        </html>
        "#
    )
}

// Function to get information for other page
fn get_other_info() -> String {
    format!(
        r#"
        <html>
        <body>
            <h1>Other Page Information</h1>
            <p>Some other information here...</p>
        </body>
        </html>
        "#
    )
}
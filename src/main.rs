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
    let cpu_route = warp::path!("cpu")
    .map(|| warp::reply::html(get_cpu_info()));

    // GET other_page_route => 200 OK with body containing other info
    let process_route = warp::path!("process")
    .map(|| warp::reply::html(get_process_info()));

    // Combine routes
    let routes = process_monitor_route.or(cpu_route).or(process_route);

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
            <a href="/process" target="_blank">Process</a>
            <h1>System Information</h1>
            <p>System Name: {sys_name}</p>
            <p>Host Name: {sys_host}</p>  
        </body>
        </html>
        "#
    )
}

// Function to get cpu information for other page
fn get_cpu_info() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();
    let cpu = sys.cpus().len().to_string();
    println!("NB CPUs: {}", &sys.cpus().len());


    format!(
        r#"
        <html>
        <body>
            <h1>CPU Information</h1>
            <p>No of CPUs: {cpu}</p>
        </body>
        </html>
        "#
    )
}

// Function to get cpu information for other page
fn get_process_info() -> String {
    let mut sys = System::new_all();

    sys.refresh_all();
    println!("Refreshed!");
    // Display processes ID, name na disk usage:
    // for (pid, process) in sys.processes() {
    //     println!("[{}] {} {:?} {} {}%", pid, process.name(), process.disk_usage(), process.status(), process.cpu_usage());
    // }

    // Create a table header
    let table_header = "<tr><th>PID</th><th>Name</th><th>Disk Usage</th><th>Status</th><th>CPU Usage</th></tr>";

    // Create table rows for each process
    let table_rows: String = sys.processes().iter()
        .map(|(pid, process)| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{:?}</td><td>{}</td><td>{:.2}%</td></tr>",
                pid,
                process.name(),
                process.disk_usage(),
                process.status(),
                process.cpu_usage()
            )
        })
        .collect();

    // Format the HTML with the table
    format!(
        r#"
        <html>
        <header><meta http-equiv="refresh" content="30"></header>
        <body>
            <h1>Process Information</h1>
            <table border="1">
                {table_header}<br>
                {table_rows}
            </table>
        </body>
        </html>
        "#
    )
}
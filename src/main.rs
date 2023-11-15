mod utils{
    pub mod system;
    pub mod process;
}

use utils::system::{get_system_info, SystemInfo};
use utils::process::{get_process_info, ProcessInfo};

fn main() {
    print_system_info();
    print_process_info();
}

fn print_process_info() {
    let process_info: Vec<ProcessInfo> = get_process_info();

    println!("No of processes : {}", process_info.len());
}

fn print_system_info() {
    let system_info: SystemInfo = get_system_info();

    println!("System Host : {} ", system_info.host);
    println!("System Name : {} ", system_info.name);
}
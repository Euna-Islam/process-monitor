mod utils{
    pub mod system;
    pub mod process;
}

mod app_args;

use std::env;
use process_monitor::models::system_info::SystemInfo;
use process_monitor::models::process_info::ProcessInfo;

use utils::system::get_system_info;
use utils::process::get_process_info;

fn main() {
    take_cmd_line_arg();
}

fn take_cmd_line_arg() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match args[1].as_str() {
            app_args::SYSTEM_ARG => print_system_info(),
            app_args::PROCESS_ARG => print_process_info(),
            app_args::HELP_ARG => println!("Not implemented yet!"),
            _ => println!("Invalid argument. Use --system, or --process."),
        }
    } else {
        println!("Please provide an argument: --system, or --process.");
    }
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
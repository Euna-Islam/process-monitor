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

    match args.len() {
        2 => {
            let result = process_arg(&args);
            match result {
                Ok(_) => println!("Operation successful"),
                Err(err) => eprintln!("Error processing input: {}", err),
            }
        },
        _ => {
            eprintln!("Error handling arguments. Please use -- manual");
        }

    }
}

fn process_arg(args: &Vec<String>) -> Result<(), &'static str> {
    match args[1].as_str() {
        app_args::SYSTEM_ARG => {
            print_system_info();
            Ok(())
        },
        app_args::PROCESS_ARG => {
            print_process_info();
            Ok(())
        },
        app_args::HELP_ARG => {
            print_help_file();
            Ok(())
        },
        _ => Err("Invalid argument. Use --help, --system, or --process."),
    }
}

fn print_help_file() {
    let result = std::fs::read_to_string("help.txt");
    match result {
        Ok(content) => { println!("{}", content); }
        Err(error) => { eprintln!("Internal error! Error: {}", error); }
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
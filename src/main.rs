mod utils{
    pub mod system;
    pub mod process;
}

mod app_args;

use clap::Parser;

use std::env;
use process_monitor::models::system_info::SystemInfo;
use process_monitor::models::process_info::ProcessInfo;

use utils::system::get_system_info;
use utils::process::get_process_info;

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    take_cmd_line_arg();
}

fn take_cmd_line_arg() {

    let args: Cli = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    let my_c_instance = C {}; // Creating an instance of struct C

    my_c_instance.print(&args);
    // let args: Vec<String> = env::args().collect();

    // if args.len() > 1 {
    //     match args[1].as_str() {
    //         app_args::SYSTEM_ARG => print_system_info(),
    //         app_args::PROCESS_ARG => print_process_info(),
    //         app_args::HELP_ARG => println!("Not implemented yet!"),
    //         _ => println!("Invalid argument. Use --system, or --process."),
    //     }
    // } else {
    //     println!("Please provide an argument: --system, or --process.");
    // }
}

struct C {}

impl C {
    // Method that accepts an instance of struct X
    fn print(&self, x: &Cli) {
        // Print or perform operations on the received X instance
        println!("Received data: {:?}", x.pattern);
        println!("Received file path: {:?}", x.path);
        // Additional processing or operations can be performed here
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
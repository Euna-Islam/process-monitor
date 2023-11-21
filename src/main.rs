mod utils{
    pub mod system;
    pub mod process;
}

mod app_args;

use std::io::{self, Write};
use std::process;
use std::env;

use process_monitor::models::system_info::SystemInfo;
use process_monitor::models::process_info::ProcessInfo;

use utils::process::get_expensive_processes;
use utils::system::get_system_info;
use utils::process::get_process_info;


fn main() {
    take_cmd_line_arg();
    process::exit(exitcode::OK);
}

fn read_config() {
    let content = read_file(app_args::INTRO_FILE_PATH);
    match content {
        Ok(_) => {
            start_app();
        },
        Err(_) => {eprint!("testing error")}
    }
}

fn start_app() {
    loop {
        print!("Enter command: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim();

        match input {
            "exit" => {
                println!("Exiting the program.");
                break;
            },
            "0" => {
                
                let content = read_file(app_args::HELP_FILE_PATH);
                match content {
                    Err(_) => {eprint!("testing error")},
                    _ => {}
                }
            },
            "1" => {
                print_system_info();
            },
            "2" => {
                print_process_info();
            },
            "3" => {
                get_memory_heavy_process();
            }
            _ => println!("Invalid command. Please try again or type 'exit' to quit."),
        }
    }
}



fn read_file(file_path: &str) -> Result<(), &'static str> {
    let file_content = std::fs::read_to_string(file_path);
    match file_content {
        Ok(content) => {
                                    println!("{}", content);
                                    Ok(())
                                }
        Err(_error) => {
                                Err("Could not open start file")
                             }
    }
}

fn take_cmd_line_arg() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            read_config();
        },
        2 => {
            let result = process_arg(&args);
            match result {
                Ok(_) => println!("Operation successful"),
                Err(err) => eprintln!(" {}", err),
            }
        },
        _ => {
            eprintln!("Error handling arguments. Please use -- manual");
            process::exit(exitcode::USAGE);
        }

    }
}

fn process_arg(args: &Vec<String>) -> Result<(), &'static str> {
    match args[1].as_str() {
        app_args::HELP_ARG => {
            print_help_file();
            Ok(())
        }
        _ => Err("Invalid argument. Use --help, --system, or --process.")
    }
}

fn print_help_file() {
    let result = std::fs::read_to_string(app_args::MAN_FILE_PATH);
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

fn get_memory_heavy_process() {
    let process_info: Vec<ProcessInfo> = get_expensive_processes();

    println!("No of processes : {}", process_info.len());
}


mod utils {
    pub mod process;
    pub mod system;
}

mod app_args;

use std::env;
use std::io::{self, Write};
use std::process;

use process_monitor::models::process_info::ProcessInfo;
use process_monitor::models::system_info::SystemInfo;

use utils::process::{get_process_info, sort_by_cpu_usage, sort_by_memory};
use utils::system::get_system_info;

fn main() {
    take_cmd_line_arg();
}

fn take_cmd_line_arg() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            read_config();
        }
        2 => {
            process_arg(&args);
        }
        _ => {
            handle_argument_error();
        }
    }
}

fn read_config() {
    let content = read_file(app_args::INTRO_FILE_PATH);
    match content {
        Ok(_) => {
            run_app();
        }
        Err(_) => handle_file_error(),
    }
}

fn process_arg(args: &Vec<String>) {
    match args[1].as_str() {
        app_args::HELP_ARG => {
            let content = read_file(app_args::MAN_FILE_PATH);
            match content {
                Err(_) => handle_file_error(),
                _ => {}
            }
        }
        _ => {
            handle_argument_error();
        }
    }
}

fn run_app() {
    loop {
        print_info("Enter command: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();

        match input {
            "exit" => {
                print_info("Exiting the program.");
                break;
            }
            "0" => {
                let content = read_file(app_args::HELP_FILE_PATH);
                match content {
                    Err(_) => handle_file_error(),
                    _ => {}
                }
            }
            "1" => {
                let system_info: SystemInfo = get_system_info();
                print_info(&system_info.to_string());
            }
            "2" => {
                let process_info: Vec<ProcessInfo> = get_process_info();
                print_processes(process_info);
            }
            "3" => {
                let process_info: Vec<ProcessInfo> = sort_by_memory();
                print_processes(process_info);
            }
            "4" => {
                let process_info: Vec<ProcessInfo> = sort_by_cpu_usage();
                print_processes(process_info);
            }
            _ => {
                print_error("Invalid command. Please use help.");
                process::exit(exitcode::USAGE);
            }
        }
    }

    process::exit(exitcode::OK);
}

fn read_file(file_path: &str) -> Result<(), &'static str> {
    let file_content = std::fs::read_to_string(file_path);
    match file_content {
        Ok(content) => {
            print_info(&content);
            Ok(())
        }
        Err(_error) => Err("Internal Error. Could not open start file."),
    }
}

fn print_processes(process_info: Vec<ProcessInfo>) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    if let Err(err) = writeln!(
        handle,
        "{:<50} {:<5} {:<10} {:<10} {:<10}",
        "Name", "Id", "CPU", "Memory", "Status"
    ) {
        print_error(&err.to_string())
    };
    for item in process_info {
        if let Err(err) = writeln!(handle, "{}", item) {
            print_error(&err.to_string())
        };
    }
}

fn handle_argument_error() {
    print_error(app_args::ARGUMENT_ERROR);
    process::exit(exitcode::USAGE);
}

fn handle_file_error() {
    print_error(app_args::FILE_ERROR);
    process::exit(exitcode::CONFIG);
}

fn print_error(err: &str) {
    eprintln!("{}", &err);
}

fn print_info(content: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if let Err(err) = writeln!(handle, "{}", &content) {
        print_error(&err.to_string())
    };
}

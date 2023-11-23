/* SPDX-License-Identifier: Apache-2.0 */

//import module util
mod utils {
    pub mod process;
    pub mod system;
}

//import module app_arguments
mod app_args;

//import  module from standard library
use std::env;
use std::io::{self, Write};
use std::process;

//import custom structs
use process_monitor::models::process_info::ProcessInfo;
use process_monitor::models::system_info::SystemInfo;

//import functions from modules utils
use utils::process::{get_process_info, sort_by_cpu_usage, sort_by_memory};
use utils::system::get_system_info;

/*Execution starts */
fn main() {
    take_cmd_line_arg();
}

/*Take argument from user */
fn take_cmd_line_arg() {
    let cmd_args: Vec<String> = env::args().collect();
    match cmd_args.len() {
        1 => {
            //if the user did not pass an argument start the app
            read_config();
        }
        2 => {
            //if the user gives an argument, process it
            process_arg(&cmd_args);
        }
        _ => {
            //if the user gives multiple argument throw error
            handle_argument_error();
        }
    }
}

fn read_config() {
    //read intro file
    let content = read_file(app_args::INTRO_FILE_PATH);
    match content {
        Ok(_) => {
            //on successful read, start the app
            run_app();
        }
        Err(_) => handle_file_error(), //on failure, handle file error
    }
}

/*
 * Process argument received from user
 * takes args
 */
fn process_arg(args: &Vec<String>) {
    match args[1].as_str() {
        app_args::HELP_ARG => {
            //if the user sent help argument
            //read the help file and process it
            let content = read_file(app_args::MAN_FILE_PATH);
            match content {
                Err(_) => handle_file_error(),
                _ => {}
            }
        }
        _ => {
            handle_argument_error(); //if the argument is not recognized, handle argument error
        }
    }
}

/*
 * On successful reading of intro file
 * app starts the main loop
 * where it waits for user's command and
 * process it
 */
fn run_app() {
    loop {
        print_info("Enter command: ");

        //empty the stdout
        io::stdout().flush().unwrap();

        let mut input = String::new(); //variable to hold user input
                                       //get input
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading command from user");
        //remove white spaces from front and tail of string
        let input = input.trim();

        //match user's input
        match input {
            "exit" => {
                //quit app
                print_info("Exiting the program.");
                break;
            }
            "0" => {
                //print manual
                let content = read_file(app_args::HELP_FILE_PATH);
                match content {
                    Err(_) => handle_file_error(),
                    _ => {}
                }
            }
            "1" => {
                //print system info
                let system_info: SystemInfo = get_system_info();
                print_info(&system_info.to_string());
            }
            "2" => {
                //print process info
                let process_info: Vec<ProcessInfo> = get_process_info();
                print_processes(process_info);
            }
            "3" => {
                //sort process info by memory usage
                let process_info: Vec<ProcessInfo> = sort_by_memory();
                print_processes(process_info);
            }
            "4" => {
                //sort process info by cpu usage
                let process_info: Vec<ProcessInfo> = sort_by_cpu_usage();
                print_processes(process_info);
            }
            _ => {
                //handle invalid input from user
                print_error("Invalid command. Please use help.");
                process::exit(exitcode::USAGE);
            }
        }
    }

    process::exit(exitcode::OK); //successful exit
}

/*
 * Reads a file and prints its content
 * takes file path
 * Returns success or error
 */
fn read_file(file_path: &str) -> Result<(), &'static str> {
    let file_content = std::fs::read_to_string(file_path);
    match file_content {
        Ok(content) => {
            print_info(&content);
            Ok(())
        }
        Err(_error) => Err("Internal Error. Could not open file."),
    }
}

/*
 *formats and prints process info
 *takes a vector of ProcessInfo
 */
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

/*
 *handles errors with user argument
 *prints the error
 *exits the app
 */
fn handle_argument_error() {
    print_error(app_args::ARGUMENT_ERROR);
    process::exit(exitcode::USAGE);
}
/*
 *handles errors with reading files
 *prints the error
 *exits the app
 */
fn handle_file_error() {
    print_error(app_args::FILE_ERROR);
    process::exit(exitcode::CONFIG);
}

/*
 *prints to std error
 *takes string
 */
fn print_error(err: &str) {
    eprintln!("{}", &err);
}
/*
 * prints to std out
 * takes string
 */
fn print_info(content: &str) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    if let Err(err) = writeln!(handle, "{}", &content) {
        print_error(&err.to_string())
    };
}

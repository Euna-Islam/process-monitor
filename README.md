# process-monitor

## Features
1. Read system information
2. Read the process list
3. Sort process by memory and CPU usage

## Language
Rust

## Operating System
Windows

## How to build
1. Run Powershell script ".\build.ps1"
2. In the "target/debug" folder .exe file will be generated
3. Needed files will be copied to the same folder

## How to use
1. Use the command "cargo run --manual" if you want to see help
2. Use the command "cargo run" to start the app
3. Once the app starts, you can select from options to see the system and process data Or exit the app
4. If you build the app using the provided PowerShell script<br>
   - browse to the "target/debug" folder<br>
   - run ".\process_monitor.exe"
## References
[1] [Learn Rust Programming - Complete Course. FreeCodeCamp.](https://www.youtube.com/watch?v=BpPEoZW5IiY)<br>
[2] [Crate sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)<br>
[3] [Rust Documentation](https://doc.rust-lang.org/book/title-page.html)<br>
# process-monitor

## Features
1. Read system information
2. Read the process list
3. Sort process by memory and CPU usage

## Language
Rust<br>
Version: 1.73.0 

## Operating System
Windows

## How to build
1. Run Powershell script ".\build.ps1"
2. In the "target/release" folder .exe file will be generated
3. Needed files will be copied to the same folder

## How to use
Build the app using the provided PowerShell script<br>
   - browse to the "target/release" folder<br>
   - run ".\process_monitor.exe"

## Example Usage
![Usage](img/usage.png?raw=true "Usage")

## License
Apache-2.0

## References
[1] [Learn Rust Programming - Complete Course. FreeCodeCamp.](https://www.youtube.com/watch?v=BpPEoZW5IiY)<br>
[2] [Crate sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)<br>
[3] [Rust Documentation](https://doc.rust-lang.org/book/title-page.html)<br>
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
[3] [Rust Iterator Trait](https://doc.rust-lang.org/std/iter/trait.Iterator.html)<br>
[4] [Rust Program Arguments](https://doc.rust-lang.org/rust-by-example/std_misc/arg.html)<br>
[5] [Rust Flow of Control - Match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)<br>
[6] [Rust Display Trait](https://doc.rust-lang.org/std/fmt/trait.Display.html)<br>

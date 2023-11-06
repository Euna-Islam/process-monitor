use std::process::Command;

fn main() {
    list_processes();
}

fn list_processes() {
    // Run a command to list all processes (e.g., using the `ps` command on Unix-like systems).
    let output = Command::new("ps")
    .arg("aux")
    .output()
    .expect("Failed to execute command");

    // Print the output of the command (list of processes).
    println!("Process List:\n{}", String::from_utf8_lossy(&output.stdout));
}

mod utils{
    pub mod system;
}

use utils::system::{get_system_info, SystemInfo};

fn main() {
    print_system_info();
}

fn print_system_info() {
    let system_info: SystemInfo = get_system_info();

    println!("System Host : {} ", system_info.host);
    println!("System Name : {} ", system_info.name);
}
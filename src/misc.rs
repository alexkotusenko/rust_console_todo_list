use std::process::Command;

pub fn clear_console() {
    if cfg!(target_os = "windows") { // only one = sign
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Failed to clear console.");
    } else {
        Command::new("clear")
            .status()
            .expect("Failed to clear console");
    }
}

pub fn print_header() {
    println!(r" _____ ___________ _____ _     _____ _____ _____ 
|_   _|  _  |  _  \  _  | |   |_   _/  ___|_   _|
  | | | | | | | | | | | | |     | | \ `--.  | |  
  | | | | | | | | | | | | |     | |  `--. \ | |  
  | | \ \_/ / |/ /\ \_/ / |_____| |_/\__/ / | |  
  \_/  \___/|___/  \___/\_____/\___/\____/  \_/  
                                                 
                                                 ");
}

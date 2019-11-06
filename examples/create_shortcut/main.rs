extern crate powershell_script;
use std::io::{stdin, Read};

/// Creates a shortcut to notpad on the desktop
fn main() {
    let create_shortcut = include_str!("script.ps");
    match powershell_script::run(create_shortcut, true) {
        Ok(output) => {
            println!("{}", output);
            println!("Press ENTER to continue...");
            stdin().read(&mut [0]).unwrap();
        }

        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

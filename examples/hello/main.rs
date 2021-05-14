extern crate powershell_script;

/// Print 'Hello' to console.
fn main() {
    let hello = include_str!("script.ps1");
    match powershell_script::run(hello, false) {
        Ok(output) => {
            eprint!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

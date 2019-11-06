//! # Windows Powershell script runner
//! 
//! This crate is pretty basic. It uses `std::process::Command` to pipe commands
//! to PowerShell. In addition to that there is a convenient wrapper around `process::Output`
//! especially tailored towards the usecase of running Windows PowerShell commands.
//! 
//! ## Example
//! 
//! I recommend that you write the commands to a `*.ps` file to be able to take advantage
//! of existing tools to create the script.
//! 
//! This example creates a shortcut of `notepad.exe` to the desktop.
//! 
//! _NB. If you use OneDrive chances are that your desktop is located at "$env:UserProfile\OneDrive\Desktop\notepad.lnk" instead._
//! 
//! **In `script.ps`**
//! ```ps
//! $SourceFileLocation="C:\Windows\notepad.exe"
//! $ShortcutLocation="$env:UserProfile\Desktop\notepad.lnk"
//! $WScriptShell=New-Object -ComObject WScript.Shell
//! $Shortcut=$WScriptShell.CreateShortcut($ShortcutLocation)
//! $Shortcut.TargetPath=$SourceFileLocation
//! $Shortcut.Save()
//! ```
//! 
//! **In `main.rs`**
//! ```rust
//! extern crate powershell_script;
//! use std::io::{stdin, Read};
//! 
//! // Creates a shortcut to notpad on the desktop
//! fn main() {
//!     let create_shortcut = include_str!("script.ps");
//!     match powershell_script::run(create_shortcut, true) {
//!         Ok(output) => {
//!             println!("{}", output);
//!         }
//!         Err(e) => {
//!             println!("Error: {}", e);
//!         }
//!     }
//! }
//! ```
//! 
//! You can of course provide the commands as a string literal instead. Just beware that
//! we run each `line` as a separate command.
//! 
//! ## Compatability
//! 
//! This is only tested on Windows and most likely will only work on Windows. It should
//! be possible to support PowerShell Core on Linux with only minor adjustments so leave
//! a feature request if there is any interest in that.
//! 


use std::process::{Stdio, Command, Output as ProcessOutput};
use std::io::{self, Write};
use std::fmt;

type Result<T> = std::result::Result<T, PsError>;

/// Runs the script and returns an instance of `std::process::Output` on
/// success.
/// 
/// ## Panics
/// If there is an error retrieving a handle to `stdin` in the child process.
pub fn run_raw(script: &str, print_commands: bool) -> Result<ProcessOutput> {
    let mut cmd = Command::new("PowerShell");
    cmd.stdin(Stdio::piped());
    let mut process = cmd.args(&["-Command", "-"]).spawn()?;
    let stdin = process.stdin.as_mut().expect("Pipe failure");
    
    for line in script.lines() {
        if print_commands { println!("{}", line) };
        writeln!(stdin, "{}", line)?;
    }
    
    let output = process.wait_with_output()?;

    Ok(output)
}

/// Runs a script in PowerShell. Returns an instance of `Output`. In the case of
/// a failure when running the script it returns an `PsError::Powershell(Output)`
/// which holds the output object containing the captures of `stderr` and `stdout`
/// for display.
/// 
/// ## Panics
/// If there is an error retrieving a handle to `stdin` in the child process.
pub fn run(script: &str, print_commands: bool) -> Result<Output> {
    let proc_output = run_raw(script, print_commands)?;

    let output = Output::from(proc_output);
    if output.success {
        Ok(output)
    } else {
        Err(PsError::Powershell(output))
    }
    
}

#[derive(Debug, Clone)]
pub struct Output {
    success: bool,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl Output {
    /// Returns the parsed output of the `stdout` capture of the child process
    pub fn stdout(&self) -> Option<&str> {
        self.stdout.as_ref().map(|s| s.as_str())
    }

    /// Returns the parsed output of the `stdout` capture of the child process
    pub fn stderr(&self) -> Option<&str> {
        self.stderr.as_ref().map(|s| s.as_str())
    }
}

impl From<ProcessOutput> for Output {
    fn from(proc_output: ProcessOutput) -> Output {
        let stdout = if proc_output.stdout.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(&proc_output.stdout).to_string())
        };

        let stderr = if proc_output.stderr.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(&proc_output.stderr).to_string())
        };

        Output {
            success: proc_output.status.success(),
            stdout,
            stderr,
        }

    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(stdout) = self.stdout().as_ref() {
            write!(f, "{}", stdout)?;
        }

        if let Some(stderr) = self.stderr().as_ref() {
            write!(f, "{}", stderr)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum PsError {
    /// An error in the PowerShell script.
    Powershell(Output),
    /// An I/O error related to the child process.
    Io(io::Error),
}

impl std::error::Error for PsError { }

impl fmt::Display for PsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsError::*;
        match self {
            Powershell(out) => write!(f, "{}", out)?,
            Io(e) => write!(f, "{}", e)?,
        }
        Ok(())
    }
}

impl From<io::Error> for PsError {
    fn from(io: io::Error) -> PsError {
        PsError::Io(io)
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

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
//! _NB. If you use OneDrive chances are that your desktop is located at "$env:UserProfile\OneDrive\Desktop\" instead._
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
//! ```rust, ignore
//! use crate powershell_script;
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
//! The flag `print_commands` can be set to `true` if you want each
//! command to be printed to the `stdout` of the main process as they're run which
//! can be useful for debugging scripts or displaying the progress.
//!
//! ## Features and compatability
//!
//! On Windows it defaults to using the PowerShell which ships with Windows, but you
//! can also run scripts using PowerShell Core by enabling the `core` feature.
//!
//! On all other operating systems it will run scripts using PowerShell core.
//!

use std::{fmt, env};
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Output as ProcessOutput, Stdio};
use std::os::windows::process::CommandExt;

type Result<T> = std::result::Result<T, PsError>;

/// Runs the script and returns an instance of `std::process::Output` on
/// success. The flag `print_commands` can be set to `true` if you want each
/// command to be printed to the `stdout` of the main process as they're run.
///
/// ## Panics
/// If there is an error retrieving a handle to `stdin` in the child process.
pub fn run_raw(script: &str, print_commands: bool) -> Result<ProcessOutput> {
    let mut cmd = Command::new(get_powershell_path()?);

    cmd.stdin(Stdio::piped());
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());

    let os = std::env::consts::OS;
    const CREATE_NO_WINDOW: u32 = 0x08000000;
    let mut process =
        if os == "windows" {
            cmd.args(&["-NoProfile", "-Command", "-"]).creation_flags(CREATE_NO_WINDOW).spawn()?
        } else {
            cmd.args(&["-NoProfile", "-Command", "-"]).spawn()?
        };
    
    let stdin = process.stdin.as_mut().ok_or(PsError::ChildStdinNotFound)?;

    for line in script.lines() {
        if print_commands {
            println!("{}", line)
        };
        writeln!(stdin, "{}", line)?;
    }

    let output = process.wait_with_output()?;

    Ok(output)
}

/// Runs a script in PowerShell. Returns an instance of `Output`. In the case of
/// a failure when running the script it returns an `PsError::Powershell(Output)`
/// which holds the output object containing the captures of `stderr` and `stdout`
/// for display. The flag `print_commands` can be set to `true` if you want each
/// command to be printed to the `stdout` of the main process as they're run. Useful
/// for debugging scripts.
///
/// ## Panics
/// If there is an error retrieving a handle to `stdin` in the child process.
///
/// ## Example
///
/// ```rust
/// fn main() {
///     let script = r#"echo "hello world""#;
///     let output = powershell_script::run(script, false).unwrap();
///     assert_eq!(output.stdout().unwrap().trim(), "hello world");
/// }
/// ```
pub fn run(script: &str, print_commands: bool) -> Result<Output> {
    let proc_output = run_raw(script, print_commands)?;

    let output = Output::from(proc_output);
    if output.success {
        Ok(output)
    } else {
        Err(PsError::Powershell(output))
    }
}

fn get_powershell_path() -> Result<String> {
    // Get name of a script 

    #[cfg(all(not(feature = "core"), windows))]
    // Windows PowerShell
    let powershell_name = "PowerShell";

    #[cfg(any(feature = "core", not(windows)))]
    // PowerShell Core
    let powershell_name = "pwsh";

    // Preferred option: use the powershell installation that is on path
    if is_program_on_path(powershell_name).unwrap() {
        return Ok(powershell_name.to_string())
    }

    // Backup option for windows, because cmd apparently ignores powershell on path: Try powershell's default installation path
    if std::env::consts::OS == "windows" {
        let system_root = match env::var("SYSTEMROOT")  {
            Ok(x) => x,
            Err(_e) => return Err(PsError::PowershellNotFound)
        };
        let path_candidate = Path::new(&system_root).join(r#"System32\WindowsPowerShell\v1.0\powershell.exe"#);
        return match path_candidate.exists() {
            true => Ok(path_candidate.to_string_lossy().to_string()),
            false => Err(PsError::PowershellNotFound)
        }
    }

    // If none of the above worked then we must give up
    return Err(PsError::PowershellNotFound);
}

fn is_program_on_path(program_name: &str) -> Option<bool> {
    // Check whether there is a program called "program name" on the system path
    let path_splitter = match std::env::consts::OS {
        "windows" => ';',
        _ => ':'
    };
    let system_path = match env::var("PATH")  {
        Ok(x) => x,
        Err(_e) => return None
    };
    for path_dir in system_path.split(path_splitter) {
        let path = std::path::Path::new(path_dir).join(&program_name);
        if path.exists() {
            return Some(true);
        }
    }
    return Some(false);
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
    // Failed to find PowerShell in this system
    PowershellNotFound,
    /// Failed to retrieve a handle to `stdin` for the child process
    ChildStdinNotFound,
}

impl std::error::Error for PsError {}

impl fmt::Display for PsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PsError::*;
        match self {
            Powershell(out) => write!(f, "{}", out)?,
            Io(e) => write!(f, "{}", e)?,
            PowershellNotFound => write!(f, "Failed to find powershell on this system")?,
            ChildStdinNotFound => write!(f, "Failed to acquire a handle to stdin in the child process.")?,
        }
        Ok(())
    }
}

impl From<io::Error> for PsError {
    fn from(io: io::Error) -> PsError {
        PsError::Io(io)
    }
}

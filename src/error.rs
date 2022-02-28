use std::fmt;
use std::io;

use crate::output::Output;

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

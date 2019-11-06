use std::process::{Stdio, Command, Output as ProcessOutput};
use std::io::{self, Write};
use std::fmt;

type Result<T> = std::result::Result<T, PsError>;

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
    pub fn stdout(&self) -> Option<&str> {
        self.stdout.as_ref().map(|s| s.as_str())
    }

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
    Powershell(Output),
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

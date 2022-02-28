use std::{
    env,
    io::Write,
    process::{self, Command, Stdio},
};

use crate::{error::PsError, output::Output, Result, POWERSHELL_NAME};

const PATH_SPLITTER: char = ':';

pub struct PsScript {
    pub(crate) args: Vec<&'static str>,
    pub(crate) hidden: bool,
    pub(crate) print_commands: bool,
}

impl PsScript {
    pub fn run(&self, script: &str) -> Result<Output> {
        let proc_output = self.run_raw(script)?;

        let output = Output::from(proc_output);
        if output.success {
            Ok(output)
        } else {
            Err(PsError::Powershell(output))
        }
    }

    fn run_raw(&self, script: &str) -> Result<process::Output> {
        let mut cmd = Command::new(get_powershell_path()?);

        cmd.stdin(Stdio::piped());
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());

        cmd.args(&self.args);

        if self.hidden {
            // TODO: Check if this is a problem in PS Core on Unix platforms
            // See: https://github.com/cfsamson/powershell-script/pull/9
        }

        let mut process = cmd.spawn()?;
        let stdin = process.stdin.as_mut().ok_or(PsError::ChildStdinNotFound)?;

        for line in script.lines() {
            if self.print_commands {
                println!("{}", line)
            };
            writeln!(stdin, "{}", line)?;
        }

        let output = process.wait_with_output()?;
        Ok(output)
    }
}

/// Check whether there is a program called "program name" on the system path
fn is_program_on_path(program_name: &str) -> Option<bool> {
    let system_path = match env::var("PATH") {
        Ok(x) => x,
        Err(_e) => return None,
    };

    for path_dir in system_path.split(PATH_SPLITTER) {
        let path = std::path::Path::new(path_dir).join(&program_name);
        if path.exists() {
            return Some(true);
        }
    }
    return Some(false);
}

fn get_powershell_path() -> Result<String> {
    if is_program_on_path(POWERSHELL_NAME).unwrap() {
        Ok(POWERSHELL_NAME.to_string())
    } else {
        Err(PsError::PowershellNotFound)
    }
}

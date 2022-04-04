use std::{process, fmt};

/// A convenient wrapper around `process::Output` which indicates if the
/// script ran successfully or not and gives easy access to both the utf-8
/// parsed output of `stdout` or `stderr`.
#[derive(Debug, Clone)]
pub struct Output {
    inner: process::Output,
    pub(crate) success: bool,
}

impl Output {
    /// Returns the parsed output of the `stdout` capture of the child process
    pub fn stdout(&self) -> Option<String> {
        if self.inner.stdout.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(&self.inner.stdout).to_string())
        }
    }

    /// Returns the parsed output of the `stderr` capture of the child process
    pub fn stderr(&self) -> Option<String> {
        if self.inner.stderr.is_empty() {
            None
        } else {
            Some(String::from_utf8_lossy(&self.inner.stderr).to_string())
        }
    }

    /// Returns the raw `process::Output` type
    pub fn into_inner(self) -> process::Output {
        self.inner
    }

    /// Whether the script ran successfully or not
    pub fn success(&self) -> bool {
        self.success
    }
}

impl From<process::Output> for Output {
    fn from(proc_output: process::Output) -> Output {
        let success = proc_output.status.success();
        Output {
            inner: proc_output,
            success,
        }
    }
}

impl fmt::Display for Output {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(stdout) = self.stdout() {
            write!(f, "{}", stdout)?;
        }

        if let Some(stderr) = self.stderr() {
            write!(f, "{}", stderr)?;
        }
        Ok(())
    }
}
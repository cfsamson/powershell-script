use std::collections::VecDeque;

use crate::PsScript;

pub struct PsScriptBuilder {
    args: VecDeque<&'static str>,
    no_profile: bool,
    non_interactive: bool,
    hidden: bool,
    print_commands: bool,
}

impl PsScriptBuilder {

    /// Creates a default builder with no_profile, non_interactive and hidden
    /// options set to true and print_commands set to false.
    pub fn new() -> Self {
        Self::default()
    }

    /// Prevents environment specifc scripts from being loaded. See [NoProfile parameter](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_profiles?view=powershell-7.2#the-noprofile-parameter)
    pub fn no_profile(&mut self, flag: bool) {
        self.no_profile = flag;
    }

    /// Runs the script in non-interactive mode, which does not present an
    /// interactive prompt to the user. See [NonInteractive flag](https://docs.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1#-noninteractive)
    pub fn non_interactive(&mut self, flag: bool) {
        self.non_interactive = flag;
    }

    /// Prevents PowerShell window from being shown by creating a console
    /// window with the CREATE_NO_WINDOW flag set. See [creation flags](https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags)
    ///
    /// ## Note
    /// On any other platform than Windows this is currently a no-op.
    pub fn hidden(&mut self, flag: bool) {
        self.hidden = flag;
    }

    /// If set to `true` it will print each command to `stdout` as they're run.
    /// This can be particularely useful when debugging.
    pub fn print_commands(&mut self, flag: bool) {
        self.print_commands = flag;
    }

    pub fn build(self) -> PsScript {
        let mut args = self.args;
        if self.non_interactive {
            args.push_front("-NonInteractive");
        }

        if self.no_profile {
            args.push_front("-NoProfile");
        }

        PsScript {
            args: args.make_contiguous().to_vec(),
            hidden: self.hidden,
            print_commands: self.print_commands,
        }
    }
}

impl Default for PsScriptBuilder {

    /// Creates a default builder with `no_profile`, `non_interactive` and `hidden`
    /// options set to `true` and `print_commands` set to `false`.
    fn default() -> Self {
        let mut args = VecDeque::new();
        args.push_back("-Command");
        args.push_back("-");

        Self {
            args,
            no_profile: true,
            non_interactive: true,
            hidden: true,
            print_commands: false,
        }
    }
}

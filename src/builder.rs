use std::collections::VecDeque;

pub struct PsScriptBuilder {
    args: VecDeque<&'static str>,
    no_profile: bool,
    non_interactive: bool,
    hidden: bool,
    print_commands: bool,
}

impl PsScriptBuilder {
    pub fn new() -> Self {
        let mut args = VecDeque::new();
        args.push_back("-Command");
        args.push_back("-");

        Self { args, ..Self::default() }
    }

    pub fn no_profile(&mut self, flag: bool) {
        self.no_profile = flag;
    }

    pub fn non_interactive(&mut self, flag: bool) {
       self.non_interactive = flag;
    }

    pub fn hidden(&mut self, flag: bool) {
        self.hidden = flag;
    }

    pub fn pring_commands(&mut self, flag: bool) {
        self.print_commands = flag;
    }

    pub fn build(self) -> PsScript {

    }

}

impl Default for PsScriptBuilder {
    fn default() -> Self {
        let mut args = VecDeque::new();
        args.push_back("-Command");
        args.push_back("-");

        Self { args, no_profile: true, non_interactive: true, hidden: true }
    }
}

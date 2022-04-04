# Windows Powershell script runner

This crate is pretty basic. It uses `std::process::Command` to pipe commands
to PowerShell. In addition to that there is a convenient wrapper around `process::Output`
especially tailored towards the usecase of running Windows PowerShell commands.

## Usage

I recommend that you write the commands to a `*.ps` file to be able to take advantage
of existing tools to create the script.

This example creates a shortcut of `notepad.exe` to the desktop.

_NB. If you use OneDrive chances are that your desktop is located at "$env:UserProfile\OneDrive\Desktop\" instead._

**In `script.ps`**

```ps
$SourceFileLocation="C:\Windows\notepad.exe"
$ShortcutLocation="$env:UserProfile\Desktop\notepad.lnk"
$WScriptShell=New-Object -ComObject WScript.Shell
$Shortcut=$WScriptShell.CreateShortcut($ShortcutLocation)
$Shortcut.TargetPath=$SourceFileLocation
$Shortcut.Save()
```

**In `main.rs`**

```rust
use powershell_script;

fn main() {
    let create_shortcut = include_str!("script.ps");
    match powershell_script::run(create_shortcut) {
        Ok(output) => {
            println!("{}", output);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
```

You can of course provide the commands as a string literal instead. Just beware that
we run each `line` as a separate command.

The flag `print_commands` can be set to `true` if you want each
command to be printed to the `stdout` of the main process as they're run which
can be useful for debugging scripts or displaying the progress.

## Use the `PsScriptBuilder` for better control

Instead of running a script using `powershell_script::run()` you can use
`PsScriptBuilder` to configure several options:

```rust
use powershell_script::PsScriptBuilder;

fn main() {
    let ps = PsScriptBuilder::new()
        .no_profile(true)
        .non_interactive(true)
        .hidden(false)
        .print_commands(false)
        .build();
    let output = ps.run(r#"echo "hello world""#).unwrap();

    assert!(output.stdout().unwrap().contains("hello world"));
}
```

## Features and compatability

On Windows it defaults to using the PowerShell which ships with Windows, but you
can also run scripts using PowerShell Core on Windows by enabling the
`core` feature.

On all other operating systems it will run scripts using PowerShell core.

## Contributing

Right now this is only meant as a convenient wrapper for running PowerShell scripts,
and I've been thinking about creating a `utils` crate with common tasks on Windows
like creating a shortcut to a file (symlinking requires administrative privileges)
but that will be better off in a separate create so this can focus on running scripts.

Any pull requests with bugfixes or efficiency improvements is greatly appreciated.
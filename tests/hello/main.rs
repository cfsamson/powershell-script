extern crate powershell_script;

#[cfg(windows)]
const LINE_ENDING: &'static str = "\r\n";
#[cfg(not(windows))]
const LINE_ENDING: &'static str = "\n";

#[test]
fn main() {
    let script = r#"echo "hello world""#;
    let output = powershell_script::run(script).unwrap();
    assert_eq!(output.stdout().unwrap(), format!("hello world{}", LINE_ENDING));
}

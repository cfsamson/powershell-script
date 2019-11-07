extern crate powershell_script;

#[test]
fn main() {
    let script = r#"echo "hello world""#;
    let output = powershell_script::run(script, false).unwrap();
    assert_eq!(output.stdout().unwrap(), "hello world\r\n");
}

use std::io::{Read, Write};
use std::process::{Command, Stdio};
#[test]
fn counts_single_word_through_cli() {
    let mut child: std::process::Child = Command::new("cargo")
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"rust\n").unwrap();
    std::mem::drop(stdin);
    let mut stdout = child.stdout.take().unwrap();
    let mut output = String::new();
    stdout.read_to_string(&mut output).unwrap();
    let result = child.wait().unwrap();

    assert!(output.contains("rust: 1"));
    assert!(result.success())
}
#[test]
fn counts_repeated_words_through_cli() {
    let mut child: std::process::Child = Command::new("cargo")
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    let mut stdin = child.stdin.take().unwrap();
    stdin.write_all(b"rust rust rust\n").unwrap();
    std::mem::drop(stdin);
    let mut stdout = child.stdout.take().unwrap();
    let mut output = String::new();
    stdout.read_to_string(&mut output).unwrap();
    let result = child.wait().unwrap();

    assert!(output.contains("rust: 3"));
    assert!(result.success())
}

use std::process::{Command, Stdio};

pub fn run_git_command(command: &str) {
    let output = Command::new("git")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run git command");

    let stdout = String::from_utf8(output.stdout).expect("Error converting stdout to String");
    println!("{}", stdout);
    println!("Git command completed!");
}
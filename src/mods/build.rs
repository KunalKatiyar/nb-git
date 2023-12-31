use std::process::{Command, Stdio};

pub fn run_build_command() {
    let build_output = Command::new("make")
        .arg("build")
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to run build command");

    let stdout_build = String::from_utf8(build_output.stdout).expect("Error converting stdout to String");
    if stdout_build.contains("BUILD FAILURE") {
        println!("Build failed! Here are the logs...\n");
        println!("{}", stdout_build);
    } else {
        println!("Build success! Pushing to git...\n");
    }
}
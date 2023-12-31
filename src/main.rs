mod mods;

use std::{env, process};
use mods::{git, build};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please enter an operation! \nEx: nb-git push");
        process::exit(1);
    }

    let operation = &args[1];

    match operation.as_str() {
        "push" => {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let path = current_dir.to_str().expect("Failed to convert path to string");

            if path.contains("nobroker-code") {
                build::run_build_command();
                git::run_git_command(operation);
            }
        }
        _ => {
            git::run_git_command(operation);
        }
    }
}
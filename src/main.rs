use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "install" => install(),
        "check" => check(&args[2]),
        _ => println!("Unknown command. use 'porter install' or 'porter check'")
    }
}

fn install() {
    if !Path::new(".git").is_dir() {
        println!("Use Porter in a Git-initialized project!");
        return
    }

    match Command::new("git").args(["config", "core.hooksPath", ".githooks"]).status() {
        Ok(_) => println!("Installed"),
        Err(_) => println!("Unable to configure Porter")
    }
}

fn check(commit_message: &str) {
    println!("{commit_message}");
}

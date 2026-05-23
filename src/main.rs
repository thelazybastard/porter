use std::os::unix::fs::PermissionsExt;
use std::{env, fs};
use std::process::Command;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

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
    // only takes a second might as well just run it in case install gets called again
    match Command::new("git").args(["config", "core.hooksPath", ".githooks"]).status() {
        Ok(_) => (),
        Err(_) => println!("Unable to configure Porter")
    }

    // need to get permission of the .githook and modify it to allow githook exectution 
    // for all users.  
    let hook_path = ".githooks/commit-msg";
    let metadata = match fs::metadata(hook_path) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut perms = metadata.permissions();
    perms.set_mode(perms.mode() | 0o111);
    let _ = fs::set_permissions(hook_path, perms);

    // perform a check if ollama qwen exists before installing it
    let check_output = Command::new("ollama")
        .arg("list")
        .output()
        .expect("Failed to run ollama list");
    if !String::from_utf8_lossy(&check_output.stdout).contains("qwen3.5:0.8b") {
        match Command::new("ollama").arg("pull").status() {
            Ok(_) => println!("Downloading 'qwen3.5:0.8b' (this may take a while)..."),
            Err(_) => println!("Failed to download Qwen 3.5. Try again!")
        }
    } else {
        println!("Porter and Qwen 3.5 are already installed!")
    } 
}

fn check(commit_message: &str) {
    println!("{commit_message}");

    // match Command::new("ollama").args(["run", "qwen3.5:0.8b"]).status() {
    //     Ok(_) => println!("Installed Qwen 3.5"),
    //     Err(_) => println!("Could not install Qwen 3.5. Try again!")
    // }
}

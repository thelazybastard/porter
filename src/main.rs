use std::os::unix::fs::PermissionsExt;
use std::{env, fs};
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

    // need to get permission of the .githook and modify it to allow githook exectution 
    // permission for all users.  
    let hook_path = ".githooks/commit-msg";
    let metadata = match fs::metadata(hook_path) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut perms = metadata.permissions();
    perms.set_mode(perms.mode() | 0o111);
    let _ = fs::set_permissions(hook_path, perms);
    
}

fn check(commit_message: &str) {
    println!("{commit_message}");
}

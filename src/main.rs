use std::os::unix::fs::PermissionsExt;
use std::{env, fs};
use std::process::Command;
use std::path::Path;
use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "install" => install(),
        "check" => check(&args[2]).await,
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
    if !String::from_utf8_lossy(&check_output.stdout).contains("tinyllama") {
        match Command::new("ollama").args(["pull", "tinyllama"]).status() {
            Ok(_) => println!("Downloaded tinyllama"),
            Err(_) => println!("Failed to download tinyllama. Try again!")
        }
    } else {
        println!("Porter and tinyllama are already installed!")
    } 
}

async fn check(commit_message: &str) {
    let ollama = Ollama::default();

    let model = "tinyllama:latest".to_string();
    let mut prompt = "Check out this commit message and give criticism in 1 - 2 sentences: ".to_string();
    prompt.push_str(&commit_message);

    let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

    println!("{}", res.unwrap().response);
}

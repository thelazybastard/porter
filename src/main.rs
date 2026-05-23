// 2 args. args 0 is binary so exclude that. args1 wil be either install or check. args2 is either empty or
// the commit message
// 
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "install" => install(),
        "check" => check(),
        _ => println!("Unknown command. use 'porter' install or 'porter check'")
    }
}

fn install() {
    println!("Install");
}

fn check() {
    println!("Check");
}

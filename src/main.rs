use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process;
use RustyGrep::{InputArgs, run};


fn main() {
    let args:Vec<String> = args().collect();

    let config:Result<InputArgs, &str> = InputArgs::new(&args);
    let mut conf: InputArgs;
    match config {
        Ok(t) => {
            conf = t;
        }
        Err(err) => {
            println!("{err}");
            process::exit(1);
        }
    }

    if let Err(e) = run(&conf) {
        println!("Application Error: {}", e);
        process::exit(1);
    }

    process::exit(0);
}


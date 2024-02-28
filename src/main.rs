use std::env::args;
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
            eprintln!("{err}");
            process::exit(1);
        }
    }

    if let Err(e) = run(&conf) {
        eprintln!("Application Error: {}", e);
        process::exit(1);
    }

    process::exit(0);
}


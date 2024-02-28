use std::env::args;
use std::fs::File;
use std::io::Read;
use std::process;

struct InputArgs<'a> {
    query: &'a String,
    filename: &'a String
}

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
    let mut f = File::open(conf.filename).expect("File Not Found !");
    let mut contents: String = String::new();

    f.read_to_string(&mut contents).expect(
        "Something Went Wrong During the reading of the File Contents");


}

impl<'a> InputArgs<'a> {
    fn new(args: &Vec<String>) -> Result<InputArgs, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments !");
        }
        let query = &args[0];
        let filename = &args[1];

        Ok(InputArgs{
            query,
            filename,
        })
    }
}

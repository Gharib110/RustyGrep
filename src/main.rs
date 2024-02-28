use std::env::args;
use std::fs::File;
use std::io::Read;
use std::ops::Bound::Included;

struct InputArgs<'a> {
    query: &'a String,
    filename: &'a String
}

fn main() {
    let args:Vec<String> = args().collect();

    let config: InputArgs = InputArgs::new(&args);
    let mut f = File::open(config.filename).expect("File Not Found !");
    let mut contents: String = String::new();

    f.read_to_string(&mut contents).expect(
        "Something Went Wrong During the reading of the File Contents");


}

impl InputArgs {
    fn new(args: &Vec<String>) -> InputArgs {
        let query = &args[0];
        let filename = &args[1];

        return InputArgs{
            query,
            filename,
        };
    }
}

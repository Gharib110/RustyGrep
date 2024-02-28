use std::env::args;
use std::fs::File;
use std::io::Read;

fn main() {
    let args:Vec<String> = args().collect();

    let query = &args[0];
    let filename = &args[1];

    let mut f = File::open(filename).expect("File Not Found !");
    let mut contents: String = String::new();

    f.read_to_string(&mut contents).expect(
        "Something Went Wrong During the reading of the File Contents");


}

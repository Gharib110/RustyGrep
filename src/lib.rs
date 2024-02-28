use std::env;
use std::fs::File;
use std::io::Read;

pub struct InputArgs<'a> {
    query: &'a String,
    filename: &'a String,
    case_sensitive: bool,
}


impl<'a> InputArgs<'a> {
    pub fn new(args: &Vec<String>) -> Result<InputArgs, &str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments !");
        }
        let query = &args[0];
        let filename = &args[1];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(InputArgs{
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run<'b>(config: &InputArgs) -> Result<(), &'b str> {
    let mut f = File::open(config.filename).unwrap();
    let mut contents: String = String::new();

    f.read_to_string(&mut contents).expect("Error With Reading Content");

    if config.case_sensitive == false {
        for line in search(config.query, &contents) {
            println!("{line}");
        }
    } else {
        for line in search_sensitive(config.query, &contents) {
            println!("{line}");
        }
    }


    Ok(())
}
pub fn search_sensitive<'c>(query: &str, contents: &'c str) -> Vec<&'c str> {
    let mut results: Vec<&str> = Vec::new();

    for content in contents.lines() {
        if content.contains(query) {
            results.push(content);
        }
    }
    return results;
}

pub fn search<'c>(query: &str, contents: &'c str) -> Vec<&'c str> {
    let mut results: Vec<&str> = Vec::new();

    for content in contents.lines() {
        if content.to_lowercase().contains(query) {
            results.push(content);
        }
    }
    return results;
}
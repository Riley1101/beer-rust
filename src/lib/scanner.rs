use std::env;
use std::fs;
use std::io::Error;

fn run_file(file_path: &String) {
    let contents = read_file(&file_path);
    println!("{:?}", contents);
}

fn read_file(file_path: &String) -> Result<String, Error> {
    println!("Reading file {file_path}");
    let contents = fs::read_to_string(file_path);
    contents
}

const FILE_INDEX: usize = 2;

pub fn read_args() {
    let args: Vec<String> = env::args().collect();
    const FILE: usize = 1;
    if args.len() < 2 {
        println!("Usage rBeer [script]");
    } else if args.len() == FILE_INDEX {
        run_file(&args[FILE])
    } else {
        todo!("Run the prompt")
    }
}


use std::env;
use std::fs;

fn run_file(file: &String) {
    println!("Reading file {file}");
    let contents = fs::read_to_string(file).expect("File not found at {file}");
    println!("{contents}");
}

const FILE_INDEX: usize = 2;

pub fn read_args() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage rBeer [script]");
    } else if args.len() == FILE_INDEX {
        run_file(&args[1])
    } else {
        todo!("Run the prompt")
    }
}

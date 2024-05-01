use std::env;
use std::fs;
use std::io::Error;

fn run_file(file_path: &String) -> Result<String, Error> {
    let contents = read_file(&file_path);
    contents
}

fn read_file(file_path: &String) -> Result<String, Error> {
    println!("Reading file {file_path}");
    let contents = fs::read_to_string(file_path);
    contents
}

fn run_repl() {
    println!("Usage rBeer [script]");
}

const FILE_INDEX: usize = 2;

type Callback = fn(input: String);

pub fn read_args(cb: Callback) {
    let args: Vec<String> = env::args().collect();
    const FILE: usize = 1;
    if args.len() < 2 {
        run_repl();
    } else if args.len() == FILE_INDEX {
        match run_file(&args[FILE]) {
            Ok(input) => cb(input),
            Err(_) => panic!("Execution error "),
        }
    } else {
        todo!("In progress");
    }
}

fn do_magic(input: String) {
    println!("Running code \n {input}");
}

pub fn run() {
    read_args(do_magic);
}

extern crate linecount;

use std::env;
use std::process;
use std::fs::File;

use linecount::count_lines;

const USAGE: &str = "Usage: lc <file> [<file>...]";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        for fname in &args[1..] {
            let count = match File::open(fname) {
                Ok(f) => count_lines(f),
                Err(e) => {
                    println!("Error reading from '{}': {}", fname, e);
                    continue;
                }
            };
            if args.len() == 2 {
                println!("{}", count.unwrap());
            } else {
                println!("{}:{}", fname, count.unwrap());
            }
            
        };

    } else {
        println!("{}", USAGE);
        process::exit(1);
    }
}
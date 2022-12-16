use core::panic;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

// Returns an Iterator to the Reader of the lines of the file. The output is
// wrapped in a Result to allow error matching.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn store_input(filename: &String) {
    let lines: Lines<BufReader<File>> =
        read_lines(filename).expect("Error in reading file!");
    let mut linenum = 0;
    for l in lines {
        let line = match l {
            Ok(x) => x,
            Err(error) => {
                panic!("Error in reading line {}: {}!", linenum + 1, error);
            }
        };
        linenum += 1;
        let trim_line = line.trim();
        println!("{}", trim_line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Invalid parameters. Usage: ./interpreter <filename>"),
    };

    store_input(filename);
}

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
    let mut tokens: Vec<Vec<String>> = Vec::new();
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
        let s1 = line.replace(";", "");
        let s2 = s1.trim();
        if s2.len() > 0 {
            // One word detected.
            let s2_iter = s2.split(",");
            for s in s2_iter {
                let word = s.trim().to_string();
                if word.len() == 0 {
                    panic!(
                        "Incorrect number of parameters in line {}",
                        linenum
                    );
                }
                tokens.push(Vec::new());
                tokens[linenum - 1].push(word);
            }
            if tokens[linenum - 1].len() != 3 {
                panic!("Incorrect number of parameters in line {}", linenum);
            }
        } else {
            panic!("Incorrect number of parameters in line {}", linenum);
        }
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

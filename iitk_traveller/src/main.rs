use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

fn create_memorystrip() -> Vec<i32> {
    let mut mem: Vec<i32> = Vec::with_capacity(2048);
    mem.push(0); // mem1
    mem.push(0); // mem2
    mem.push(0); // mem3
    return mem;
}

// Returns an Iterator to the Reader of the lines of the file. The output is
// wrapped in a Result to allow error matching.
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn store_input(filename: &String) -> Vec<Vec<String>> {
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
    return tokens;
}

fn create_map() -> HashMap<&str, i32> {
    let locations = HashMap::from([
        ("start", 0),
        ("finish", 1),
        ("iit_gate_in_1", 2),
        ("iit_gate_in_2", 3),
        ("hall_2", 4),
        ("hall_3", 5),
        ("hall_5", 6),
        ("hall_12", 7),
        ("mt_1_3", 8),
        ("mt_3_1", 9),
        ("mt_2_3", 10),
        ("mt_3_2", 11),
        ("iit_gate_out_1", 12),
        ("iit_gate_out_2", 13),
        ("lecture_hall_gt", 14),
        ("lecture_hall_gt_t", 15),
        ("lecture_hall_gt_f", 16),
        ("lecture_hall_lt", 17),
        ("lecture_hall_lt_t", 18),
        ("lecture_hall_lt_f", 19),
        ("lecture_hall_eq", 20),
        ("lecture_hall_eq_t", 21),
        ("lecture_hall_eq_f", 22),
        ("oat_stairs_1", 23),
        ("oat_stairs_2", 24),
        ("oat_stairs_c", 25),
        ("southern_labs_1", 26),
        ("southern_labs_2", 27),
        ("southern_labs_c", 28),
        ("hall_13_1", 29),
        ("hall_13_2", 30),
        ("hall_13_3", 31),
        ("hall_13_c", 32),
        ("rm_1", 33),
        ("rm_2", 34),
        ("rm_3", 35),
        ("kd_1", 36),
        ("kd_2", 37),
        ("kd_3", 38),
    ]);
    return locations;
}

fn operations(
    operation: &str,
    mem: &Vec<i32>,
    cond: &i32,
    curr_lm: &i32,
    loc: HashMap<&str, i32>,
) {
    let mut mem1 = 0;
    let mut mem2 = 1;
    let mut mem3 = 2;
    let mut inp: String;

    match operation {
        "iit_gate_in_1" => {
            io::stdin()
                .read_line(&mut inp)
                .expect("Failed to read line");
            mem[mem1] = match inp.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Invalid Input!"),
            };
        }
        "iit_gate_in_2" => {
            io::stdin()
                .read_line(&mut inp)
                .expect("Failed to read line");
            mem[mem2] = match inp.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("Invalid Input!"),
            };
        }
        "hall_2" => mem[mem3] = mem[mem1] + mem[mem2],
        "hall_3" => mem[mem3] = mem[mem1] * mem[mem2],
        "hall_5" => mem[mem3] = mem[mem1] - mem[mem2],
        "hall_12" => mem[mem3] = mem[mem1] / mem[mem2],
        "mt_1_3" => mem[mem1] = mem[mem3],
        "mt_3_1" => mem[mem3] = mem[mem1],
        "mt_2_3" => mem[mem2] = mem[mem3],
        "mt_3_2" => mem[mem3] = mem[mem2],
        "iit_gate_out_1" => println!("{}", mem[mem1]),
        "iit_gate_out_2" => println!("{}", mem[mem2]),
        "lecture_hall_gt" => {
            if mem[mem1] > mem[mem2] {
                *curr_lm = loc["lecture_hall_gt_t"];
            } else {
                *curr_lm = loc["lecture_hall_gt_f"];
            }
        }
        "lecture_hall_lt" => {
            if mem[mem1] < mem[mem2] {
                *curr_lm = loc["lecture_hall_lt_t"];
            } else {
                *curr_lm = loc["lecture_hall_lt_f"];
            }
        }
        "lecture_hall_eq" => {
            if mem[mem1] >= mem[mem2] {
                *curr_lm = loc["lecture_hall_eq_t"];
            } else {
                *curr_lm = loc["lecture_hall_eq_f"];
            }
        }
        "oat_stairs_1" => mem[mem1] += 1,
        "oat_stairs_2" => mem[mem2] += 1,
        "oat_stairs_c" => *cond += 1,
        "southern_labs_1" => mem[mem1] -= 1,
        "southern_labs_2" => mem[mem2] -= 1,
        "southern_labs_c" => *cond -= 1,
        "hall_13_1" => mem[mem1] = 0,
        "hall_13_2" => mem[mem2] = 0,
        "hall_13_3" => mem[mem3] = 0,
        "hall_13_c" => *cond = 0,
        "rm_1" => mem1 += 1,
        "rm_2" => mem2 += 1,
        "rm_3" => mem3 += 1,
        "kd_1" => mem1 -= 1,
        "kd_2" => mem2 -= 1,
        "kd_3" => mem3 -= 1,
        _ => panic!("No such operation exists!"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Invalid parameters. Usage: ./interpreter <filename>"),
    };

    let tokens = store_input(filename);
    let locations = create_map();
}

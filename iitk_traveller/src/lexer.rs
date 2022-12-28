// use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;
use std::convert::TryInto;

// Returns an Iterator to the Reader of the lines of the file. The output is
// wrapped in a Result to allow error matching.
pub fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn store_input(filename: &String) -> Vec<Vec<String>> {
    let mut tokens: Vec<Vec<String>> = Vec::new();

    let lines: Lines<BufReader<File>> =
        read_lines(filename).expect("Error in reading file!");

    let mut linenum = 0;
    for l in lines {
        linenum += 1;

        let line = match l {
            Ok(x) => x,
            Err(error) => {
                panic!("Error in reading line {}: {}!", linenum, error);
            }
        };
        
        // let s1 = line.replace(";", "");
        let mut s2 = line.trim().to_string();
        if s2.len() > 0 {
            // One word detected.
            let last_char = s2.chars().nth(line.chars().count() - 1).unwrap();
            if last_char != ';' {
                panic!("Line number {}: Missing ;", linenum);
            }

            s2 = s2.replace(";", "");
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
        } 
        else {
            panic!("Incorrect number of parameters in line {}", linenum);
        }
    }
    return tokens;
}

pub fn create_map() -> HashMap<String, i32> {
    let locations = HashMap::from([
        ("start".to_string(), 0),
        ("finish".to_string(), 1),
        ("iit_gate_in_1".to_string(), 2),
        ("iit_gate_in_2".to_string(), 3),
        ("hall_2".to_string(), 4),
        ("hall_3".to_string(), 5),
        ("hall_5".to_string(), 6),
        ("hall_12".to_string(), 7),
        ("mt_1_3".to_string(), 8),
        ("mt_3_1".to_string(), 9),
        ("mt_2_3".to_string(), 10),
        ("mt_3_2".to_string(), 11),
        ("iit_gate_out_1".to_string(), 12),
        ("iit_gate_out_2".to_string(), 13),
        ("lecture_hall_gt".to_string(), 14),
        ("lecture_hall_gt_t".to_string(), 15),
        ("lecture_hall_gt_f".to_string(), 16),
        ("lecture_hall_lt".to_string(), 17),
        ("lecture_hall_lt_t".to_string(), 18),
        ("lecture_hall_lt_f".to_string(), 19),
        ("lecture_hall_eq".to_string(), 20),
        ("lecture_hall_eq_t".to_string(), 21),
        ("lecture_hall_eq_f".to_string(), 22),
        ("oat_stairs_1".to_string(), 23),
        ("oat_stairs_2".to_string(), 24),
        ("oat_stairs_c".to_string(), 25),
        ("southern_labs_1".to_string(), 26),
        ("southern_labs_2".to_string(), 27),
        ("southern_labs_c".to_string(), 28),
        ("hall_13_1".to_string(), 29),
        ("hall_13_2".to_string(), 30),
        ("hall_13_3".to_string(), 31),
        ("hall_13_c".to_string(), 32),
        ("rm_1".to_string(), 33),
        ("rm_2".to_string(), 34),
        ("rm_3".to_string(), 35),
        ("kd_1".to_string(), 36),
        ("kd_2".to_string(), 37),
        ("kd_3".to_string(), 38),
    ]);
    return locations;
}

pub fn build_graph(
    tokens: &Vec<Vec<String>>,
    locations: &HashMap<String, i32>,
) -> HashMap<i32, HashMap<i32, i32>> 
{
    let mut graph: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for i in 0..locations.len() {
        graph.insert(i.try_into().unwrap(), HashMap::new());
    }

    let length = tokens.len() / 3;
    for linenum in 0..length {
        let loc1 = match locations.get(&tokens[linenum][0]) {
            Some(l) => l,
            None => panic!(
                "Line {}: '{}' is not a valid landmark!",
                linenum + 1,
                tokens[linenum][0]
            ),
        };

        let cond_val: i32 = match tokens[linenum][1].parse() {
            Ok(num) => num,
            Err(_) => panic!(
                "Line {}: Given weight is not a valid integer value!",
                linenum + 1
            ),
        };

        let loc2 = match locations.get(&tokens[linenum][2]) {
            Some(l) => l,
            None => panic!(
                "Line {}: '{}' is not a valid landmark!",
                linenum + 1,
                tokens[linenum][2]
            ),
        };

        if graph[&loc1].contains_key(&cond_val) {
            panic!("Graph exists");
        } else {
            graph.get_mut(&loc1).map(|val| val.insert(cond_val, *loc2));
        }
    }
    return graph;
}

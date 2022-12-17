use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::path::Path;

struct TravelStat {
    mem1: usize,
    mem2: usize,
    mem3: usize,
    cond: i32,
    curr_loc: i32,
}

impl TravelStat {
    fn new(
        mem1: usize,
        mem2: usize,
        mem3: usize,
        cond: i32,
        curr_loc: i32,
    ) -> TravelStat {
        TravelStat {
            mem1,
            mem2,
            mem3,
            cond,
            curr_loc,
        }
    }

    fn perform_operation(
        &mut self,
        operation: i32,
        mem: &mut Vec<i32>,
        loc: &HashMap<String, i32>,
    ) {
        let mut inp = "".to_string();

        match operation {
            2 => {
                // "iit_gate_in_1"
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                mem[self.mem1] = match inp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Invalid Input!"),
                };
            }
            3 => {
                // "iit_gate_in_2"
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                mem[self.mem2] = match inp.trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!("Invalid Input!"),
                };
            }
            4 => mem[self.mem3] = mem[self.mem1] + mem[self.mem2], // "hall_2"
            5 => mem[self.mem3] = mem[self.mem1] * mem[self.mem2], // "hall_3"
            6 => mem[self.mem3] = mem[self.mem1] - mem[self.mem2], // "hall_5"
            7 => mem[self.mem3] = mem[self.mem1] / mem[self.mem2], // "hall_12"
            8 => mem[self.mem1] = mem[self.mem3],                  // "mt_1_3"
            9 => mem[self.mem3] = mem[self.mem1],                  // "mt_3_1"
            10 => mem[self.mem2] = mem[self.mem3],                 // "mt_2_3"
            11 => mem[self.mem3] = mem[self.mem2],                 // "mt_3_2"
            12 => println!("{}", mem[self.mem1]), // "iit_gate_out_1"
            13 => println!("{}", mem[self.mem2]), // "iit_gate_out_2"
            14 => {
                // "lecture_hall_gt"
                if mem[self.mem1] > mem[self.mem2] {
                    self.curr_loc = loc["lecture_hall_gt_t"];
                } else {
                    self.curr_loc = loc["lecture_hall_gt_f"];
                }
            }
            17 => {
                // "lecture_hall_lt"
                if mem[self.mem1] < mem[self.mem2] {
                    self.curr_loc = loc["lecture_hall_lt_t"];
                } else {
                    self.curr_loc = loc["lecture_hall_lt_f"];
                }
            }
            20 => {
                // "lecture_hall_eq"
                if mem[self.mem1] >= mem[self.mem2] {
                    self.curr_loc = loc["lecture_hall_eq_t"];
                } else {
                    self.curr_loc = loc["lecture_hall_eq_f"];
                }
            }
            23 => mem[self.mem1] += 1, // "oat_stairs_1"
            24 => mem[self.mem2] += 1, // "oat_stairs_2"
            25 => self.cond += 1,      // "oat_stairs_c"
            26 => mem[self.mem1] -= 1, // "southern_labs_1"
            27 => mem[self.mem2] -= 1, // "southern_labs_2"
            28 => self.cond -= 1,      // "southern_labs_c"
            29 => mem[self.mem1] = 0,  // "hall_13_1"
            30 => mem[self.mem2] = 0,  // "hall_13_2"
            31 => mem[self.mem3] = 0,  // "hall_13_3"
            32 => self.cond = 0,       // "hall_13_c"
            33 => self.mem1 += 1,      // "rm_1"
            34 => self.mem2 += 1,      // "rm_2"
            35 => self.mem3 += 1,      // "rm_3"
            36 => self.mem1 -= 1,      // "kd_1"
            37 => self.mem2 -= 1,      // "kd_2"
            38 => self.mem3 -= 1,      // "kd_3"
            _ => panic!("No such operation exists!"),
        }
    }

    fn travel(
        &mut self,
        mem: &mut Vec<i32>,
        locations: &HashMap<String, i32>,
        graph: &HashMap<i32, HashMap<i32, i32>>,
    ) {
        if self.curr_loc == locations["finish"] {
            return;
        }

        if self.curr_loc != locations["start"] {
            self.perform_operation(self.curr_loc, mem, &locations);
        }

        if !graph[&self.curr_loc].contains_key(&self.cond) {
            panic!("Stuck in landmark number {}", self.curr_loc);
        }

        self.curr_loc = graph[&self.curr_loc][&self.cond];
        return self.travel(mem, locations, graph);
    }
}

fn create_memorystrip(len: i32) -> Vec<i32> {
    let mut mem: Vec<i32> = Vec::new();
    for _i in 0..len {
        mem.push(0);
    }
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

fn create_map() -> HashMap<String, i32> {
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

fn build_graph(
    tokens: &Vec<Vec<String>>,
    locations: &HashMap<String, i32>,
) -> HashMap<i32, HashMap<i32, i32>> {
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

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = match args.len() {
        2 => &args[1],
        _ => panic!("Invalid parameters. Usage: ./interpreter <filename>"),
    };

    let mut mem = create_memorystrip(2048);
    let tokens = store_input(filename);
    let locations = create_map();
    let graph = build_graph(&tokens, &locations);
    let mut traveller = TravelStat::new(0, 1, 2, 0, 0);

    traveller.travel(&mut mem, &locations, &graph);
}

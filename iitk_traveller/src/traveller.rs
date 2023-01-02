use std::collections::HashMap;
use text_io::read;

pub struct TravelStat {
    mem1: usize,
    mem1_lvl: usize,
    mem2: usize,
    mem2_lvl: usize,
    mem3: usize,
    mem3_lvl: usize,
    cond: i32,
    curr_loc: i32,
}

impl TravelStat {
    pub fn new(
        mem1: usize,
        mem1_lvl: usize,
        mem2: usize,
        mem2_lvl: usize,
        mem3: usize,
        mem3_lvl: usize,
        cond: i32,
        curr_loc: i32,
    ) -> TravelStat {
        TravelStat {
            mem1,
            mem1_lvl,
            mem2,
            mem2_lvl,
            mem3,
            mem3_lvl,
            cond,
            curr_loc,
        }
    }

    fn perform_operation(
        &mut self,
        operation: i32,
        mem: &mut Vec<Vec<i32>>,
        loc: &HashMap<String, i32>,
    ) {

        match operation {
            2 => {
                let i: i32 = read!();
                mem[self.mem1_lvl][self.mem1] = i;
            }
            3 => {
                let i:i32 = read!();
                mem[self.mem2_lvl][self.mem2] = i;
            }
            4 => {
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    + mem[self.mem2_lvl][self.mem2]
            } // "hall_2"
            5 => {
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    * mem[self.mem2_lvl][self.mem2]
            } // "hall_3"
            6 => {
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    - mem[self.mem2_lvl][self.mem2]
            } // "hall_5"
            7 => {
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    / mem[self.mem2_lvl][self.mem2]
            } // "hall_12"
            8 => mem[self.mem1_lvl][self.mem1] = mem[self.mem3_lvl][self.mem3], // "mt_1_3"
            9 => mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1], // "mt_3_1"
            10 => mem[self.mem2_lvl][self.mem2] = mem[self.mem3_lvl][self.mem3], // "mt_2_3"
            11 => mem[self.mem3_lvl][self.mem3] = mem[self.mem2_lvl][self.mem2], // "mt_3_2"
            12 => println!("{}", mem[self.mem1_lvl][self.mem1]), // "iit_gate_out_1"
            13 => println!("{}", mem[self.mem2_lvl][self.mem2]), // "iit_gate_out_2"
            14 => {
                // "lecture_hall_gt"
                if mem[self.mem1_lvl][self.mem1] > mem[self.mem2_lvl][self.mem2]
                {
                    self.curr_loc = loc["lecture_hall_gt_t"];
                } else {
                    self.curr_loc = loc["lecture_hall_gt_f"];
                }
            }
            17 => {
                // "lecture_hall_lt"
                if mem[self.mem1_lvl][self.mem1] < mem[self.mem2_lvl][self.mem2]
                {
                    self.curr_loc = loc["lecture_hall_lt_t"];
                } else {
                    self.curr_loc = loc["lecture_hall_lt_f"];
                }
            }
            20 => {
                // "lecture_hall_eq"
                if mem[self.mem1_lvl][self.mem1]
                    >= mem[self.mem2_lvl][self.mem2]
                {
                    self.curr_loc = loc["lecture_hall_eq_t"];
                } else {
                    self.curr_loc = loc["lecture_hall_eq_f"];
                }
            }
            23 => mem[self.mem1_lvl][self.mem1] += 1, // "oat_stairs_1"
            24 => mem[self.mem2_lvl][self.mem2] += 1, // "oat_stairs_2"
            25 => self.cond += 1,                     // "oat_stairs_c"
            26 => mem[self.mem1_lvl][self.mem1] -= 1, // "southern_labs_1"
            27 => mem[self.mem2_lvl][self.mem2] -= 1, // "southern_labs_2"
            28 => self.cond -= 1,                     // "southern_labs_c"
            29 => mem[self.mem1_lvl][self.mem1] = 0,  // "hall_13_1"
            30 => mem[self.mem2_lvl][self.mem2] = 0,  // "hall_13_2"
            31 => mem[self.mem3_lvl][self.mem3] = 0,  // "hall_13_3"
            32 => self.cond = 0,                      // "hall_13_c"
            33 => {
                // "rm_1"
                self.mem1 += 1;
                if self.mem1 == 2048 {
                    self.mem1_lvl += 1;
                    if mem.len() <= self.mem1_lvl {
                        mem.push(vec![0; 2048]);
                    }
                    self.mem1 = 0;
                }
            }
            34 => {
                // "rm_2"
                self.mem2 += 1;
                if self.mem2 == 2048 {
                    self.mem2_lvl += 1;
                    if mem.len() <= self.mem1_lvl {
                        mem.push(vec![0; 2048]);
                    }
                    self.mem2 = 0;
                }
            }
            35 => {
                // "rm_3"
                self.mem3 += 1;
                if self.mem3 == 2048 {
                    self.mem3_lvl += 1;
                    if mem.len() <= self.mem1_lvl {
                        mem.push(vec![0; 2048]);
                    }
                    self.mem3 = 0;
                }
            }
            36 => {
                // "kd_1"
                if self.mem1 != 0 {
                    self.mem1 -= 1;
                } else {
                    if self.mem1_lvl != 0 {
                        self.mem1 = 2047;
                        self.mem1_lvl -= 1;
                    }
                }
            }
            37 => {
                // "kd_2"
                if self.mem2 != 0 {
                    self.mem2 -= 1;
                } else {
                    if self.mem2_lvl != 0 {
                        self.mem2 = 2047;
                        self.mem2_lvl -= 1;
                    }
                }
            }
            38 => {
                // "kd_3"
                if self.mem3 != 0 {
                    self.mem3 -= 1;
                } else {
                    if self.mem3_lvl != 0 {
                        self.mem3 = 2047;
                        self.mem3_lvl -= 1;
                    }
                }
            }
            39 => {
                mem[self.mem1_lvl][self.mem1] *= mem[self.mem1_lvl][self.mem1]
            } // "eshop_1"
            40 => {
                mem[self.mem2_lvl][self.mem2] *= mem[self.mem2_lvl][self.mem2]
            } // "eshop_2"
            41 => {
                // "doaa_1"
                let test: u32 = match mem[self.mem1_lvl][self.mem1].try_into() {
                    Ok(c) => c,
                    Err(_) => panic!("No equivalent character for the code!"),
                };
                let ch = match char::from_u32(test) {
                    Some(c) => c,
                    None => panic!("Cannot convert into character!"),
                };
                print!("{}", ch);
            }
            42 => {
                // "doaa_2"
                let test: u32 = match mem[self.mem2_lvl][self.mem2].try_into() {
                    Ok(c) => c,
                    Err(_) => panic!("No equivalent character for the code!"),
                };
                let ch = match char::from_u32(test) {
                    Some(c) => c,
                    None => panic!("Cannot convert into character!"),
                };
                print!("{}", ch);
            }
            _ => panic!("No such operation exists!"),
        }
    }

    pub fn travel(
        &mut self,
        mem: &mut Vec<Vec<i32>>,
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

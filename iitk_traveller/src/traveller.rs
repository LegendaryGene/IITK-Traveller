use std::collections::HashMap;
use std::io::{self};
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
    prev_loc: i32,
}

impl TravelStat {
    // Construct a new instance.
    pub fn new(
        mem1: usize,
        mem1_lvl: usize,
        mem2: usize,
        mem2_lvl: usize,
        mem3: usize,
        mem3_lvl: usize,
        cond: i32,
        curr_loc: i32,
        prev_loc: i32,
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
            prev_loc,
        }
    }

    fn perform_operation(
        &mut self,
        operation: i32,
        mem: &mut Vec<Vec<i32>>,
        mem_flag: &mut Vec<Vec<i8>>,
        loc: &HashMap<String, i32>,
        increment_graph: &HashMap<(i32, i32), i32>,
    ) {
        let mut inp = "".to_string();

        match operation {
            2 => {
                // "iit_gate_in_1"
                let i = read!();
                mem[self.mem1_lvl][self.mem1] = i;
                mem_flag[self.mem1_lvl][self.mem1] = 0;
            }
            3 => {
                // "iit_gate_in_2"
                let i = read!();
                mem[self.mem2_lvl][self.mem2] = i;
                mem_flag[self.mem2_lvl][self.mem2] = 0;
            }
            4 => {
                // "hall_2"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("hall_2 operation is not allowed for EOS literal");
                }
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    + mem[self.mem2_lvl][self.mem2];
                mem_flag[self.mem3_lvl][self.mem3] = 0;
            }
            5 => {
                // "hall_3"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("hall_3 operation is not allowed for EOS literal");
                }
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    * mem[self.mem2_lvl][self.mem2];
                mem_flag[self.mem3_lvl][self.mem3] = 0;
            }
            6 => {
                // "hall_5"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("hall_5 operation is not allowed for EOS literal");
                }
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    - mem[self.mem2_lvl][self.mem2];
                mem_flag[self.mem3_lvl][self.mem3] = 0;
            }
            7 => {
                // "hall_12"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("hall_12 operation is not allowed for EOS literal");
                }
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1]
                    / mem[self.mem2_lvl][self.mem2];
                mem_flag[self.mem3_lvl][self.mem3] = 0;
            }
            8 => {
                // "mt_1_3"
                mem[self.mem1_lvl][self.mem1] = mem[self.mem3_lvl][self.mem3];
                mem_flag[self.mem1_lvl][self.mem1] =
                    mem_flag[self.mem3_lvl][self.mem3]
            }
            9 => {
                // "mt_3_1"
                mem[self.mem3_lvl][self.mem3] = mem[self.mem1_lvl][self.mem1];
                mem_flag[self.mem3_lvl][self.mem3] =
                    mem_flag[self.mem1_lvl][self.mem1];
            }
            10 => {
                // "mt_2_3"
                mem[self.mem2_lvl][self.mem2] = mem[self.mem3_lvl][self.mem3];
                mem_flag[self.mem2_lvl][self.mem2] =
                    mem_flag[self.mem3_lvl][self.mem3];
            }
            11 => {
                // "mt_3_2"
                mem[self.mem3_lvl][self.mem3] = mem[self.mem2_lvl][self.mem2];
                mem_flag[self.mem3_lvl][self.mem3] =
                    mem_flag[self.mem2_lvl][self.mem2];
            }
            12 => print!("{} ", mem[self.mem1_lvl][self.mem1]), // "iit_gate_out_1"
            13 => print!("{} ", mem[self.mem2_lvl][self.mem2]), // "iit_gate_out_2"
            14 => {
                // "lecture_hall_gt"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("lecture_hall_gt operation is not allowed for EOS literal");
                }
                if mem[self.mem1_lvl][self.mem1] > mem[self.mem2_lvl][self.mem2]
                {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["lecture_hall_gt_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["lecture_hall_gt_f"];
                }
            }
            17 => {
                // "lecture_hall_lt"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("lecture_hall_lt operation is not allowed for EOS literal");
                }
                if mem[self.mem1_lvl][self.mem1] < mem[self.mem2_lvl][self.mem2]
                {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["lecture_hall_lt_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["lecture_hall_lt_f"];
                }
            }
            20 => {
                // "lecture_hall_eq"
                if mem_flag[self.mem1_lvl][self.mem1] == 1
                    || mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    panic!("lecture_hall_eq operation is not allowed for EOS literal");
                }
                if mem[self.mem1_lvl][self.mem1]
                    == mem[self.mem2_lvl][self.mem2]
                    && mem_flag[self.mem1_lvl][self.mem1]
                        == mem_flag[self.mem2_lvl][self.mem2]
                {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["lecture_hall_eq_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["lecture_hall_eq_f"];
                }
            }
            23 => {
                // "oat_stairs_1"
                if mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    panic!(
                        "oat_stairs_1 operation is not allowed for EOS literal"
                    );
                }
                mem[self.mem1_lvl][self.mem1] += 1;
            }
            24 => {
                // "oat_stairs_2"
                if mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    panic!(
                        "oat_stairs_2 operation is not allowed for EOS literal"
                    );
                }
                mem[self.mem2_lvl][self.mem2] += 1;
            }
            25 => self.cond += 1, // "oat_stairs_c"
            26 => {
                // "southern_labs_1"
                if mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    panic!("southern_labs_1 operation is not allowed for EOS literal");
                }
                mem[self.mem1_lvl][self.mem1] -= 1;
            }
            27 => {
                // "southern_labs_2"
                if mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    panic!("southern_labs_2 operation is not allowed for EOS literal");
                }
                mem[self.mem2_lvl][self.mem2] -= 1;
            }
            28 => self.cond -= 1, // "southern_labs_c"
            29 => {
                // "hall_13_1"
                mem[self.mem1_lvl][self.mem1] = 0;
                mem_flag[self.mem1_lvl][self.mem1] = 0;
            }
            30 => {
                // "hall_13_2"
                mem[self.mem2_lvl][self.mem2] = 0;
                mem_flag[self.mem2_lvl][self.mem2] = 0;
            }
            31 => {
                // "hall_13_3"
                mem[self.mem3_lvl][self.mem3] = 0;
                mem_flag[self.mem3_lvl][self.mem3] = 0;
            }
            32 => self.cond = 0, // "hall_13_c"
            33 => {
                // "rm_1"
                self.mem1 += 1;
                if self.mem1 == 2048 {
                    self.mem1_lvl += 1;
                    if mem.len() <= self.mem1_lvl {
                        mem.push(vec![0; 2048]);
                        mem_flag.push(vec![0; 2048]);
                    }
                    self.mem1 = 0;
                }
            }
            34 => {
                // "rm_2"
                self.mem2 += 1;
                if self.mem2 == 2048 {
                    self.mem2_lvl += 1;
                    if mem.len() <= self.mem3_lvl {
                        mem.push(vec![0; 2048]);
                        mem_flag.push(vec![0; 2048]);
                    }
                    self.mem2 = 0;
                }
            }
            35 => {
                // "rm_3"
                self.mem3 += 1;
                if self.mem3 == 2048 {
                    self.mem3_lvl += 1;
                    if mem.len() <= self.mem3_lvl {
                        mem.push(vec![0; 2048]);
                        mem_flag.push(vec![0; 2048]);
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
                // "eshop_1"
                if mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    panic!("Eshop1 operation not allowed for EOS literals");
                }
                mem[self.mem1_lvl][self.mem1] *= mem[self.mem1_lvl][self.mem1]
            }
            40 => {
                // "eshop_2"
                if mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    panic!("Eshop2 operation not allowed for EOS literals");
                }
                mem[self.mem2_lvl][self.mem2] *= mem[self.mem2_lvl][self.mem2]
            }
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
                println!("{}", ch);
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
                println!("{}", ch);
            }
            43 => {
                // "airstrip_land_1"
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let mut ptr_pos: usize = self.mem1;
                let mut ptr_lvl: usize = self.mem1_lvl;
                for c in inp.chars() {
                    let ascii = c as i32;
                    if ascii == 10 {
                        continue;
                    }
                    mem[ptr_lvl][ptr_pos] = ascii;
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                mem_flag[ptr_lvl][ptr_pos] = 1;
            }
            44 => {
                // "airstrip_land_2"
                io::stdin()
                    .read_line(&mut inp)
                    .expect("Failed to read line");
                let mut ptr_pos: usize = self.mem2;
                let mut ptr_lvl: usize = self.mem2_lvl;

                for c in inp.chars() {
                    let ascii = c as i32;
                    if ascii == 10 {
                        continue;
                    }
                    mem[ptr_lvl][ptr_pos] = ascii;
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                mem_flag[ptr_lvl][ptr_pos] = 1;
            }
            45 => {
                // "airstrip_takeoff_1"
                let mut ptr_pos = self.mem1;
                let mut ptr_lvl = self.mem1_lvl;
                while mem_flag[ptr_lvl][ptr_pos] != 1 {
                    let ch = match char::from_u32(mem[ptr_lvl][ptr_pos] as u32)
                    {
                        Some(c) => c,
                        None => panic!("Cannot convert into character!"),
                    };
                    print!("{}", ch);
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                println!();
            }
            46 => {
                // "airstrip_takeoff_2"
                let mut ptr_pos = self.mem2;
                let mut ptr_lvl = self.mem2_lvl;
                while mem_flag[ptr_lvl][ptr_pos] != 1 {
                    let ch = match char::from_u32(mem[ptr_lvl][ptr_pos] as u32)
                    {
                        Some(c) => c,
                        None => panic!("Cannot convert into character!"),
                    };
                    print!("{}", ch);
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                println!();
            }
            47 => {
                // "pronite_1"
                mem[self.mem1_lvl][self.mem1] = 0;
                mem_flag[self.mem1_lvl][self.mem1] = 1
            }
            48 => {
                // "pronite_2"
                mem_flag[self.mem2_lvl][self.mem2] = 1;
                mem[self.mem2_lvl][self.mem2] = 0;
            }
            49 => {
                // "events_1"
                if mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["events_1_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["events_1_f"];
                }
            }
            52 => {
                // "events_2"
                if mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["events_2_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = loc["events_2_f"];
                }
            }
            55 => {
                // "oat_stage"
                if increment_graph.contains_key(&(self.prev_loc, self.cond)) {
                    let change_by: i32 = *(increment_graph
                        .get(&(self.prev_loc, self.cond))
                        .unwrap());
                    self.cond += change_by;
                }
            }
            _ => panic!("No such operation exists!"),
        }
    }

    pub fn travel(
        &mut self,
        mem: &mut Vec<Vec<i32>>,
        mem_flag: &mut Vec<Vec<i8>>,
        locations: &HashMap<String, i32>,
        graph: &HashMap<i32, HashMap<i32, i32>>,
        increment_graph: &HashMap<(i32, i32), i32>,
    ) {
        if self.curr_loc == locations["finish"] {
            return;
        }

        if self.curr_loc != locations["start"] {
            self.perform_operation(
                self.curr_loc,
                mem,
                mem_flag,
                &locations,
                increment_graph,
            );
        }

        if !graph[&self.curr_loc].contains_key(&self.cond) {
            panic!(
                "Stuck in landmark number {} with condition {}",
                self.curr_loc, self.cond
            );
        }

        self.prev_loc = self.curr_loc;
        self.curr_loc = graph[&self.curr_loc][&self.cond];

        return self.travel(mem, mem_flag, locations, graph, increment_graph);
    }
}

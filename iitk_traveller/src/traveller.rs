use std::collections::HashMap;
// use std::hash::Hash;
use std::io::{self};
use text_io::read;

pub struct TravelStat<'a> {
    mem1: usize,
    mem1_lvl: usize,
    mem2: usize,
    mem2_lvl: usize,
    mem3: usize,
    mem3_lvl: usize,
    cond: i32,
    curr_loc: i32,
    prev_loc: i32,
    mem: &'a mut Vec<Vec<i32>>,
    mem_flag: &'a mut Vec<Vec<i8>>,
}

impl TravelStat<'_> {
    // Construct a new instance.
    pub fn new<'a>(
        mem1: usize,
        mem1_lvl: usize,
        mem2: usize,
        mem2_lvl: usize,
        mem3: usize,
        mem3_lvl: usize,
        cond: i32,
        curr_loc: i32,
        prev_loc: i32,
        mem: &'a mut Vec<Vec<i32>>,
        mem_flag: &'a mut Vec<Vec<i8>>,
    ) -> TravelStat<'a> {
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
            mem,
            mem_flag,
        }
    }

    fn perform_operation(
        &mut self,
        locations: &HashMap<String, i32>,
        increment_graph: &HashMap<(i32, i32), i32>,
        operation: i32,
    ) -> Result<(), String> {
        let mut inp = "".to_string();

        match operation {
            2 => {
                // "iit_gate_in_1"
                let i = read!();
                self.mem[self.mem1_lvl][self.mem1] = i;
                self.mem_flag[self.mem1_lvl][self.mem1] = 0;
                return Ok(());
            }
            3 => {
                // "iit_gate_in_2"
                let i = read!();
                self.mem[self.mem2_lvl][self.mem2] = i;
                self.mem_flag[self.mem2_lvl][self.mem2] = 0;
                return Ok(());
            }
            4 => {
                // "hall_2"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!(
                        "hall_2 operation is not allowed for EOS literal"
                    ));
                }
                self.mem[self.mem3_lvl][self.mem3] = self.mem[self.mem1_lvl]
                    [self.mem1]
                    + self.mem[self.mem2_lvl][self.mem2];
                self.mem_flag[self.mem3_lvl][self.mem3] = 0;
                return Ok(());
            }
            5 => {
                // "hall_3"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!(
                        "hall_3 operation is not allowed for EOS literal"
                    ));
                }
                self.mem[self.mem3_lvl][self.mem3] = self.mem[self.mem1_lvl]
                    [self.mem1]
                    * self.mem[self.mem2_lvl][self.mem2];
                self.mem_flag[self.mem3_lvl][self.mem3] = 0;
                return Ok(());
            }
            6 => {
                // "hall_5"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!(
                        "hall_5 operation is not allowed for EOS literal"
                    ));
                }
                self.mem[self.mem3_lvl][self.mem3] = self.mem[self.mem1_lvl]
                    [self.mem1]
                    - self.mem[self.mem2_lvl][self.mem2];
                self.mem_flag[self.mem3_lvl][self.mem3] = 0;
                return Ok(());
            }
            7 => {
                // "hall_12"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!(
                        "hall_12 operation is not allowed for EOS literal"
                    ));
                }
                self.mem[self.mem3_lvl][self.mem3] = self.mem[self.mem1_lvl]
                    [self.mem1]
                    / self.mem[self.mem2_lvl][self.mem2];
                self.mem_flag[self.mem3_lvl][self.mem3] = 0;
                return Ok(());
            }
            8 => {
                // "mt_1_3"
                self.mem[self.mem1_lvl][self.mem1] =
                    self.mem[self.mem3_lvl][self.mem3];
                self.mem_flag[self.mem1_lvl][self.mem1] =
                    self.mem_flag[self.mem3_lvl][self.mem3];
                return Ok(());
            }
            9 => {
                // "mt_3_1"
                self.mem[self.mem3_lvl][self.mem3] =
                    self.mem[self.mem1_lvl][self.mem1];
                self.mem_flag[self.mem3_lvl][self.mem3] =
                    self.mem_flag[self.mem1_lvl][self.mem1];
                return Ok(());
            }
            10 => {
                // "mt_2_3"
                self.mem[self.mem2_lvl][self.mem2] =
                    self.mem[self.mem3_lvl][self.mem3];
                self.mem_flag[self.mem2_lvl][self.mem2] =
                    self.mem_flag[self.mem3_lvl][self.mem3];
                return Ok(());
            }
            11 => {
                // "mt_3_2"
                self.mem[self.mem3_lvl][self.mem3] =
                    self.mem[self.mem2_lvl][self.mem2];
                self.mem_flag[self.mem3_lvl][self.mem3] =
                    self.mem_flag[self.mem2_lvl][self.mem2];
                return Ok(());
            }
            12 => {
                print!("{} ", self.mem[self.mem1_lvl][self.mem1]);
                return Ok(());
            } // "iit_gate_out_1"
            13 => {
                print!("{} ", self.mem[self.mem2_lvl][self.mem2]);
                return Ok(());
            } // "iit_gate_out_2"
            14 => {
                // "lecture_hall_gt"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!("lecture_hall_gt operation is not allowed for EOS literal"));
                }
                if self.mem[self.mem1_lvl][self.mem1]
                    > self.mem[self.mem2_lvl][self.mem2]
                {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["lecture_hall_gt_t"];
                    return Ok(());
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["lecture_hall_gt_f"];
                    return Ok(());
                }
            }
            17 => {
                // "lecture_hall_lt"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!("lecture_hall_lt operation is not allowed for EOS literal"));
                }
                if self.mem[self.mem1_lvl][self.mem1]
                    < self.mem[self.mem2_lvl][self.mem2]
                {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["lecture_hall_lt_t"];
                    return Ok(());
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["lecture_hall_lt_f"];
                    return Ok(());
                }
            }
            20 => {
                // "lecture_hall_eq"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1
                    || self.mem_flag[self.mem2_lvl][self.mem2] == 1
                {
                    return Err(format!("lecture_hall_eq operation is not allowed for EOS literal"));
                }
                if self.mem[self.mem1_lvl][self.mem1]
                    == self.mem[self.mem2_lvl][self.mem2]
                    && self.mem_flag[self.mem1_lvl][self.mem1]
                        == self.mem_flag[self.mem2_lvl][self.mem2]
                {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["lecture_hall_eq_t"];
                    return Ok(());
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["lecture_hall_eq_f"];
                    return Ok(());
                }
            }
            23 => {
                // "oat_stairs_1"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    return Err(format!(
                        "oat_stairs_1 operation is not allowed for EOS literal"
                    ));
                }
                self.mem[self.mem1_lvl][self.mem1] += 1;
                return Ok(());
            }
            24 => {
                // "oat_stairs_2"
                if self.mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    return Err(format!(
                        "oat_stairs_2 operation is not allowed for EOS literal"
                    ));
                }
                self.mem[self.mem2_lvl][self.mem2] += 1;
                return Ok(());
            }
            25 => {
                self.cond += 1;
                return Ok(());
            } // "oat_stairs_c"
            26 => {
                // "southern_labs_1"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    return Err(format!("southern_labs_1 operation is not allowed for EOS literal"));
                }
                self.mem[self.mem1_lvl][self.mem1] -= 1;
                return Ok(());
            }
            27 => {
                // "southern_labs_2"
                if self.mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    return Err(format!("southern_labs_2 operation is not allowed for EOS literal"));
                }
                self.mem[self.mem2_lvl][self.mem2] -= 1;
                return Ok(());
            }
            28 => {
                self.cond -= 1;
                return Ok(());
            } // "southern_labs_c"
            29 => {
                // "hall_13_1"
                self.mem[self.mem1_lvl][self.mem1] = 0;
                self.mem_flag[self.mem1_lvl][self.mem1] = 0;
                return Ok(());
            }
            30 => {
                // "hall_13_2"
                self.mem[self.mem2_lvl][self.mem2] = 0;
                self.mem_flag[self.mem2_lvl][self.mem2] = 0;
                return Ok(());
            }
            31 => {
                // "hall_13_3"
                self.mem[self.mem3_lvl][self.mem3] = 0;
                self.mem_flag[self.mem3_lvl][self.mem3] = 0;
                return Ok(());
            }
            32 => {
                // "hall_13_c"
                self.cond = 0;
                return Ok(());
            }
            33 => {
                // "rm_1"
                self.mem1 += 1;
                if self.mem1 == 2048 {
                    self.mem1_lvl += 1;
                    if self.mem.len() <= self.mem1_lvl {
                        self.mem.push(vec![0; 2048]);
                        self.mem_flag.push(vec![0; 2048]);
                    }
                    self.mem1 = 0;
                }
                return Ok(());
            }
            34 => {
                // "rm_2"
                self.mem2 += 1;
                if self.mem2 == 2048 {
                    self.mem2_lvl += 1;
                    if self.mem.len() <= self.mem2_lvl {
                        self.mem.push(vec![0; 2048]);
                        self.mem_flag.push(vec![0; 2048]);
                    }
                    self.mem2 = 0;
                }
                return Ok(());
            }
            35 => {
                // "rm_3"
                self.mem3 += 1;
                if self.mem3 == 2048 {
                    self.mem3_lvl += 1;
                    if self.mem.len() <= self.mem3_lvl {
                        self.mem.push(vec![0; 2048]);
                        self.mem_flag.push(vec![0; 2048]);
                    }
                    self.mem3 = 0;
                }
                return Ok(());
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
                return Ok(());
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
                return Ok(());
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
                return Ok(());
            }
            39 => {
                // "eshop_1"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    return Err(format!(
                        "Eshop1 operation not allowed for EOS literals"
                    ));
                }
                self.mem[self.mem1_lvl][self.mem1] *=
                    self.mem[self.mem1_lvl][self.mem1];
                return Ok(());
            }
            40 => {
                // "eshop_2"
                if self.mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    return Err(format!(
                        "Eshop2 operation not allowed for EOS literals"
                    ));
                }
                self.mem[self.mem2_lvl][self.mem2] *=
                    self.mem[self.mem2_lvl][self.mem2];
                return Ok(());
            }
            41 => {
                // "nankari_gate_out_1"
                let test: u32 =
                    match self.mem[self.mem1_lvl][self.mem1].try_into() {
                        Ok(c) => c,
                        Err(_) => {
                            return Err(format!(
                                "No equivalent character for the code!"
                            ))
                        }
                    };
                let ch = match char::from_u32(test) {
                    Some(c) => c,
                    None => {
                        return Err(format!("Cannot convert into character!"))
                    }
                };
                print!("{}", ch);
                return Ok(());
            }
            42 => {
                // "nankari_gate_out_2"
                let test: u32 =
                    match self.mem[self.mem2_lvl][self.mem2].try_into() {
                        Ok(c) => c,
                        Err(_) => {
                            return Err(format!(
                                "No equivalent character for the code!"
                            ))
                        }
                    };
                let ch = match char::from_u32(test) {
                    Some(c) => c,
                    None => {
                        return Err(format!("Cannot convert into character!"))
                    }
                };
                print!("{}", ch);
                return Ok(());
            }
            43 => {
                // "airstrip_land_1"
                match io::stdin().read_line(&mut inp) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!("Failed to read line"));
                    }
                }

                let mut ptr_pos: usize = self.mem1;
                let mut ptr_lvl: usize = self.mem1_lvl;
                for c in inp.chars() {
                    let ascii = c as i32;
                    if ascii == 10 {
                        continue;
                    }
                    self.mem[ptr_lvl][ptr_pos] = ascii;
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                self.mem_flag[ptr_lvl][ptr_pos] = 1;
                return Ok(());
            }
            44 => {
                // "airstrip_land_2"
                match io::stdin().read_line(&mut inp) {
                    Ok(_) => {}
                    Err(_) => {
                        return Err(format!("Failed to read line"));
                    }
                }

                let mut ptr_pos: usize = self.mem2;
                let mut ptr_lvl: usize = self.mem2_lvl;

                for c in inp.chars() {
                    let ascii = c as i32;
                    if ascii == 10 {
                        continue;
                    }
                    self.mem[ptr_lvl][ptr_pos] = ascii;
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                self.mem_flag[ptr_lvl][ptr_pos] = 1;
                return Ok(());
            }
            45 => {
                // "airstrip_takeoff_1"
                let mut ptr_pos = self.mem1;
                let mut ptr_lvl = self.mem1_lvl;
                while self.mem_flag[ptr_lvl][ptr_pos] != 1 {
                    let ch =
                        match char::from_u32(self.mem[ptr_lvl][ptr_pos] as u32)
                        {
                            Some(c) => c,
                            None => {
                                return Err(format!(
                                    "Cannot convert into character!"
                                ))
                            }
                        };
                    print!("{}", ch);
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                println!();
                return Ok(());
            }
            46 => {
                // "airstrip_takeoff_2"
                let mut ptr_pos = self.mem2;
                let mut ptr_lvl = self.mem2_lvl;
                while self.mem_flag[ptr_lvl][ptr_pos] != 1 {
                    let ch =
                        match char::from_u32(self.mem[ptr_lvl][ptr_pos] as u32)
                        {
                            Some(c) => c,
                            None => {
                                return Err(format!(
                                    "Cannot convert into character!"
                                ))
                            }
                        };
                    print!("{}", ch);
                    ptr_pos += 1;
                    if ptr_pos == 2048 {
                        ptr_pos = 0;
                        ptr_lvl += 1;
                    }
                }
                println!();
                return Ok(());
            }
            47 => {
                // "pronite_1"
                self.mem[self.mem1_lvl][self.mem1] = 0;
                self.mem_flag[self.mem1_lvl][self.mem1] = 1;
                return Ok(());
            }
            48 => {
                // "pronite_2"
                self.mem_flag[self.mem2_lvl][self.mem2] = 1;
                self.mem[self.mem2_lvl][self.mem2] = 0;
                return Ok(());
            }
            49 => {
                // "events_1"
                if self.mem_flag[self.mem1_lvl][self.mem1] == 1 {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["events_1_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["events_1_f"];
                }
                return Ok(());
            }
            52 => {
                // "events_2"
                if self.mem_flag[self.mem2_lvl][self.mem2] == 1 {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["events_2_t"];
                } else {
                    self.prev_loc = self.curr_loc;
                    self.curr_loc = locations["events_2_f"];
                }
                return Ok(());
            }
            55 => {
                // "oat_stage"
                if increment_graph.contains_key(&(self.prev_loc, self.cond)) {
                    let change_by_ad: &i32 = match increment_graph
                        .get(&(self.prev_loc, self.cond))
                    {
                        Some(v) => v,
                        None => return Err(format!("Key not found!")),
                    };
                    let change_by: i32 = *change_by_ad;

                    self.cond += change_by;
                }
                return Ok(());
            }
            56 => {
                // "nankari_gate_in_1"
                let c: char = read!();
                self.mem[self.mem1_lvl][self.mem1] = c as i32;
                self.mem_flag[self.mem1_lvl][self.mem1] = 0;
                return Ok(());
            }
            57 => {
                // "nankari_gate_in_2"
                let c: char = read!();
                self.mem[self.mem2_lvl][self.mem2] = c as i32;
                self.mem_flag[self.mem2_lvl][self.mem2] = 0;
                return Ok(());
            }
            _ => return Err(format!("No such operation exists!")),
        }
    }

    pub fn travel(
        &mut self,
        locations: &HashMap<String, i32>,
        graph: &HashMap<i32, HashMap<i32, i32>>,
        increment_graph: &HashMap<(i32, i32), i32>,
        mut depth_limit: i32,
    ) -> Result<(), String> {
        while self.curr_loc != locations["finish"] && depth_limit > 0 {
            if self.curr_loc != locations["start"] {
                match self.perform_operation(
                    locations,
                    increment_graph,
                    self.curr_loc,
                ) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }

            if !graph[&self.curr_loc].contains_key(&self.cond) {
                return Err(format!(
                    "Stuck in landmark number {} with condition {}",
                    self.curr_loc, self.cond
                ));
            }

            self.prev_loc = self.curr_loc;
            self.curr_loc = graph[&self.curr_loc][&self.cond];

            depth_limit -= 1;
        }

        if self.curr_loc == locations["finish"] {
            return Ok(());
        } else if depth_limit < 0 {
            return Err(format!("Too many operations! Please check for infinite loops or other similar stack overflows."));
        } else {
            return Err(format!("Something went wrong!"));
        }
    }
}

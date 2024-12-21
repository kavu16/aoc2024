use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct ThreeBit {
    a_register: i64,
    b_register: i64,
    c_register: i64,
    idx: usize,
    program: Vec<i64>,
    cache: HashMap<(i64,i64,i64,usize), Vec<i64>>,
    caching: bool,
}

impl ThreeBit {
    fn new(data: &str, caching: bool) -> Self {
        let (registers, program) = data.split_once("\n\n").unwrap();
        let (a_register, b_register, c_register): (i64, i64, i64) = registers.lines()
            .map(|line| {
                let (_, num) = line.split_once(':').unwrap();
                num.trim().parse().unwrap()
            })
            .next_tuple().unwrap();

        let (_, program) = program.split_once(':').unwrap();
        let program: Vec<i64> = program.trim().split(',').flat_map(|n| n.parse()).collect();
        Self {
            a_register,
            b_register,
            c_register,
            idx: 0,
            program,
            cache: HashMap::new(),
            caching,
        }
    }

    fn op_code(&mut self, output: &mut Vec<i64>) {
        // println!("Current state:\n{self:?}");
        if self.caching {
            if let Some(out) = self.cache.get(&self.get_curr_state()) {
                output.extend(out);
                self.idx = usize::MAX;
                return;
            }
        }
        match self.program[self.idx] {
            0 => {
                self.a_register /= 2i64.pow(self.combo_operand(self.program[self.idx + 1]) as u32);
                self.idx += 2;
            }
            1 => {
                self.b_register ^= self.program[self.idx+1];
                self.idx += 2;
            }
            2 => {
                self.b_register = self.combo_operand(self.program[self.idx+1]) % 8;
                self.idx += 2;
            },
            3 => {
                if self.a_register > 0 {
                    self.idx = self.program[self.idx+1] as usize;
                } else {
                    self.idx += 2;
                }
            }
            4 => {
                self.b_register ^= self.c_register;
                self.idx += 2;
            }
            5 => {
                output.push(self.combo_operand(self.program[self.idx+1]) % 8);
                self.idx += 2;
            }
            6 => {
                self.b_register = self.a_register / 2i64.pow(self.combo_operand(self.program[self.idx + 1]) as u32);
                self.idx += 2; 
            }
            7 => {
                self.c_register = self.a_register / 2i64.pow(self.combo_operand(self.program[self.idx + 1]) as u32);
                self.idx += 2; 
            }
            _ => unreachable!("Unrecognized opcode"),
        }
    }

    fn combo_operand(&self, val: i64) -> i64 {
        match val {
            0..=3 => val,
            4 => self.a_register,
            5 => self.b_register,
            6 => self.c_register,
            _ => unreachable!("Unrecognized combo operand"),
        }
    }

    fn run(&mut self) -> Vec<i64> {
        let mut output: Vec<i64> = Vec::new();
        let curr_state = self.get_curr_state();
        while self.idx < self.program.len()-1 {
            self.op_code(&mut output);
        }
        if self.caching {
            self.cache.insert(curr_state, output.clone());
        }
        output
    }

    fn program(&self) -> Vec<i64> {
        self.program.clone()
    }

    fn get_curr_state(&self) -> (i64, i64, i64, usize) {
        (self.a_register, self.b_register, self.c_register, self.idx)
    }

    fn reset_state(&mut self, a_register: i64) {
        self.a_register = a_register;
        self.b_register = 0;
        self.c_register = 0;
        self.idx = 0;
    }
}

pub fn solve1(data: &str) -> String {
    let mut three_bit = ThreeBit::new(data, false);
    let res = three_bit.run().into_iter().join(",");
    println!("Day 17 Part 1 = {res}");
    res
}

pub fn solve2(data: &str) -> i64 {
    let mut three_bit = ThreeBit::new(data, true);
    let program = three_bit.program();
    println!("{program:?}");
    let n = program.len();
    let mut queue: VecDeque<(i64, usize)> = VecDeque::new();
    let seed = (1 << 3*(n-1))*3;
    queue.push_back((seed, n-1));
    
    let mut res = 0;
    while let Some((curr_seed, curr_lvl)) = queue.pop_back() {
        if curr_lvl == 0 {
            res = curr_seed;
            continue;
        }
        let pow8 = 1 << (3 * (curr_lvl-1));
        for i in 0..8 {
            three_bit.reset_state(curr_seed + i * pow8);
            let output = three_bit.run();
            if output[curr_lvl-1] == program[curr_lvl-1] {
                queue.push_back((curr_seed + i * pow8, curr_lvl - 1));
            }
        }
    }

    three_bit.reset_state(res);
    println!("{:?}", three_bit.run());
    
    println!("Day 17 Part 2 = {res}");
    res
}

#[test]
fn test() {
    let data = String::from(
"Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0"
    );

    let data2 = String::from(
"Register A: 117440
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0"
    );

    assert_eq!(solve1(&data), String::from("4,6,3,5,6,3,5,2,1,0"));
    assert_eq!(solve1(&data2), String::from("0,3,5,4,3,0"));

    assert_eq!(solve2(&data2), 117440);
}
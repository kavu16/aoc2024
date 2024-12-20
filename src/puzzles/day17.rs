use itertools::Itertools;

#[derive(Debug, Clone)]
struct ThreeBit {
    a_register: i64,
    b_register: i64,
    c_register: i64,
    idx: usize,
    program: Vec<i64>,
    output: Vec<i64>,
}

impl ThreeBit {
    fn new(data: &str) -> Self {
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
            output: Vec::new(),
        }
    }

    fn op_code(&mut self) {
        // println!("Current state:\n{self:?}");
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
                self.output.push(self.combo_operand(self.program[self.idx+1]) % 8);
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
            _ => panic!("Unrecognized opcode"),
        }
    }

    fn combo_operand(&self, val: i64) -> i64 {
        match val {
            0..=3 => val,
            4 => self.a_register,
            5 => self.b_register,
            6 => self.c_register,
            _ => panic!("Unrecognized combo operand"),
        }
    }

    fn run(&mut self) -> String {
        while self.idx < self.program.len()-1 {
            self.op_code();
        }
        self.output.iter().map(|&n| n.to_string()).join(",")
    }
}

pub fn solve1(data: &str) -> String {
    let mut three_bit = ThreeBit::new(data);
    let res = three_bit.run();
    println!("Day 17 Part 1 = {res}");
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
}
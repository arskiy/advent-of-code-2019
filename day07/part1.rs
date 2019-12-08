mod permutations;
use permutations::permutations;

static INPUT: &'static str = include_str!(r"input.txt");

const OP_ADD: i32 = 1;
const OP_MULT: i32 = 2;
const OP_STDIN: i32 = 3;
const OP_STDOUT: i32 = 4;
const OP_JMP_TRUE: i32 = 5;
const OP_JMP_FALSE: i32 = 6;
const OP_LT: i32 = 7;
const OP_EQ: i32 = 8;
const OP_HALT: i32 = 99;

const LOAD_POS: i32 = 0;
const LOAD_IMM: i32 = 1;

#[derive(Debug, Default)]
struct IntcodeVM {
    pc: usize,
    modes: i32,
    memory: Vec<i32>,
}

impl IntcodeVM {
    fn new(memory: Vec<i32>) -> Self {
        Self {
            pc: 0,
            modes: Default::default(),
            memory,
        }
    }

    fn load(&mut self) -> i32 {
        let val = self.memory[self.pc];
        let result = match self.modes % 10 {
            LOAD_POS => self.memory[val as usize],
            LOAD_IMM => val,
            _ => panic!("Unexpected indirection mode"),
        };
        self.pc += 1;
        self.modes /= 10;
        result
    }

    fn set(&mut self, val: i32) {
        let x = self.memory[self.pc] as usize;
        self.memory[x] = val;
        self.pc += 1;
    }

    fn run(&mut self, stdin: Vec<i32>) -> i32 {
        let mut i = 0;
        loop {
            self.modes = self.memory[self.pc];
            let op = self.modes % 100;
            self.modes /= 100;
            self.pc += 1;

            match op {
                OP_ADD => {
                    let val = self.load() + self.load();
                    self.set(val);
                }

                OP_MULT => {
                    let val = self.load() * self.load();
                    self.set(val);
                }

                OP_STDIN => {
                    self.set(stdin[i]);
                    println!("input: {}", stdin[i]);
                    i += 1;
                }

                OP_STDOUT => {
                    // println!("{}", self.load());
                    let x = self.load();
                    println!("output: {}", x);
                    return x;
                }

                OP_JMP_TRUE => {
                    let cond = self.load();
                    let jmp = self.load() as usize;
                    if cond != 0 {
                        self.pc = jmp;
                    }
                }
                
                OP_JMP_FALSE => {
                    let cond = self.load();
                    let jmp = self.load() as usize;
                    if cond == 0 {
                        self.pc = jmp;
                    }
                }

                OP_LT => {
                    let val = if self.load() < self.load() { 1 } else { 0 };
                    self.set(val);
                }

                OP_EQ => {
                    let val = if self.load() == self.load() { 1 } else { 0 };
                    self.set(val);
                }


                OP_HALT => return self.load(),

                x => panic!("Unknown opcode: {} at {}", x, self.pc),
            };
        }
    }
}

fn main() {
    let input: Vec<i32> = INPUT
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let perms = permutations(5).collect::<Vec<_>>();
    let mut biggest = 0;

    for perm in perms.iter() {
        let mut intcode = IntcodeVM::new(input.clone());
        let mut output = intcode.run(vec!(perm[0] as i32, 0));

        for i in 1..perm.len() {
            intcode = IntcodeVM::new(input.clone());
            output = intcode.run(vec!(perm[i] as i32, output));
        }
        
        if output > biggest {
            biggest = output;
        }
    }
    println!("biggest perm: {}", biggest);
}

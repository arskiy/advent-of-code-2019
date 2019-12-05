use std::fs;
// use std::io;

const ADD: i32 = 1;
const MULT: i32 = 2;
const STDIN: i32 = 3;
const STDOUT: i32 = 4;
const HALT: i32 = 99;

fn intcode(input: &mut Vec<i32>) -> i32 {
    let mut pc: usize = 0;
    while pc < input.len() {
        let n = input[pc];

        let opcode = n % 100;

        let first  = n / 100 % 10;
        let second = n / 1000 % 10;
        // let third  = n / 10000;

        match opcode {
            HALT => return input[0],
            ADD => {
                let x = input[pc + 3] as usize;
                let y = input[pc + 1] as i32;
                let z = input[pc + 2] as i32;

                match (first == 1, second == 1) {
                    (false, false) => input[x] = (input[y as usize] + input[z as usize]) as i32,
                    (false, true ) => input[x] = (input[y as usize] + z                ) as i32,
                    (true,  false) => input[x] = (y                 + input[z as usize]) as i32,
                    (true,  true ) => input[x] = (y                 + z                ) as i32,
                }
                pc += 4;
            },

            MULT => {
                let x = input[pc + 3] as usize;
                let y = input[pc + 1] as i32;
                let z = input[pc + 2] as i32;

                match (first == 1, second == 1) {
                    (false, false) => input[x] = (input[y as usize] * input[z as usize]) as i32,
                    (false, true ) => input[x] = (input[y as usize] * z                ) as i32,
                    (true, false ) => input[x] = (y                 * input[z as usize]) as i32,
                    (true, true  ) => input[x] = (y                 * z                ) as i32,
                }
                pc += 4;
            }

            STDIN => {
                /*
                let mut inp = String::new();

                println!("Please input a number: ");
                io::stdin().read_line(&mut inp)
                    .expect("Failed to read line");


                input[x] = inp
                    .trim()
                    .parse::<i32>()
                    .expect("error parsing input.");
                    */
                let x = input[pc + 1] as usize;
                input[x] = 1;

                pc += 2;
            }

            STDOUT => {
                let x = input[pc + 1] as usize;

                if first == 1 {
                    println!("The Intcode computer spoke: {}", x);
                } else {
                    println!("The Intcode computer spoke: {}", input[x]);
                }

                pc += 2;
            }

            _ => panic!("Unknown opcode: {}, at {}", opcode, pc),
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string("../input.txt")
        .expect("Error reading file");

    let input = input
        .trim()
        .split(",")
        .collect::<Vec<_>>();

    let mut input: Vec<i32> = input.into_iter()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    intcode(&mut input);
}

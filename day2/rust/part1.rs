use std::fs;

const ADD: usize = 1;
const MULT: usize = 2;
const HALT: usize = 99;

// What value is left at position 0 after the program halts?

fn restore(input: &mut Vec<usize>) -> usize {
    for pc in (0..input.len()).step_by(4) {
        match input[pc] {
            HALT => return input[0],
            ADD => {
                let x = input[pc + 3];
                let y = input[pc + 1];
                let z = input[pc + 2];
                input[x] = input[y] + input[z];
            },

            MULT => {
                let x = input[pc + 3];
                let y = input[pc + 1];
                let z = input[pc + 2];
                input[x] = input[y] * input[z];
            }
            _ => continue,
        }
    }
    0
}

fn main() {
    let mut input = fs::read_to_string("../input.txt")
        .expect("Error reading file");

    input.pop();
    // println!("{:?}", input);
    let input = input
        .split(",")
        .collect::<Vec<_>>();

    // println!("{:?}", input);

    let mut input: Vec<usize> = input.into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    input[1] = 12;
    input[2] = 2;

    println!("{}", restore(&mut input));
}

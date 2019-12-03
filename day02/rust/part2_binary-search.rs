use std::fs;
use std::{thread, time};

const TARGET: i32 = 19690720;

const ADD: i32 = 1;
const MULT: i32 = 2;
const HALT: i32 = 99;

fn restore(x: &Vec<i32>) -> i32 {
    let mut input = x.clone();
    for pc in (0..input.len()).step_by(4) {
        match input[pc] {
            HALT => return input[0],
            ADD => {
                let x = input[pc + 3] as usize;
                let y = input[pc + 1] as usize;
                let z = input[pc + 2] as usize;
                input[x] = input[y] + input[z];
            },

            MULT => {
                let x = input[pc + 3] as usize;
                let y = input[pc + 1] as usize;
                let z = input[pc + 2] as usize;
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

    let input = input
        .trim()
        .split(",")
        .collect::<Vec<_>>();

    let input: Vec<i32> = input.into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut upper = 100;
    let mut lower = 0;

    loop {
        // binary searches the result in O(log upper_bound) => O(2)
        let pivot = (upper - lower) / 2 + lower;
        let mut copy = input.clone();
        
        copy[1] = pivot;

        let res = restore(&copy);
        
        if TARGET - res < 100 && TARGET - res > 0 {
            println!("RESULT FOUND: {}", pivot * 100 + (TARGET - res));
            break;
        }

        else if res < TARGET {
            lower = pivot;
        }

        else {
            upper = pivot;
        }

        println!("pivot: {}, res - target: {}, res: {}, target: {}", pivot, res - TARGET, res, TARGET);
    }
}

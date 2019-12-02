use std::fs;

const FINAL: usize = 19690720;

fn restore(x: &Vec<usize>) -> usize {
    let mut input = x.clone();
    for i in (0..input.len()).step_by(4) {
        if input[i] == 99 {
            // What value is left at position 0 after the program halts?
            return input[0];
        }
        else if input[i] == 1 {
            let x = input[i + 3];
            let y = input[i + 1];
            let z = input[i + 2];
            input[x] = input[y] + input[z];
        }
        else if input[i] == 2 {
            let x = input[i + 3];
            let y = input[i + 1];
            let z = input[i + 2];
            input[x] = input[y] * input[z];
        }
    }
    0
}

fn main() {
    let mut input = fs::read_to_string("input.txt")
        .expect("Error reading file");

    input.pop();
    // println!("{:?}", input);
    let input = input
        .split(",")
        .collect::<Vec<_>>();

    // println!("{:?}", input);

    let input: Vec<usize> = input.into_iter()
        .map(|s| s.parse().unwrap())
        .collect();

    // To do this, before running the program, replace position 1 with the value 12 and replace position 2 with the value 2.
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut copy = input.clone();
            copy[1] = noun;
            copy[2] = verb;
            if restore(&copy) == FINAL {
                println!("{}", noun * 100 + verb);
            }
        }
    }
}

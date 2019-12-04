const MIN: u32 = 356261;
const MAX: u32 = 846303;

fn password_check(input: u32) -> bool {
    let mut y = input;
    let mut prev = 10;
    let mut is_double = false;

    while y != 0 {
        let curr = y % 10;

        if curr > prev {
            return false;
        }

        else if curr == prev {
            is_double = true;
        }

        prev = curr;
        y /= 10;
    }
    
    is_double
}

fn main() {
    let mut count = 0;

    for i in MIN..=MAX {
        if password_check(i) {
            println!("{}", i);
            count += 1;
        }
    }

    println!("count: {}", count);
}

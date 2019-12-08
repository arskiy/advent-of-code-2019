use std::cmp;

static INPUT: &'static str = include_str!(r"input.txt");

fn main() {
    let mut v = vec![];
    let mut cur = INPUT.clone().trim();
    while !cur.is_empty() {
        let (chunk, rest) = cur.split_at(cmp::min(25 * 6, cur.len()));
        v.push(chunk);
        cur = rest;
    }

    let mut smallest = 0;
    let mut index = 18;
    
    for i in 0..v.len() {
        let y = v[i].chars().filter(|&n| n == '0').count();
        println!("{}", y);
        if y < smallest {
            smallest = y;
            index = i;
        }
    }

    println!("{} : {}", index, v[index]);

    let x = v[index].chars().filter(|&n| n == '1').count();
    let y = v[index].chars().filter(|&n| n == '2').count();

    println!("{}", x * y);
}

// BEWARE!!! You shall not run this or you shall suffer the consequences.
// This shall run for a whole 40 minutes in your machine if you shall run this.
// YOU SHALL CONSIDER YOURSELF WARNED!

use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    pub x: i32,
    pub y: i32,
    pub steps: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, steps: i32) -> Self {
        Self {
            x,
            y,
            steps,
        }
    }

    pub fn eq(&self, b: &Point) -> bool {
        self.x == b.x && self.y == b.y
    }

    pub fn convert(input: Vec<String>) -> Vec<Point> {
        let mut p = Point::new(0, 0, 0);
        let mut points: Vec<Point> = vec!();
        
        let mut input_iter = input.clone().into_iter();
        while let Some(line) = &mut input_iter.next() {
            println!("converting... {} - {:?}", line, p);
            let dir = line.remove(0);
            let x = line.parse::<i32>().unwrap();

            match dir {
                'U' => {
                    let mut i = p.y;
                    p.y += x;
                    while i != p.y {
                        p.steps += 1;
                        points.push(Point::new(p.x, i, p.steps));
                        i += 1;
                    }
                }

                'R' => {
                    let mut i = p.x;
                    p.x += x;
                    while i != p.x {
                        p.steps += 1;
                        points.push(Point::new(i, p.y, p.steps));
                        i += 1;
                    }
                }

                'D' => {
                    let mut i = p.y;
                    p.y -= x;
                    while i != p.y {
                        p.steps += 1;
                        points.push(Point::new(p.x, i, p.steps));
                        i -= 1;
                    }
                }

                'L' => {
                    let mut i = p.x;
                    p.x -= x;
                    while i != p.x {
                        p.steps += 1;
                        points.push(Point::new(i, p.y, p.steps));
                        i -= 1;
                    }
                }

                x => panic!(format!("Unexpected direction: {}", x)),
            }
            p.steps += 1;
        }

        points.remove(0);
        points
    }

    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn grab_intersections(a: &mut Vec<Point>, b: &mut Vec<Point>) -> Vec<Point> {
    let mut intersections: Vec<Point> = vec!();
    // runs 2.2E10 times....
    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i].eq(&b[j]) {
                a[i].steps = (i + j + 2) as i32;
                println!("Intersection found: {:?}", a[i]);
                intersections.push(a[i]);
            }
        }
    }
    intersections
}

fn calc(input: String) -> (i32, i32) {
    let instant = Instant::now();
    let input1: String;
    let input2: String;
    
    {
        let input = input
            .clone()
            .split("\n")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        input1 = input[0].clone();
        input2 = input[1].clone();
    }

    let input1 = input1
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let input2 = input2
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    
    let intersections = grab_intersections(&mut Point::convert(input1), &mut Point::convert(input2));
    
    let mut dists = intersections
        .clone()
        .into_iter()
        .map(|x| x.dist())
        .collect::<Vec<i32>>();

    let mut step = intersections
        .into_iter()
        .map(|x| x.steps)
        .collect::<Vec<i32>>();

    dists.sort();
    step.sort();

    println!("time: {:?}", instant.elapsed());

    (*dists.first().unwrap(), *step.first().unwrap())
}

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Error reading file");
    let x = calc(input);

    println!("Part 1: {:?}", x.0);
    println!("Part 2: {:?}", x.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let test = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83".to_string();
        assert_eq!((159, 610), calc(test));
    }

    #[test]
    fn example2() {
        let test = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7".to_string();
        assert_eq!((135, 410), calc(test));
    }
}

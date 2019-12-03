use std::fs;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn eq(&self, b: &Point) -> bool {
        self.x == b.x && self.y == b.y
    }

    pub fn convert(input: Vec<String>) -> Vec<Point> {
        let mut p = Point::new(0, 0);
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
                        points.push(Point::new(p.x, i));
                        i += 1;
                    }
                }

                'R' => {
                    let mut i = p.x;
                    p.x += x;
                    while i != p.x {
                        points.push(Point::new(i, p.y));
                        i += 1;
                    }
                }

                'D' => {
                    let mut i = p.y;
                    p.y -= x;
                    while i != p.y {
                        points.push(Point::new(p.x, i));
                        i -= 1;
                    }
                }

                'L' => {
                    let mut i = p.x;
                    p.x -= x;
                    while i != p.x {
                        points.push(Point::new(i, p.y));
                        i -= 1;
                    }
                }

                x => panic!(format!("Unexpected direction: {}", x)),
            }
        }

        points.remove(0);
        points
    }

    pub fn dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

fn grab_intersections(a: Vec<Point>, b: Vec<Point>) -> Vec<Point> {
    println!("a.len: {} - b.len: {}", a.len(), b.len());
    let mut intersections: Vec<Point> = vec!();

    for i in 0..a.len() {
        for j in 0..b.len() {
            if a[i].eq(&b[j]) {
                println!("Intersection found: {:?}", a[i]);
                intersections.push(a[i]);
            }
        }
    }

    intersections
}

fn main() {
    let instant = Instant::now();
    let input1: String;
    let input2: String;
    
    {
        let input = fs::read_to_string("input.txt")
            .expect("Error reading file");

        let input = input
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
    
    let intersections = grab_intersections(Point::convert(input1), Point::convert(input2));
    
    let mut dists = intersections
        .into_iter()
        .map(|x| x.dist())
        .collect::<Vec<i32>>();

    dists.sort();

    println!("RESULT FOUND: {:?}", dists.first());
    println!("time: {:?}", instant.elapsed());
}


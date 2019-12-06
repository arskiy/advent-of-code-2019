use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!(r"input.txt");

fn parse(s: &str) -> (&str, &str) {
    let mut splitter = s.split(")");

    (splitter.next().unwrap(), splitter.next().unwrap())
}

fn orbitters_of<'a>(orbittee: &'a str, map: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    map.get(&orbittee)
        .map(|orbits| {
            orbits
                .iter()
                .fold(0, |count, orbitter| count + 1 + orbitters_of(orbitter, map))
        })
        .unwrap_or(0)
}

fn orbitters_wrap<'a>(orbits: &HashMap<&'a str, HashSet<&'a str>>) -> usize {
    orbits
        .iter()
        .map(|(orbittee, _)| orbitters_of(orbittee, &orbits))
        .sum()
}

fn main() {
    let mut orbits: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in INPUT.lines() {
        let (orbittee, orbitter) = parse(line);
        orbits
            .entry(orbittee)
            .or_insert_with(HashSet::default)
            .insert(orbitter);
        orbits.entry(orbitter).or_insert_with(HashSet::default);
    }

    let res = orbitters_wrap(&orbits);

    println!("{}", res);
}

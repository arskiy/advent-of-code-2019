use std::collections::{HashMap, HashSet};

static INPUT: &'static str = include_str!(r"input.txt");

fn parse(s: &str) -> (&str, &str) {
    let mut splitter = s.split(")");

    (splitter.next().unwrap(), splitter.next().unwrap())
}

fn orbittee_of<'a>(orbitter: &'a str, map: &HashMap<&'a str, HashSet<&'a str>>) -> Option<&'a str> {
    map.iter()
        .find(|(_, orbitters)| orbitters.contains(orbitter))
        .map(|(orbittee, _)| *orbittee)
}

fn contains_orbitter(
    orbittee: &str,
    orbitter: &str,
    map: &HashMap<&str, HashSet<&str>>,
) -> Option<usize> {
    map.get(orbittee).and_then(|orbitters| {
        orbitters.get(orbitter).map_or_else( || {
                orbitters
                    .iter()
                    .find_map(|orbittee| contains_orbitter(orbittee, orbitter, map))
                    .map(|depth| depth + 1)
            },
            |_| Some(1),
        )
    })
}

fn find_res(orbits: &HashMap<&str, HashSet<&str>>) -> usize {
    std::iter::successors(Some("SAN"), |orbitter| orbittee_of(orbitter, orbits))
        .enumerate()
        .find_map(|(count, orbitter)| Some((count, contains_orbitter(orbitter, "YOU", orbits)?)))
        .map(|(count, depth)| count + depth - 2)
        .unwrap()
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

    println!("result found: {}", find_res(&orbits));
}

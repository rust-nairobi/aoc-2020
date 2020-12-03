use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::result::Result;
use std::time::Instant;

use itertools::Itertools;

fn get_entries() -> HashSet<u32> {
    let file = File::open("input_1.txt").expect("Unable to read file");
    let lines = io::BufReader::new(file).lines();

    lines
        .filter_map(Result::ok)
        .map(|x| x.parse().unwrap())
        .collect()
}

fn part_1(entries: &HashSet<u32>) {
    println!("Part 1:");

    let (entry_1, entry_2) = entries
        .iter()
        .map(|entry| (entry, 2020 - entry))
        .filter(|x| entries.contains(&x.1))
        .next()
        .unwrap();

    println!("Values: {} and {}", entry_1, entry_2);
    println!("Answer: {}", entry_1 * entry_2);
}

fn part_2(entries: &HashSet<u32>) {
    println!("Part 2");

    let combinations = entries.iter().tuple_combinations::<(&u32, &u32)>();
    let addition = combinations.map(|x| (x.0, x.1, x.0 + x.1));
    let subtraction = addition
        .filter(|x| x.2 < 2020)
        .map(|x| (x.0, x.1, 2020 - x.2));

    let (entry_1, entry_2, entry_3) = subtraction
        .filter(|x| entries.contains(&x.2))
        .next()
        .unwrap();

    println!("Values: {}, {} and {}", entry_1, entry_2, entry_3);
    println!("Answer: {}", entry_1 * entry_2 * entry_3);
}

fn main() {
    let entries = get_entries();

    let mut start = Instant::now();
    part_1(&entries);
    println!("Time Taken: {:?}", start.elapsed());

    println!("---------------");

    start = Instant::now();
    part_2(&entries);
    println!("Time Taken: {:?}", start.elapsed());
}

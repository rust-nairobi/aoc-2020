use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn read_lines() -> Vec<String> {
    let file = File::open("input_2.txt").expect("Unable to read file");
    io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .collect()
}

fn is_valid_password(input: &&String) -> bool {
    let (policy, mut char_str, password) = input.split(" ").collect_tuple().unwrap();

    char_str = char_str.strip_suffix(':').unwrap();
    let character: char = char_str.parse().unwrap();

    let char_count = password.chars().filter(|x| x == &character).count();

    let (min, max) = policy
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    min <= char_count && char_count <= max
}

fn part_1(lines: &Vec<String>) {
    println!("Part 1:");

    let valid_count = lines.iter().filter(is_valid_password).count();
    println!("Answer: {} passwords", valid_count);
}

fn is_valid_password_2(input: &&String) -> bool {
    let (policy, mut char_str, password) = input.split(" ").collect_tuple().unwrap();
    char_str = char_str.strip_suffix(':').unwrap();

    let (first, second) = policy
        .split("-")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_tuple()
        .unwrap();

    let no_of_matches = password
        .match_indices(char_str)
        .map(|result| result.0 + 1)
        .filter(|index| index == &first || index == &second)
        .count();

    no_of_matches == 1
}

fn part_2(lines: &Vec<String>) {
    println!("Part 2:");

    let valid_count = lines.iter().filter(is_valid_password_2).count();
    println!("Answer: {} passwords", valid_count)
}

fn main() {
    let lines = read_lines();
    part_1(&lines);

    println!("----------");

    part_2(&lines);
}

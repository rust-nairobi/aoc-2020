// --- Day 9: Encoding Error ---
//
// https://adventofcode.com/2020/day/9
use std::fs::File;
use std::io::{self, BufRead};

type Cypher = Vec<usize>;
fn find_irreducible(v: &[usize]) -> Option<usize> {
    let l = v.len() - 1;
    for i in &v[..l] {
        for j in &v[..l] {
            if i + j == v[l] {
                return None;
            }
        }
    }
    Some(v[l])
}

fn part_one(cypher: &Cypher, length: usize) -> usize {
    for window in cypher.windows(length) {
        if let Some(num) = find_irreducible(window) {
            return num;
        }
    }
    0
}

fn part_two(cypher: &Cypher, length: usize) -> usize {
    let n = part_one(cypher, length);
    let mut sz = 2;
    loop {
        for window in cypher.windows(sz) {
            let window: Vec<_> = window.iter().collect();
            if window.iter().fold(0, |acc, x| acc + *x) == n {
                return *window.iter().min().unwrap() + *window.iter().max().unwrap();
            }
        }
        sz += 1;
    }
}

fn main() {
    let cypher = load_input("input.txt").unwrap();

    println!("Part One: {} ", part_one(&cypher, 25 + 1));
    println!("Part Two: {} ", part_two(&cypher, 25 + 1));
}

fn load_input(fname: &str) -> io::Result<Cypher> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<usize>().ok())
        .collect())
}

#[test]
fn test() {
    let cypher = load_input("test.txt").unwrap();
    assert_eq!(127, part_one(&cypher, 5 + 1));
    assert_eq!(62, part_two(&cypher, 5 + 1));
}

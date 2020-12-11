// --- Day 10: Adapter Array ---
//
// https://adventofcode.com/2020/day/10
use std::fs::File;
use std::io::{self, BufRead};

type Joltages = Vec<usize>;

fn part_one(joltages: &[usize]) -> usize {
    let mut three_count = 0;
    let mut one_count = 0;

    for diff in joltages.windows(2).map(|w| w[1] - w[0]) {
        if diff == 3 {
            three_count += 1;
        } else if diff == 1 {
            one_count += 1;
        }
    }
    three_count * one_count
}

fn part_two(joltages: &[usize]) -> usize {
    let mut arrangements = vec![0; *joltages.iter().max().unwrap() + 1];
    arrangements[0] = 1;
    for jolt in joltages {
        for source in jolt.saturating_sub(3)..*jolt {
            arrangements[*jolt] += arrangements[source];
        }
    }
    *arrangements.last().unwrap()
}
fn main() {
    let joltages = load_input("input.txt").unwrap();

    println!("Part One: {} ", part_one(&joltages));
    println!("Part Two: {} ", part_two(&joltages));
}

fn load_input(fname: &str) -> io::Result<Joltages> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut js: Vec<_> = buf
        .lines()
        .filter_map(|x| x.ok())
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();
    js.push(0);
    js.sort_unstable();
    js.push(js[js.len() - 1] + 3);
    Ok(js)
}

#[test]
fn test() {
    let joltages = load_input("test.txt").unwrap();
    assert_eq!(220, part_one(&joltages));
    assert_eq!(19208, part_two(&joltages));
}

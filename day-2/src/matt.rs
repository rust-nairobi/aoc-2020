// --- Day 2: Password Philosophy--
//
// https://adventofcode.com/2020/day/2
use std::fs::File;
use std::io::{self, BufRead};

struct PwdEntry {
    low: i32,
    high: i32,
    pat: char,
    pwd: String,
}

impl PwdEntry {
    fn new(tup: (i32, i32, char, String)) -> PwdEntry {
        PwdEntry {
            low: tup.0,
            high: tup.1,
            pat: tup.2,
            pwd: tup.3,
        }
    }
}

fn valid_passwords(passwords: &[PwdEntry]) -> i32 {
    let mut valid = 0;
    for entry in passwords {
        if is_valid(entry) {
            valid += 1;
        }
    }
    valid
}
fn is_valid(entry: &PwdEntry) -> bool {
    let m = entry.pwd.matches(entry.pat).count() as i32;
    if m >= entry.low && m <= entry.high {
        true
    } else {
        false
    }
}
fn main() {
    let input = input().unwrap();

    println!("Solution: {}", valid_passwords(&input));
}

fn input() -> io::Result<Vec<PwdEntry>> {
    let mut input = vec![];
    let file = File::open("src/matt.txt")?;
    let buf = io::BufReader::new(file);
    for item in buf.lines() {
        let line = item?;
        input.push(parse_input_line(&line).unwrap())
    }
    Ok(input)
}

fn parse_input_line(line: &str) -> Result<PwdEntry, std::num::ParseIntError> {
    let parts: Vec<_> = line.split(' ').collect();
    let low_high: Vec<_> = parts[0].split('-').collect();
    let low: i32 = low_high[0].parse()?;
    let high: i32 = low_high[1].parse()?;
    let chr: char = parts[1].split(':').collect::<Vec<_>>()[0]
        .chars()
        .next()
        .unwrap();
    let pwd = parts[2].to_string();

    Ok(PwdEntry::new((low, high, chr, pwd)))
}

#[test]
fn test_matt() {
    assert_eq!(
        valid_passwords(&vec![
            PwdEntry::new((1, 3, 'a', "abcde".to_string())),
            PwdEntry::new((1, 3, 'b', "cdefg".to_string())),
            PwdEntry::new((2, 9, 'c', "ccccccccc".to_string()))
        ]),
        2
    )
}

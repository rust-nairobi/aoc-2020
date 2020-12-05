// --- Day 4: Passport Processing ---
//
// https://adventofcode.com/2020/day/4
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    Byr,
    Iyr,
    Hgt,
    Hcl,
    Ecl,
    Pid,
    Eyr,
    Cid,
}
#[derive(Debug)]
struct Passport(HashMap<Field, String>);
impl Passport {
    fn is_valid(&self) -> bool {
        self.0.len() == 8 || (self.0.len() == 7 && !self.0.contains_key(&Field::Cid))
    }

    fn is_valid_strict(&self) -> bool {
        self.is_valid() && {
            self.0
                .iter()
                .map(|fv| is_field_valid(fv.0, fv.1))
                .fold(true, |acc, x| acc && x)
        }
    }

    fn from_seq(seq: &Vec<String>) -> Passport {
        let mut pass = HashMap::new();
        for line in seq {
            for part in line.split(' ') {
                let kv: Vec<_> = part.splitn(2, ':').collect();
                let f = match kv[0] {
                    "byr" => Field::Byr,
                    "iyr" => Field::Iyr,
                    "eyr" => Field::Eyr,
                    "hgt" => Field::Hgt,
                    "hcl" => Field::Hcl,
                    "ecl" => Field::Ecl,
                    "pid" => Field::Pid,
                    "cid" => Field::Cid,
                    _ => unreachable!(),
                };
                pass.insert(f, kv[1].to_string());
            }
        }
        Passport(pass)
    }
}

fn is_field_valid(f: &Field, val: &str) -> bool {
    match f {
        Field::Byr => {
            if let Ok(num) = val.parse::<i32>() {
                num >= 1920 && num <= 2002
            } else {
                false
            }
        }
        Field::Iyr => {
            if let Ok(num) = val.parse::<i32>() {
                num >= 2010 && num <= 2020
            } else {
                false
            }
        }
        Field::Eyr => {
            if let Ok(num) = val.parse::<i32>() {
                num >= 2020 && num <= 2030
            } else {
                false
            }
        }
        Field::Hgt => {
            if val.ends_with("cm") {
                if let Ok(num) = val.trim_end_matches("cm").parse::<i32>() {
                    num >= 150 && num <= 193
                } else {
                    false
                }
            } else if val.ends_with("in") {
                if let Ok(num) = val.trim_end_matches("in").parse::<i32>() {
                    num >= 59 && num <= 76
                } else {
                    false
                }
            } else {
                false
            }
        }
        Field::Hcl => {
            val.len() == 7
                && val.chars().nth(0).unwrap() == '#'
                && val
                    .chars()
                    .skip(1)
                    .map(|c| c.is_digit(10) || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c))
                    .fold(true, |acc, x| acc && x)
        }
        Field::Ecl => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&val),
        Field::Pid => {
            val.len() == 9
                && val
                    .chars()
                    .map(|c| c.is_digit(10))
                    .fold(true, |acc, x| acc && x)
        }
        Field::Cid => true,
    }
}

fn main() {
    let passports = input("src/matt.txt").unwrap();

    println!(
        "Part One: {} ",
        passports.iter().filter(|x| x.is_valid()).count(),
    );
    println!(
        "Part Two: {} ",
        passports.iter().filter(|x| x.is_valid_strict()).count(),
    );
}

fn input(fname: &str) -> io::Result<Vec<Passport>> {
    let mut input = vec![];
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut p = vec![];
    for item in buf.lines() {
        let line = item?;
        if line.is_empty() {
            input.push(Passport::from_seq(&p));
            p.clear();
        } else {
            p.push(line);
        }
    }
    input.push(Passport::from_seq(&p));
    Ok(input)
}

#[test]
fn test_matt() -> io::Result<()> {
    let valid = input("src/matt_test.txt")?
        .iter()
        .filter(|x| x.is_valid())
        .count();
    assert_eq!(valid, 2);

    Ok(())
}

#[test]
fn test_valid() -> io::Result<()> {
    let valid = input("src/matt_test_valid.txt")?
        .iter()
        .filter(|x| x.is_valid_strict())
        .count();
    assert_eq!(valid, 4);

    Ok(())
}

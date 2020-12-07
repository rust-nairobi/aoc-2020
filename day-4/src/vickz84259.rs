use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
enum Entry {
    StrVal(String),
    IntVal(u32),
}

#[derive(Default, Debug)]
struct Passport {
    birth_year: Option<Entry>,
    issue_year: Option<Entry>,
    exp_year: Option<Entry>,
    height: Option<Entry>,
    hair_color: Option<Entry>,
    eye_color: Option<Entry>,
    pid: Option<Entry>,
    cid: Option<Entry>,
}

#[derive(Debug)]
struct PassParseError;

impl fmt::Display for PassParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Invalid Passport String")
    }
}

impl Passport {
    fn new() -> Self {
        Default::default()
    }

    fn is_valid(&self) -> bool {
        // Checks whether all entries exist

        let test = self
            .birth_year
            .as_ref()
            .zip(self.issue_year.as_ref())
            .zip(self.exp_year.as_ref())
            .zip(self.height.as_ref())
            .zip(self.hair_color.as_ref())
            .zip(self.eye_color.as_ref())
            .zip(self.pid.as_ref());

        test.is_some()
    }

    fn validate_ints(value: u32, lower: u32, higher: u32) -> bool {
        value >= lower && value <= higher
    }

    fn validate_limits(value: &Option<Entry>, lower: u32, higher: u32) -> bool {
        use Entry::IntVal;
        match value {
            Some(IntVal(value)) => Passport::validate_ints(*value, lower, higher),
            _ => false,
        }
    }

    fn validate_height(value: &Option<Entry>) -> bool {
        use Entry::StrVal;
        match value {
            Some(StrVal(value_str)) => {
                let index = value_str.chars().count() - 2;
                let slice = &value_str[..index];

                match slice.parse::<u32>() {
                    Ok(height) => match &value_str[index..] {
                        "cm" => Passport::validate_ints(height, 150, 193),
                        "in" => Passport::validate_ints(height, 59, 76),
                        _ => false,
                    },
                    Err(_) => false,
                }
            }
            _ => false,
        }
    }

    fn validate_hair_color(value: &Option<Entry>) -> bool {
        use Entry::StrVal;
        match value {
            Some(StrVal(value_str)) => {
                value_str.starts_with("#")
                    && value_str[1..]
                        .chars()
                        .map(|c| "0123456789abcdef".contains(c))
                        .all(|x| x)
            }
            _ => false,
        }
    }

    fn validate_eye_color(value: &Option<Entry>) -> bool {
        use Entry::StrVal;
        match value {
            Some(StrVal(value_str)) => {
                let slice = &value_str[..];
                vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&slice)
            }
            _ => false,
        }
    }

    fn validate_id(value: &Option<Entry>) -> bool {
        use Entry::StrVal;
        match value {
            Some(StrVal(id_str)) => match id_str.parse::<u32>() {
                Ok(_) => id_str.chars().count() == 9,
                Err(_) => false,
            },
            _ => false,
        }
    }

    fn validate(&self) -> bool {
        vec![
            Passport::validate_limits(&self.birth_year, 1920, 2002),
            Passport::validate_limits(&self.issue_year, 2010, 2020),
            Passport::validate_limits(&self.exp_year, 2020, 2030),
            Passport::validate_height(&self.height),
            Passport::validate_hair_color(&self.hair_color),
            Passport::validate_eye_color(&self.eye_color),
            Passport::validate_id(&self.pid),
        ]
        .iter()
        .all(|x| *x)
    }
}

impl FromStr for Passport {
    type Err = PassParseError;

    fn from_str(s: &str) -> Result<Passport, PassParseError> {
        let pass_str: String = s.split('\n').intersperse(" ").collect();

        let fields = pass_str.split_whitespace().map(|field| {
            field
                .split(":")
                .collect_tuple::<(&str, &str)>()
                .ok_or(PassParseError)
        });

        let mut passport = Passport::new();

        for field in fields {
            let field = match field {
                Ok(field) => field,
                Err(e) => return Err(e),
            };

            let entry = field.1.parse::<u32>().map_or_else(
                |_| Some(Entry::StrVal(field.1.to_string())),
                |value| Some(Entry::IntVal(value)),
            );

            match field.0 {
                "byr" => passport.birth_year = entry,
                "iyr" => passport.issue_year = entry,
                "eyr" => passport.exp_year = entry,
                "hgt" => passport.height = entry,
                "hcl" => passport.hair_color = entry,
                "ecl" => passport.eye_color = entry,
                "pid" => passport.pid = Some(Entry::StrVal(field.1.to_string())),
                "cid" => passport.cid = entry,
                _ => return Err(PassParseError),
            }
        }
        Ok(passport)
    }
}

fn get_passports() -> Vec<Passport> {
    let file = File::open("vickz84259.txt").expect("Unable to read file");
    let mut reader = io::BufReader::new(file);

    let mut vector: Vec<Passport> = Vec::new();
    let mut buf_1 = String::new();
    let mut buf_2 = String::new();

    loop {
        let no_of_bytes = reader.read_line(&mut buf_1).unwrap_or(0);

        buf_2.push_str(&buf_1);

        if &buf_1 == "\n" || &buf_1 == "" {
            vector.push(buf_2.parse().unwrap());
            buf_2.clear();
        }

        buf_1.clear();

        if no_of_bytes == 0 {
            break;
        }
    }
    vector
}

fn part_1(passports: &Vec<Passport>) {
    let valid_count = passports
        .iter()
        .filter(|passport| passport.is_valid())
        .count();

    println!("Valid passports: {}", valid_count);
}

fn part_2(passports: &Vec<Passport>) {
    let valid_count = passports
        .iter()
        .filter(|passport| passport.validate())
        .count();

    println!("Valid passports: {}", valid_count);
}

fn main() {
    let passports = get_passports();

    println!("Part 1: \n----------");
    part_1(&passports);

    println!("----------");
    println!("Part 2: \n----------");
    part_2(&passports);
}

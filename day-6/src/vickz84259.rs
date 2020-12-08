use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::mem::size_of;
use std::str::FromStr;

type Groups = Vec<Group>;

#[derive(Debug)]
struct Group {
    number: u32,
    questions: HashMap<char, u32>,
}

impl Group {
    fn new() -> Self {
        Group {
            number: 0,
            questions: HashMap::new(),
        }
    }
}

impl FromStr for Group {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Group, Self::Err> {
        let mut group = Group::new();

        s.split_whitespace().for_each(|line| {
            group.number += 1;
            line.chars().for_each(|c| {
                let value = group.questions.entry(c).or_insert(0);
                *value += 1;
            });
        });
        Ok(group)
    }
}

fn get_groups() -> Groups {
    let file = File::open("input.txt").expect("Unable to open file");
    let mut reader = io::BufReader::new(file);

    let mut groups: Groups = Vec::with_capacity(25 * size_of::<Group>());
    let mut reader_buffer = String::with_capacity(30);
    let mut group_buffer = String::with_capacity(30 * 5);

    loop {
        let no_of_bytes = reader.read_line(&mut reader_buffer).unwrap_or(0);

        group_buffer.push_str(&reader_buffer);
        if &reader_buffer == "\n" || &reader_buffer == "" {
            groups.push(group_buffer.parse().unwrap());
            group_buffer.clear();
        }

        reader_buffer.clear();
        if no_of_bytes == 0 {
            break;
        }
    }
    groups
}

fn part_1(groups: &Groups) {
    let sum: usize = groups.iter().map(|group| group.questions.len()).sum();
    println!("Answer: {}", sum);
}

fn part_2(groups: &Groups) {
    let sum: usize = groups
        .iter()
        .map(|group| {
            group
                .questions
                .iter()
                .filter(|entry| *entry.1 == group.number)
                .count()
        })
        .sum();
    println!("Answer: {}", sum);
}

fn main() {
    let groups = get_groups();

    println!("Part 1: \n----------");
    part_1(&groups);

    println!("----------");
    println!("Part 2: \n----------");
    part_2(&groups);
}

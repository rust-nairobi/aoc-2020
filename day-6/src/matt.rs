// --- Day 6: Custom Customs ---
//
// https://adventofcode.com/2020/day/6
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};

type Answers = HashSet<char>;
type GroupAnswers = Vec<Answers>;
enum AnswersType {
    Intersection,
    Union,
}

fn count_common_answers(mut group: GroupAnswers, criteria: AnswersType) -> usize {
    match criteria {
        AnswersType::Union => group
            .into_iter()
            .fold(HashSet::new(), |acc, x| acc.union(&x).cloned().collect())
            .len(),
        AnswersType::Intersection => {
            let first = group.pop().unwrap();
            group
                .into_iter()
                .fold(first, |acc, x| acc.intersection(&x).cloned().collect())
                .len()
        }
    }
}
fn part_one(v: &Vec<GroupAnswers>) -> usize {
    v.into_iter()
        .map(|g| count_common_answers(g.to_vec(), AnswersType::Union))
        .sum()
}
fn part_two(v: &Vec<GroupAnswers>) -> usize {
    v.into_iter()
        .map(|g| count_common_answers(g.to_vec(), AnswersType::Intersection))
        .sum()
}
fn main() {
    let groups = load_input("src/matt.txt").unwrap();

    println!("Part One: {} ", part_one(&groups));
    println!("Part Two: {} ", part_two(&groups));
}

fn vec_to_group(v: Vec<String>) -> GroupAnswers {
    v.into_iter()
        .map(|line| line.chars().collect::<Answers>())
        .collect()
}
fn load_input(fname: &str) -> io::Result<Vec<GroupAnswers>> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut groups = vec![];
    let mut group = vec![];
    for line in buf.lines() {
        let line = line?;
        if line.is_empty() {
            groups.push(vec_to_group(group.clone()));
            group.clear();
        } else {
            group.push(line)
        }
    }
    groups.push(vec_to_group(group.clone()));
    Ok(groups)
}

#[test]
fn test_parsing() {
    let groups = load_input("src/matt_test.txt").unwrap();
    assert_eq!(11, part_one(&groups))
}

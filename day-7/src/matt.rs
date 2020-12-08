// --- Day 7: Handy Haversacks ---
//
// https://adventofcode.com/2020/day/7
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

type Color = String;
type Rule = (usize, Color);
type Rules = Vec<Rule>;
type RuleSet = HashMap<Color, Rules>;

fn part_one(rs: &RuleSet, c: &str) -> usize {
    let mut count = 0;
    for color in rs.keys() {
        count += recursive_find(rs, c, rs.get(color).unwrap());
    }
    count
}

fn recursive_find(rs: &RuleSet, c: &str, rules: &[Rule]) -> usize {
    for rule in rules {
        if rule.1 == c || recursive_find(rs, c, rs.get(&rule.1).unwrap()) == 1 {
            return 1;
        }
    }
    0
}

fn part_two(rs: &RuleSet, c: &str) -> usize {
    let rules = rs.get(c).unwrap();
    if rules.is_empty() {
        0
    } else {
        let mut count = 0;
        for rule in rules {
            count += rule.0 + rule.0 * part_two(rs, &rule.1)
        }
        count
    }
}

fn main() {
    let ruleset = load_input("input.txt").unwrap();

    println!("Part One: {} ", part_one(&ruleset, "shiny gold"));
    println!("Part Two: {} ", part_two(&ruleset, "shiny gold"));
}

fn parse_rule_line(l: String) -> (Color, Rules) {
    let mut rules: Rules = vec![];
    let parts: Vec<_> = l.split("bags contain").collect();
    let color: Color = parts[0].trim().to_string();
    for part in parts[1].split(',') {
        if part.contains("no other bags") {
            continue;
        }
        let mut subs = part.trim().split(' ');
        let num: usize = subs.next().unwrap().trim().parse().unwrap();
        let sub_color: String = subs.take(2).fold(String::new(), |acc, x| {
            format!("{} {}", acc, x).trim().to_string()
        });
        rules.push((num, sub_color));
    }
    (color, rules)
}
fn load_input(fname: &str) -> io::Result<RuleSet> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut ruleset: RuleSet = HashMap::new();
    for line in buf.lines() {
        let (color, rules) = parse_rule_line(line?);
        ruleset.insert(color, rules);
    }
    Ok(ruleset)
}

#[test]
fn test_one() {
    let ruleset = load_input("test.txt").unwrap();
    assert_eq!(4, part_one(&ruleset, "shiny gold"));
}
#[test]
fn test_two() {
    let ruleset = load_input("test2.txt").unwrap();
    assert_eq!(126, part_two(&ruleset, "shiny gold"));
}

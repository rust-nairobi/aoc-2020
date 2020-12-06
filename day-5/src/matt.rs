// --- Day 5: Binary Boarding ---
//
// https://adventofcode.com/2020/day/5
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct BoardingPass {
    row: usize,
    col: usize,
    seat_id: usize,
}

impl BoardingPass {
    fn new(row: usize, col: usize) -> BoardingPass {
        BoardingPass {
            row,
            col,
            seat_id: row * 8 + col,
        }
    }
}

fn main() {
    let passes = input("src/matt.txt").unwrap();

    println!("Part One: {} ", part_one(&passes));
    println!("Part Two: {} ", part_two(&passes));
}

fn part_one(xs: &Vec<BoardingPass>) -> usize {
    xs.iter().map(|x| x.seat_id).max().unwrap()
}
fn part_two(xs: &Vec<BoardingPass>) -> usize {
    let mut ids: Vec<_> = xs.iter().map(|x| x.seat_id).collect();
    ids.sort();
    let min = *ids.iter().min().unwrap();
    let max = *ids.iter().max().unwrap();
    for idx in min..=max {
        if !ids.contains(&idx) {
            return idx;
        }
    }
    0
}
fn parse_boarding_pass(s: &str) -> BoardingPass {
    let (row, col) = s.split_at(s.len() - 3);
    let row_binstr = row
        .chars()
        .map(|x| match x {
            'F' => '0',
            'B' => '1',
            _ => unreachable!(),
        })
        .collect::<String>();
    let col_binstr = col
        .chars()
        .map(|x| match x {
            'R' => '1',
            'L' => '0',
            _ => unreachable!(),
        })
        .collect::<String>();
    let row = usize::from_str_radix(&row_binstr, 2).unwrap();
    let col = usize::from_str_radix(&col_binstr, 2).unwrap();
    BoardingPass::new(row, col)
}
fn input(fname: &str) -> io::Result<Vec<BoardingPass>> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .map(|line| parse_boarding_pass(&line.unwrap()))
        .collect())
}

#[test]
fn test_parsing() {
    let tests = vec![
        (
            "BFFFBBFRRR",
            BoardingPass {
                row: 70,
                col: 7,
                seat_id: 567,
            },
        ),
        (
            "FFFBBBFRRR",
            BoardingPass {
                row: 14,
                col: 7,
                seat_id: 119,
            },
        ),
        (
            "BBFFBBFRLL",
            BoardingPass {
                row: 102,
                col: 4,
                seat_id: 820,
            },
        ),
    ];
    for (input, pass) in tests.iter() {
        assert_eq!(pass, &parse_boarding_pass(input));
    }
}

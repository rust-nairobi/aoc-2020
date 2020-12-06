use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use itertools::Itertools;

struct BoardingPass {
    row_range: (u32, u32),
    col_range: (u32, u32),
}

#[derive(Debug)]
struct PassParseError;

impl Default for BoardingPass {
    fn default() -> Self {
        BoardingPass {
            row_range: (0, 127),
            col_range: (0, 7),
        }
    }
}

impl BoardingPass {
    fn new() -> Self {
        Default::default()
    }

    fn get_seat_id(row: u32, column: u32) -> u32 {
        (row * 8) + column
    }

    fn set_range(range: &mut (u32, u32), lower: bool) {
        let new_high = (range.1 + range.0 + 1) / 2;
        *range = if lower {
            (range.0, new_high - 1)
        } else {
            (new_high, range.1)
        };
    }

    fn partition(&mut self, character: &char) -> Result<(), PassParseError> {
        match character {
            'F' => BoardingPass::set_range(&mut self.row_range, true),
            'B' => BoardingPass::set_range(&mut self.row_range, false),
            'L' => BoardingPass::set_range(&mut self.col_range, true),
            'R' => BoardingPass::set_range(&mut self.col_range, false),
            _ => return Err(PassParseError),
        };
        Ok(())
    }

    fn seat_id(&self) -> u32 {
        BoardingPass::get_seat_id(self.row_range.0, self.col_range.0)
    }
}

impl FromStr for BoardingPass {
    type Err = PassParseError;

    fn from_str(s: &str) -> Result<BoardingPass, PassParseError> {
        let mut pass = BoardingPass::new();

        for character in s.chars() {
            pass.partition(&character)?;
        }

        if pass.row_range.0 != pass.row_range.1 || pass.col_range.0 != pass.col_range.1 {
            return Err(PassParseError);
        }
        Ok(pass)
    }
}

fn get_passes() -> Vec<BoardingPass> {
    let file = File::open("vickz84259.txt").expect("Unable to read file");
    let lines = io::BufReader::new(file).lines().filter_map(Result::ok);

    lines.map(|string| string.parse().unwrap()).collect()
}

fn part_1(seat_ids: &HashSet<u32>) {
    let highest_seat_id = seat_ids.iter().max().unwrap_or(&0u32);
    println!("Highest Seat Id: {}", highest_seat_id);
}

fn part_2(seat_ids: &HashSet<u32>) {
    let seat_id = (9u32..119u32)
        .cartesian_product(0u32..7u32)
        .map(|product| BoardingPass::get_seat_id(product.0, product.1))
        .filter(|seat_id| !seat_ids.contains(&seat_id))
        .exactly_one()
        .unwrap();

    println!("Seat id: {}", seat_id);
}

fn main() {
    let seat_ids: HashSet<u32> = get_passes().iter().map(|pass| pass.seat_id()).collect();

    println!("Part 1: \n----------");
    part_1(&seat_ids);

    println!("----------");
    println!("Part 2: \n----------");
    part_2(&seat_ids);
}

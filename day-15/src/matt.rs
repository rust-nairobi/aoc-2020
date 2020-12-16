// --- Day 15: Rambunctious Recitation ---
//
// https://adventofcode.com/2020/day/15
use std::collections::HashMap;

fn part_one(nums: Vec<usize>, nth: usize) -> usize {
    let mut rec = HashMap::<usize, Vec<usize>>::new();
    let mut current = 0usize;
    for i in 0..nth {
        current = if i < nums.len() {
            nums[i]
        } else if rec[&current].len() > 1 {
            let h = &rec[&current];
            h[h.len() - 1] - h[h.len() - 2]
        } else {
            0
        };
        rec.entry(current).or_default().push(i);
    }
    current
}

fn main() {
    println!("Part One: {} ", part_one(vec!(0, 5, 4, 1, 10, 14, 7), 2020));
    println!(
        "Part Two: {} ",
        part_one(vec!(0, 5, 4, 1, 10, 14, 7), 30000000)
    );
}

#[test]
fn test() {
    assert_eq!(436, part_one(vec!(0, 3, 6), 2020));
    assert_eq!(175594, part_one(vec!(0, 3, 6), 30000000));
}

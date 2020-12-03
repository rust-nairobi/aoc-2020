// --- Day 3: Toboggan Trajectory--
//
// https://adventofcode.com/2020/day/3
//use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
enum GridPoint {
    OpenSquare,
    Tree,
}
type GridLine = Vec<GridPoint>;
type GridMap = Vec<GridLine>;
struct Slope {
    right: usize,
    down: usize,
}
impl Slope {
    fn new(right: usize, down: usize) -> Slope {
        Slope { right, down }
    }
}

fn trees_encountered(map: &GridMap, slope: &Slope) -> usize {
    let mut tree_count = 0;
    let mut right_offset = 0;
    let mut down_offset = 0;
    for _ in 0..(map.len() - 1) {
        right_offset += slope.right;
        down_offset += slope.down;
        if down_offset > map.len() - 1 {
            break;
        }
        match map[down_offset][right_offset % map[0].len()] {
            GridPoint::Tree => tree_count += 1,
            _ => {}
        }
    }
    tree_count
}

fn trees_encountered_multiplied(map: &GridMap, slopes: Vec<Slope>) -> usize {
    slopes
        .iter()
        .map(|s| trees_encountered(map, s))
        .fold(1, |acc, x| acc * x)
}

fn main() {
    let map = input().unwrap();
    let slope = Slope::new(3, 1);

    println!("Part One: {}", trees_encountered(&map, &slope));
    println!(
        "Part Two: {}",
        trees_encountered_multiplied(
            &map,
            vec!(
                Slope::new(1, 1),
                Slope::new(3, 1),
                Slope::new(5, 1),
                Slope::new(7, 1),
                Slope::new(1, 2),
            )
        )
    );
}

fn input() -> io::Result<GridMap> {
    let mut input = vec![];
    let file = File::open("src/matt.txt")?;
    let buf = io::BufReader::new(file);
    for item in buf.lines() {
        let line = item?;
        input.push(parse_input_line(&line))
    }
    Ok(input)
}

fn parse_input_line(line: &str) -> GridLine {
    let mut grid_line = vec![];
    for chr in line.chars() {
        match chr {
            '#' => grid_line.push(GridPoint::Tree),
            '.' => grid_line.push(GridPoint::OpenSquare),
            _ => unreachable!(),
        }
    }
    grid_line
}

#[test]
fn test_matt() {
    let map_lines_str = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];
    let map: Vec<_> = map_lines_str
        .into_iter()
        .map(|l| parse_input_line(l))
        .collect();
    let slope = Slope::new(3, 1);
    assert_eq!(trees_encountered(&map, &slope), 7);
    assert_eq!(
        trees_encountered_multiplied(
            &map,
            vec!(
                Slope::new(1, 1),
                Slope::new(3, 1),
                Slope::new(5, 1),
                Slope::new(7, 1),
                Slope::new(1, 2),
            )
        ),
        336
    );
}

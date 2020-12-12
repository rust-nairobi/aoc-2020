// --- Day 11: Seating System ---
//
// https://adventofcode.com/2020/day/11
use std::fs::File;
use std::io::{self, BufRead};

type Row = Vec<Pos>;
type Layout = Vec<Row>;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Pos {
    Floor,
    Empty,
    Occupied,
}

#[derive(Debug, Clone)]
struct Grid {
    layout: Vec<Row>,
}
impl Grid {
    fn new(layout: Vec<Row>) -> Grid {
        Grid { layout }
    }

    fn get(&self, x: usize, y: usize) -> Pos {
        self.layout[x][y]
    }

    fn set(&mut self, x: usize, y: usize, pos: Pos) {
        self.layout[x][y] = pos
    }

    fn length(&self) -> usize {
        self.layout.len()
    }

    fn width(&self) -> usize {
        self.layout[0].len()
    }

    fn occupied(&self) -> usize {
        self.layout
            .iter()
            .map(|row| row.iter().filter(|p| **p == Pos::Occupied).count())
            .sum()
    }

    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.layout[x][y] == Pos::Empty
    }

    fn is_occupied(&self, x: usize, y: usize) -> bool {
        self.layout[x][y] == Pos::Occupied
    }

    #[allow(dead_code)]
    fn draw(&self) {
        for row in &self.layout {
            for pos in row {
                match pos {
                    Pos::Occupied => print!("#"),
                    Pos::Empty => print!("L"),
                    Pos::Floor => print!("."),
                }
            }
            println!()
        }
    }
}

fn part_two(grid: &mut Grid) -> usize {
    let dxns = vec![
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];
    let length = grid.length();
    let width = grid.width();
    loop {
        let mut changes = vec![];
        for x in 0..length {
            for y in 0..width {
                let mut num_occupied = 0;
                for d in &dxns {
                    for scale in 1.. {
                        let xy = (
                            x as isize + d.0 * scale as isize,
                            y as isize + d.1 * scale as isize,
                        );
                        if xy.0 < 0 || xy.1 < 0 || xy.0 >= length as isize || xy.1 >= width as isize
                        {
                            break;
                        }
                        let pos = grid.get(xy.0 as usize, xy.1 as usize);
                        if pos == Pos::Occupied || pos == Pos::Empty {
                            if pos == Pos::Occupied {
                                num_occupied += 1;
                            }
                            break;
                        }
                    }
                }
                if num_occupied == 0 && grid.is_empty(x, y) {
                    changes.push((x, y, Pos::Occupied));
                }
                if num_occupied >= 5 && grid.is_occupied(x, y) {
                    changes.push((x, y, Pos::Empty));
                }
            }
        }
        if changes.is_empty() {
            break;
        } else {
            for chg in &changes {
                grid.set(chg.0, chg.1, chg.2);
            }
            changes.clear();
        }
    }
    grid.occupied()
}

fn part_one(grid: &mut Grid) -> usize {
    let length = grid.length();
    let width = grid.width();
    loop {
        let mut changes = vec![];
        for x in 0..length {
            for y in 0..width {
                let is_empty: bool = gen_surround((x, y), length, width)
                    .into_iter()
                    .filter(|p| grid.get(p.0, p.1) == Pos::Occupied)
                    .count()
                    == 0;
                if is_empty && grid.is_empty(x, y) {
                    changes.push((x, y, Pos::Occupied));
                }
                let crowded: bool = gen_surround((x, y), length, width)
                    .into_iter()
                    .filter(|p| grid.get(p.0, p.1) == Pos::Occupied)
                    .count()
                    >= 4;
                if crowded && grid.is_occupied(x, y) {
                    changes.push((x, y, Pos::Empty));
                }
            }
        }
        if changes.is_empty() {
            break;
        } else {
            for chg in &changes {
                grid.set(chg.0, chg.1, chg.2);
            }
            changes.clear();
        }
    }
    grid.occupied()
}

fn gen_surround(pos: (usize, usize), max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let (x, y) = pos;
    let mut v = std::collections::HashSet::new();
    for i in &[x.saturating_sub(1), x, x + 1] {
        for j in &[y.saturating_sub(1), y, y + 1] {
            if (*i < max_x && *j < max_y) && ((*i, *j) != (x, y)) {
                v.insert((*i, *j));
            }
        }
    }

    v.into_iter().collect()
}

fn main() {
    let layout = load_input("input.txt").unwrap();
    let mut grid = Grid::new(layout);

    println!("Part One: {} ", part_one(&mut grid.clone()));
    println!("Part Two: {} ", part_two(&mut grid));
}

fn load_input(fname: &str) -> io::Result<Layout> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| match c {
                    '.' => Pos::Floor,
                    'L' => Pos::Empty,
                    '#' => Pos::Occupied,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect())
}

#[test]
fn test() {
    let layout = load_input("test.txt").unwrap();
    let grid = Grid::new(layout);
    assert_eq!(37, part_one(&mut grid.clone()));
    assert_eq!(26, part_two(&mut grid.clone()));
}

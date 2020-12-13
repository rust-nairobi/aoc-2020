// --- Day 12: Rain Risk ---
//
// https://adventofcode.com/2020/day/12
use std::fs::File;
use std::io::{self, BufRead};

struct Position {
    x: isize,
    y: isize,
}
struct Ship {
    dir: f64,
    pos: Position,
    waypoint: Position,
}
impl Ship {
    fn new() -> Ship {
        Ship {
            dir: 0f64,
            pos: Position { x: 0, y: 0 },
            waypoint: Position { x: 10, y: 1 },
        }
    }

    fn navigate(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::Right(v) => self.dir = ((self.dir - *v as f64) as isize % 360) as f64,
                Action::Left(v) => self.dir = ((self.dir + *v as f64) as isize % 360) as f64,
                Action::Forward(v) => {
                    self.pos.x += self.dir.to_radians().cos() as isize * *v;
                    self.pos.y += self.dir.to_radians().sin() as isize * *v;
                }
                Action::North(v) => self.pos.y += *v,
                Action::South(v) => self.pos.y -= *v,
                Action::East(v) => self.pos.x += *v,
                Action::West(v) => self.pos.x -= *v,
            }
        }
    }

    fn turn(&mut self, v: isize) {
        let r = ((self.waypoint.x.pow(2) + self.waypoint.y.pow(2)) as f64).sqrt();
        let t = (self.waypoint.y as f64).atan2(self.waypoint.x as f64);
        let t = t - (v as f64).to_radians();
        self.waypoint.x = (r * t.cos()).round() as isize;
        self.waypoint.y = (r * t.sin()).round() as isize;
    }

    fn navigate2(&mut self, actions: &[Action]) {
        for action in actions {
            match action {
                Action::Right(v) => self.turn(*v),
                Action::Left(v) => self.turn(*v - 2 * v),

                Action::Forward(v) => {
                    self.pos.x += self.waypoint.x * *v;
                    self.pos.y += self.waypoint.y * *v;
                }

                Action::North(v) => self.waypoint.y += *v,
                Action::South(v) => self.waypoint.y -= *v,
                Action::East(v) => self.waypoint.x += *v,
                Action::West(v) => self.waypoint.x -= *v,
            }
        }
    }

    fn mdist(&self) -> isize {
        //|x1 - x2| + |y1 - y2|
        self.pos.x.abs() + self.pos.y.abs()
    }
}

enum Action {
    East(isize),
    North(isize),
    South(isize),
    West(isize),
    Forward(isize),
    Left(isize),
    Right(isize),
}

fn part_one(actions: &[Action]) -> isize {
    let mut ship = Ship::new();
    ship.navigate(actions);
    ship.mdist()
}
fn part_two(actions: &[Action]) -> isize {
    let mut ship = Ship::new();
    ship.navigate2(actions);
    ship.mdist()
}

fn main() {
    let actions = load_input("input.txt").unwrap();

    println!("Part One: {} ", part_one(&actions));
    println!("Part Two: {} ", part_two(&actions));
}

fn load_input(fname: &str) -> io::Result<Vec<Action>> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    Ok(buf
        .lines()
        .filter_map(|x| x.ok())
        .map(|x| {
            let val = x[1..].parse::<isize>().unwrap();
            match &x[0..1] {
                "E" => Action::East(val),
                "F" => Action::Forward(val),
                "L" => Action::Left(val),
                "N" => Action::North(val),
                "R" => Action::Right(val),
                "S" => Action::South(val),
                "W" => Action::West(val),
                _ => unreachable!(),
            }
        })
        .collect())
}

#[test]
fn test() {
    let actions = load_input("test.txt").unwrap();
    let mut ship = Ship::new();
    ship.navigate(&actions);
    assert_eq!(25, ship.mdist());
}

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

type MapLines = Vec<String>;

fn get_lines() -> MapLines {
    let file = File::open("input_3.txt").expect("Unable to read file");
    let lines = io::BufReader::new(file).lines();

    lines.filter_map(Result::ok).collect()
}

trait Map {
    fn new(lines: &MapLines) -> Self;
    fn traverse(&self, forward: usize, down: usize) -> usize;
}

struct DefaultMap {
    _map: Vec<Vec<char>>,
}

impl Map for DefaultMap {
    fn new(lines: &MapLines) -> Self {
        let vecs = lines.iter().map(|line| line.chars().collect::<Vec<char>>());

        DefaultMap {
            _map: vecs.collect(),
        }
    }

    fn traverse(&self, forward: usize, down: usize) -> usize {
        let vec_tuple = self._map.iter().step_by(down).enumerate();

        vec_tuple
            .filter(|x| {
                let index = (forward * x.0) % x.1.len();
                match x.1[index] {
                    '#' => true,
                    _ => false,
                }
            })
            .count()
    }
}

struct BoolMap {
    _map: Vec<Vec<bool>>,
}

impl Map for BoolMap {
    fn new(lines: &MapLines) -> Self {
        let _map = lines
            .iter()
            .map(|line| {
                line.chars()
                    .map(|character| match character {
                        '#' => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();

        BoolMap { _map }
    }

    fn traverse(&self, forward: usize, down: usize) -> usize {
        let vec_tuple = self._map.iter().step_by(down).enumerate();

        vec_tuple
            .filter(|x| {
                let index = (forward * x.0) % x.1.len();
                x.1[index]
            })
            .count()
    }
}

struct BitMap {
    width: u32,
    _map: Vec<u32>,
}

impl Map for BitMap {
    fn new(lines: &MapLines) -> Self {
        let mut width: u32 = 0;

        let _map = lines
            .iter()
            .map(|line| {
                let mut row: u32 = 0;
                width = line.len() as u32;

                line.chars().enumerate().for_each(|x| {
                    match x.1 {
                        '#' => row = row | (1u32 << (width - (x.0 as u32))),
                        _ => (),
                    };
                });
                row
            })
            .collect();

        BitMap { width, _map }
    }

    fn traverse(&self, forward: usize, down: usize) -> usize {
        let vec_tuple = self._map.iter().step_by(down).enumerate();

        vec_tuple
            .filter(|x| {
                let index: u32 = ((forward * x.0) as u32) % self.width;
                let bit = x.1 & (1u32 << (self.width - index));

                match bit {
                    0 => false,
                    _ => true,
                }
            })
            .count()
    }
}

fn part_1<T: Map>(map: &T) {
    let trees_count = map.traverse(3, 1);

    println!("\tTrees found: {}", trees_count);
}

fn part_2<T: Map>(map: &T) {
    let paths = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let answer: usize = paths.iter().map(|x| map.traverse(x.0, x.1)).product();
    println!("Answer: {}", answer);
}

fn main() {
    let lines = get_lines();

    println!("Part 1: \n ----------");

    println!("Default Map");
    let mut start = Instant::now();

    let default_map = DefaultMap::new(&lines);
    part_1(&default_map);
    println!("\tTime Taken: {:?}", start.elapsed());

    println!("Bool Map");
    start = Instant::now();

    let bool_map = BoolMap::new(&lines);
    part_1(&bool_map);
    println!("\tTime Taken: {:?}", start.elapsed());

    println!("Bit Map");

    start = Instant::now();

    let bit_map = BitMap::new(&lines);
    part_1(&bit_map);
    println!("\tTime Taken: {:?}", start.elapsed());

    println!("---------- \nPart 2: \n----------");

    println!("Default Map");
    start = Instant::now();
    part_2(&default_map);
    println!("\tTime Taken: {:?}", start.elapsed());

    println!("Bool Map");
    start = Instant::now();
    part_2(&bool_map);
    println!("\tTime Taken: {:?}", start.elapsed());

    println!("Bit Map");
    start = Instant::now();
    part_2(&bit_map);
    println!("\tTime Taken: {:?}", start.elapsed());
}

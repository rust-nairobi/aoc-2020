// --- Day 13: Shuttle Search ---
//
// https://adventofcode.com/2020/day/13
use std::fs::File;
use std::io::{self, BufRead};

fn part_one(time: usize, buses: Vec<usize>) -> usize {
    let mut t = time;
    let mut id: usize = 0;
    'outer: loop {
        for b in &buses {
            if t % b == 0 {
                id += *b;
                break 'outer;
            }
        }
        t += 1;
    }
    id * (t - time)
}
fn step(s: usize, n: usize, n2: usize, k2: usize) -> usize {
    let a = n2 as isize - k2 as isize;
    let b = n2 as isize;
    let f = (((a % b) + b) % b) as usize;
    for r in (s..).step_by(n) {
        if r % n2 == f {
            return r;
        }
    }
    0
}
fn part_two(mut v: Vec<(usize, usize)>) -> usize {
    v = v.into_iter().map(|x| (x.1, x.0)).collect();
    v.sort_by_key(|x| x.0);
    let mut n = 1;
    let mut r = v[v.len() - 1].0 - v[v.len() - 1].1;
    for i in 1..v.len() {
        n *= v[v.len() - i].0;
        let (n2, k2) = v[v.len() - i - 1];
        r = step(r, n, n2, k2);
    }
    r
}

fn main() {
    let (time, buses) = load_input("input.txt").unwrap();
    let buses2 = load_input2("input.txt").unwrap();

    println!("Part One: {} ", part_one(time, buses));
    println!("Part Two: {} ", part_two(buses2));
}

fn load_input(fname: &str) -> io::Result<(usize, Vec<usize>)> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut v = buf.lines().filter_map(|x| x.ok());
    let time: usize = v.next().unwrap().parse().unwrap();
    let buses: Vec<usize> = v
        .next()
        .unwrap()
        .split(',')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect();

    Ok((time, buses))
}

fn load_input2(fname: &str) -> io::Result<Vec<(usize, usize)>> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut v = vec![];
    for line in buf.lines().skip(1) {
        let l = line?;
        for x in l
            .split(',')
            .enumerate()
            .filter_map(|l| match l.1.parse::<usize>() {
                Ok(v) => Some((l.0, v)),
                Err(_) => None,
            })
        {
            v.push(x);
        }
    }
    Ok(v)
}

#[test]
fn test() {
    let (time, buses) = load_input("test.txt").unwrap();
    assert_eq!(295, part_one(time, buses));
    //7,13,x,x,59,x,31,19
    assert_eq!(
        1068781,
        part_two(vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)])
    );
    assert_eq!(3417, part_two(vec![(0, 17), (2, 13), (3, 19)]));
}

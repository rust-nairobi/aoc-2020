// --- Day 8: Handheld Halting ---
//
// https://adventofcode.com/2020/day/8
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Instruction {
    Acc(Sign, isize),
    Jmp(Sign, isize),
    Nop(Sign, isize),
}

type Program = Vec<Instruction>;

fn part_one(program: &Program) -> (isize, bool) {
    let mut acc = 0;
    let mut will_loop = false;
    let mut pos = 0;
    let mut executed = vec![];
    while !will_loop && pos < program.len() {
        let ins = &program[pos];
        if executed.contains(&pos) {
            will_loop = true;
            continue;
        }
        executed.push(pos);
        match ins {
            Instruction::Acc(Sign::Positive, x) => {
                acc += x;
                pos += 1;
            }
            Instruction::Acc(Sign::Negative, x) => {
                acc -= x;
                pos += 1;
            }
            Instruction::Jmp(Sign::Positive, x) => pos += *x as usize,
            Instruction::Jmp(Sign::Negative, x) => pos -= *x as usize,
            _ => pos += 1,
        }
    }
    (acc, will_loop)
}
fn part_two(program: &Program) -> isize {
    let mut nth = 0;
    loop {
        let mut prog = program.to_owned();
        flip_nth_instruction(&mut prog, nth);
        let (acc, looped) = part_one(&prog);
        if !looped {
            return acc;
        } else {
            nth += 1;
        }
    }
}

fn flip_nth_instruction(program: &mut Program, nth: usize) {
    let mut nth = nth;
    for idx in 0..program.len() - 1 {
        let ins = program[idx];
        match ins {
            Instruction::Acc(_, _) => {}
            _ => {
                if nth == 0 {
                    match ins {
                        Instruction::Jmp(s, i) => program[idx] = Instruction::Nop(s, i),
                        Instruction::Nop(s, i) => program[idx] = Instruction::Jmp(s, i),
                        _ => {}
                    }
                    return;
                } else {
                    nth -= 1;
                }
            }
        }
    }
}

fn main() {
    let program = load_program("input.txt").unwrap();

    println!("Part One: {} ", part_one(&program).0);
    println!("Part Two: {} ", part_two(&program));
}

fn parse_line(l: String) -> Instruction {
    let parts: Vec<_> = l.split(' ').collect();
    let arg = parts[1];
    let sign = match &arg[0..1] {
        "+" => Sign::Positive,
        "-" => Sign::Negative,
        _ => unreachable!(),
    };
    let offset: isize = arg[1..].parse().unwrap();
    match parts[0] {
        "acc" => Instruction::Acc(sign, offset),
        "jmp" => Instruction::Jmp(sign, offset),
        "nop" => Instruction::Nop(sign, offset),
        _ => unreachable!(),
    }
}
fn load_program(fname: &str) -> io::Result<Program> {
    let file = File::open(fname)?;
    let buf = io::BufReader::new(file);
    let mut program: Program = Vec::new();
    for line in buf.lines() {
        program.push(parse_line(line?))
    }
    Ok(program)
}

#[test]
fn test() {
    let program = load_program("test.txt").unwrap();
    assert_eq!(5, part_one(&program).0);
    assert_eq!(8, part_two(&program));
}

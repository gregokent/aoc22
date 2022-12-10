use std::str::FromStr;

use crate::Result;

#[derive(Debug, Copy, Clone)]
enum Opcode {
    Noop,
    AddX(i32),
}

impl Opcode {
    fn cycles(&self) -> u32 {
        match *self {
            Opcode::Noop => 1,
            Opcode::AddX(_) => 2,
        }
    }
}

impl FromStr for Opcode {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let opcode = match &tokens[..] {
            ["noop"] => Opcode::Noop,
            ["addx", val] => Opcode::AddX(val.parse()?),
            [] | [..] => {
                return Err(format!("{s} is invalid instruction").into());
            }
        };

        Ok(opcode)
    }
}
pub fn run() -> Result<()> {
    println!("* Day 10 *");

    let input = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/input/day10/input.txt"
    ));
    // let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day10/test"));

    let history = puzzle1(input)?;
    puzzle2(history)?;
    Ok(())
}

fn puzzle1(input: &str) -> Result<Vec<i32>> {
    let mut x = 1;
    let mut history = Vec::new();

    for line in input.lines() {
        let opcode = Opcode::from_str(line)?;
        let count = opcode.cycles();
        for _ in 0..count {
            history.push(x);
        }
        x += match opcode {
            Opcode::Noop => 0,
            Opcode::AddX(a) => a,
        };
    }

    let cycles_of_interest = [20usize, 60, 100, 140, 180, 220];
    let mut signal = 0;
    for coi in cycles_of_interest {
        signal += coi as i32 * history[coi - 1];
    }

    println!("{signal}");

    Ok(history)
}

fn puzzle2(history: Vec<i32>) -> Result<()> {
    let mut pixels = vec![false; history.len()];

    for (cycle, reg_x) in history.iter().enumerate() {
        let crt_pos = cycle % 40;

        pixels[cycle] = (reg_x - 1..=reg_x + 1).contains(&(crt_pos as i32));
    }

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", if pixels[40 * y + x] { "#" } else { " " });
        }
        println!();
    }

    Ok(())
}

use std::env;
use std::io::BufRead;

use crate::Result;

#[derive(Debug, Clone, Copy)]
enum Kind {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug, Clone, Copy)]
enum Throw {
    Opponent(Kind),
    Me(Kind),
}

impl TryFrom<&str> for Throw {
    type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        let throw = match value {
            "A" => Throw::Opponent(Kind::Rock),
            "B" => Throw::Opponent(Kind::Paper),
            "C" => Throw::Opponent(Kind::Scissors),
            "X" => Throw::Me(Kind::Rock),
            "Y" => Throw::Me(Kind::Paper),
            "Z" => Throw::Me(Kind::Scissors),
            _ => {
                return Err(format!("{value} is an invalid value for Throw").into());
            }
        };
        Ok(throw)
    }
}

#[derive(Debug, Clone, Copy)]
enum Round {
    Win(Kind),
    Lose(Kind),
    Draw(Kind),
}

impl From<Round> for u32 {
    fn from(round: Round) -> Self {
        use Round::*;
        match round {
            Win(x) => 6 + x as u32,
            Lose(x) => 0 + x as u32,
            Draw(x) => 3 + x as u32,
        }
    }
}

pub fn run() -> Result<()> {
    println!("* Day 2 *");

    puzzle1()?;
    puzzle2()?;
    Ok(())
}

fn puzzle1() -> Result<()> {
    let input = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2/input"))?;

    let reader = std::io::BufReader::new(input);
    let mut score = 0;
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let round = match (&line[0..1], &line[2..3]) {
            ("A", "X") => Round::Draw(Kind::Rock),
            ("A", "Y") => Round::Win(Kind::Paper),
            ("A", "Z") => Round::Lose(Kind::Scissors),
            ("B", "X") => Round::Lose(Kind::Rock),
            ("B", "Y") => Round::Draw(Kind::Paper),
            ("B", "Z") => Round::Win(Kind::Scissors),
            ("C", "X") => Round::Win(Kind::Rock),
            ("C", "Y") => Round::Lose(Kind::Paper),
            ("C", "Z") => Round::Draw(Kind::Scissors),
            _ => {
                panic!("invalid input format")
            }
        };
        score += u32::from(round);
    }

    println!("{score}");
    Ok(())
}

fn puzzle2() -> Result<()> {
    const ROCK: &'static str = "A";
    const PAPER: &'static str = "B";
    const SCISSORS: &'static str = "C";
    let input = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day2/input"))?;

    let reader = std::io::BufReader::new(input);
    let mut score = 0;
    for (idx, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let round = match (&line[0..1], &line[2..3]) {
            (ROCK, "X") => Round::Lose(Kind::Scissors),
            (ROCK, "Y") => Round::Draw(Kind::Rock),
            (ROCK, "Z") => Round::Win(Kind::Paper),
            (PAPER, "X") => Round::Lose(Kind::Rock),
            (PAPER, "Y") => Round::Draw(Kind::Paper),
            (PAPER, "Z") => Round::Win(Kind::Scissors),
            (SCISSORS, "X") => Round::Lose(Kind::Paper),
            (SCISSORS, "Y") => Round::Draw(Kind::Scissors),
            (SCISSORS, "Z") => Round::Win(Kind::Rock),
            _ => {
                panic!("invalid input format")
            }
        };
        score += u32::from(round);
    }

    println!("{score}");
    Ok(())
}

use std::env;
use std::io::BufRead;

use crate::Result;

pub fn run() -> Result<()> {
    let input = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1/input"))?;

    let mut elves = Vec::new();

    let mut calories = Vec::new();
    let reader = std::io::BufReader::new(input);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            elves.push(calories.clone());
            calories.clear();
            continue;
        }

        let val = line.parse::<u32>()?;
        calories.push(val);
    }

    let elves_totals = elves
        .iter()
        .map(|cals| cals.iter().sum())
        .collect::<Vec<u32>>();

    puzzle1(elves_totals.clone())?;
    puzzle2(elves_totals.clone())?;
    Ok(())
}

fn puzzle1(elves: Vec<u32>) -> Result<()> {
    let max: u32 = *elves.iter().max().unwrap();
    println!("puzzle 1: {}", max);
    Ok(())
}

fn puzzle2(mut elves: Vec<u32>) -> Result<()> {
    elves.sort();

    let top_3: u32 = elves.iter().rev().take(3).sum();

    println!("puzzle 2: {}", top_3);
    Ok(())
}

use std::env;
use std::io::BufRead;

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 1 *");

    let input = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1/input"))?;
    let reader = std::io::BufReader::new(input);

    let mut elves = Vec::new();
    let mut calories = Vec::new();

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

    // elves is a vector of vectors of integers
    // each element in elves represents one elf and the value
    // is a vector of the calorie count for each item they're carrying
    // we only care about the total calories carried per elf, so
    // we collapse each internal vector into the sum of all it's values
    // so now each element of elves_totals is the total calorie count per elf
    let elves_totals = elves
        .iter()
        .map(|cals| cals.iter().sum())
        .collect::<Vec<u32>>();

    puzzle1(elves_totals.clone())?;
    puzzle2(elves_totals.clone())?;
    Ok(())
}

fn puzzle1(elves: Vec<u32>) -> Result<()> {
    // we want the max calories carried by 1 elf
    let max: u32 = *elves.iter().max().unwrap();
    println!("puzzle 1: {}", max);
    Ok(())
}

fn puzzle2(mut elves: Vec<u32>) -> Result<()> {
    // we want the top 3 elves calorie counts
    elves.sort();
    // after sorting our elves by calories, we reverse the order and take the first 3 elements
    // which equal the top 3 elves
    let top_3: u32 = elves.iter().rev().take(3).sum();

    println!("puzzle 2: {}", top_3);
    Ok(())
}

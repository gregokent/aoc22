use std::collections::HashSet;

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 6 *");

    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day6/input"));

    puzzle1(input)?;
    puzzle2(input)?;
    Ok(())
}

fn find_first_index_after_unique(input: &str, unique_vals: usize) -> Result<usize> {
    let mut set: HashSet<u8> = HashSet::new();
    for (idx, window) in input.as_bytes().as_ref().windows(unique_vals).enumerate() {
        set.clear();
        set.extend(window);
        if set.len() == unique_vals {
            return Ok(idx + unique_vals);
        }
    }

    Err(format!("couldn't find string of {unique_vals} unique values").into())
}

fn puzzle1(input: &str) -> Result<()> {
    println!("{}", find_first_index_after_unique(input, 4)?);
    Ok(())
}

fn puzzle2(input: &str) -> Result<()> {
    println!("{}", find_first_index_after_unique(input, 14)?);
    Ok(())
}

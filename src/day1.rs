use std::env;

use crate::Result;

pub fn run() -> Result<()> {
    let input = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day1/input"))?;

    println!("{input:?}");
    Ok(())
}

fn puzzle1() -> Result<()> {
    Ok(())
}

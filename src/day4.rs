use std::env;
use std::io::{BufRead, Read};
use std::iter::FromIterator;
use std::str::FromStr;

use crate::Result;

#[derive(Debug)]
struct Section {
    start: u32,
    end: u32,
}

impl FromStr for Section {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let dash = s.find(|c| c == '-').ok_or("failed to find separator")?;
        let start = s[..dash].parse::<u32>()?;
        let end = s[dash + 1..].parse::<u32>()?;
        Ok(Section { start, end })
    }
}

fn parse_line(line: &str) -> Result<(Section, Section)> {
    let comma = line.find(|c| c == ',').ok_or("failed to find separator")?;
    Ok((
        Section::from_str(&line[..comma])?,
        Section::from_str(&line[comma + 1..])?,
    ))
}

pub fn run() -> Result<()> {
    println!("* Day 4 *");

    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day4/input"));

    puzzle1(input)?;
    puzzle2(input)?;
    Ok(())
}

fn puzzle1(pairings: &str) -> Result<()> {
    let mut total = 0;
    for line in pairings.lines() {
        let (one, two) = parse_line(line)?;

        match one.start.cmp(&two.start) {
            std::cmp::Ordering::Less => {
                if two.end <= one.end {
                    total += 1;
                }
            }
            std::cmp::Ordering::Equal => {
                if one.end < two.end || two.end < one.end || one.end == two.end {
                    total += 1;
                }
            }
            std::cmp::Ordering::Greater => {
                if one.end <= two.end {
                    total += 1;
                }
            }
        }
    }

    println!("{total}");

    Ok(())
}

fn puzzle2(pairings: &str) -> Result<()> {
    let mut total = 0;
    for line in pairings.lines() {
        let (one, two) = parse_line(line)?;

        match one.start.cmp(&two.start) {
            std::cmp::Ordering::Less => {
                if one.end >= two.start {
                    total += 1;
                }
            }
            std::cmp::Ordering::Equal => {
                total += 1;
            }
            std::cmp::Ordering::Greater => {
                if one.start <= two.end {
                    total += 1;
                }
            }
        }
    }
    println!("{total}");
    Ok(())
}

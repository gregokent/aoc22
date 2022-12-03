use std::collections::HashSet;
use std::env;
use std::io::{BufRead, Read};
use std::iter::FromIterator;

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 3 *");

    let mut input = std::fs::File::open(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day3/input"))?;

    let mut bags = String::new();
    input.read_to_string(&mut bags)?;

    puzzle1(&bags);
    puzzle2(&bags);
    Ok(())
}

fn puzzle1(bags: &str) {
    let mut common_items = Vec::new();

    for line in bags.lines() {
        let line = line;
        let (compartment_a, compartment_b) = &line.as_bytes().split_at(&line.len() / 2);

        for item_a in *compartment_a {
            if let Some(item) = compartment_b.iter().find(|&item_b| item_a == item_b) {
                common_items.push(*item);
                break;
            }
        }
    }
    //add up priorities of items
    let total = common_items.iter().fold(0u32, |acc, x| {
        let val = match x {
            val @ b'a'..=b'z' => val - b'a' + 1,
            val @ b'A'..=b'Z' => val - b'A' + 27,
            _ => 0,
        };
        acc + val as u32
    });

    println!("{total}");
}

fn puzzle2(bags: &str) -> Result<()> {
    let start = std::time::Instant::now();
    let mut items = Vec::new();
    // iterate over 3 lines at a time

    for (line1, line2, line3) in bags
        .lines()
        .step_by(3)
        .zip(bags.lines().skip(1).step_by(3))
        .zip(bags.lines().skip(2).step_by(3))
        .map(|((a, b), c)| (a, b, c))
    {
        // create a hashset for each line, which will give us just unique letters per line
        let set_a: HashSet<u8> = HashSet::from_iter(line1.bytes());
        let set_b: HashSet<u8> = HashSet::from_iter(line2.bytes());
        let set_c: HashSet<u8> = HashSet::from_iter(line3.bytes());

        // calculate the intersection of all 3 lines, which should yield a single element
        let intersection = set_a
            .intersection(&set_b)
            .map(|a| *a)
            .collect::<HashSet<u8>>()
            .intersection(&set_c)
            .map(|a| *a)
            .collect::<Vec<u8>>();

        items.push(intersection[0]);
    }

    let total = items.iter().fold(0u32, |acc, x| {
        let val = match x {
            val @ b'a'..=b'z' => val - b'a' + 1,
            val @ b'A'..=b'Z' => val - b'A' + 27,
            _ => 0,
        };
        acc + val as u32
    });

    let end = start.elapsed();
    println!("{total} - {}", end.as_micros());

    Ok(())
}

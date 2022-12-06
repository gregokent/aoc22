use std::collections::{BTreeMap, HashMap};

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 5 *");

    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5/input"));

    puzzle1(input)?;
    puzzle2(input)?;
    Ok(())
}

fn puzzle1(input: &str) -> Result<()> {
    let (mut stacks, lines) = parse_diagram(input)?;
    let instructions = input.lines().skip(lines + 1).map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .map(|v| v.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    });

    for instruction in instructions {
        let mut tmp = Vec::new();
        stacks
            .entry(instruction[1] as usize)
            .and_modify(|v| tmp.extend(v.drain(v.len() - instruction[0] as usize..).rev()));

        stacks
            .entry(instruction[2] as usize)
            .and_modify(|v| v.extend_from_slice(&tmp));
    }

    for (_k, v) in stacks.iter() {
        print!("{}", *v.last().unwrap() as char);
    }
    println!("");
    Ok(())
}

fn puzzle2(input: &str) -> Result<()> {
    let (mut stacks, lines) = parse_diagram(input)?;
    let instructions = input.lines().skip(lines + 1).map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .step_by(2)
            .map(|v| v.parse::<u8>().unwrap())
            .collect::<Vec<u8>>()
    });

    for instruction in instructions {
        let mut tmp = Vec::new();
        stacks
            .entry(instruction[1] as usize)
            .and_modify(|v| tmp.extend(v.drain(v.len() - instruction[0] as usize..)));

        stacks
            .entry(instruction[2] as usize)
            .and_modify(|v| v.extend_from_slice(&tmp));
    }

    for (_k, v) in stacks.iter() {
        print!("{}", *v.last().unwrap() as char);
    }
    println!("");
    Ok(())
}

fn parse_diagram(input: &str) -> Result<(BTreeMap<usize, Vec<u8>>, usize)> {
    let lines: Vec<String> = input
        .lines()
        .take_while(|line| !line.is_empty())
        .map(String::from)
        .collect();

    let mut cargo: BTreeMap<usize, Vec<u8>> = BTreeMap::new();
    for line in lines.iter().rev() {
        for (idx, &ch) in line.as_bytes().iter().skip(1).step_by(4).enumerate() {
            cargo
                .entry(idx + 1)
                .and_modify(|v| {
                    if ch != b' ' {
                        v.push(ch)
                    }
                })
                .or_insert_with(|| if ch != b' ' { Vec::new() } else { vec![ch] });
        }
    }
    Ok((cargo, lines.len()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day5/input"));
    #[test]
    fn test_name() {
        let (_diagram, lines_read) = parse_diagram(TEST_INPUT).unwrap();
        assert_eq!(9, lines_read);
    }
}

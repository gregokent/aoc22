use std::rc::Rc;

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 8 *");

    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day8/input.txt"));
    // let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day8/test"));

    puzzle1(input)?;
    puzzle2(input)?;

    Ok(())
}

fn puzzle1(input: &str) -> Result<()> {
    let forest = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<Vec<u8>>>();

    assert!(!forest.is_empty());
    assert!(forest.iter().all(|v| v.len() == forest.len()));

    let mut visibility: Vec<Vec<u8>> = vec![vec![0; forest.len()]; forest.len()];

    let max = forest.len();
    for y in 0..max {
        let mut highest = 0;
        for x in 0..max {
            let current = forest[y][x];
            if current > highest {
                visibility[y][x] += 1;
                highest = current;
            }
        }
    }

    for y in 0..max {
        let mut highest = 0;
        for x in (0..max).rev() {
            let current = forest[y][x];
            if current > highest {
                visibility[y][x] += 1;
                highest = current;
            }
        }
    }

    for x in 0..max {
        let mut highest = 0;
        for y in 0..max {
            let current = forest[y][x];
            if current > highest {
                visibility[y][x] += 1;
                highest = current;
            }
        }
    }

    for x in 0..max {
        let mut highest = 0;
        for y in (0..max).rev() {
            let current = forest[y][x];
            if current > highest {
                visibility[y][x] += 1;
                highest = current;
            }
        }
    }

    // pretty_print(&visibility);

    let visible: usize = visibility
        .iter()
        .map(|row| row.iter().filter(|&&vis| vis > 0).count())
        .sum();

    println!("total visible: {visible}");

    Ok(())
}

fn puzzle2(input: &str) -> Result<()> {
    let forest = input
        .lines()
        .map(|line| line.bytes().collect())
        .collect::<Vec<Vec<u8>>>();

    assert!(!forest.is_empty());
    assert!(forest.iter().all(|v| v.len() == forest.len()));

    let forest = Rc::new(forest);
    let scenic = forest
        .iter()
        .enumerate()
        .map(|(y, row)| {
            let forest = Rc::clone(&forest);
            row.iter()
                .enumerate()
                .map(move |(x, _)| {
                    look_left(&forest, x, y)
                        * look_right(&forest, x, y)
                        * look_up(&forest, x, y)
                        * look_down(&forest, x, y)
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    println!("{scenic}");

    Ok(())
}

fn look_left(forest: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let my_height = forest[y][x];

    let mut count = 0;
    for x in (0..x).rev() {
        let next_tree = forest[y][x];
        count += 1;
        if next_tree >= my_height {
            break;
        }
    }
    count
}

fn look_right(forest: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let my_height = forest[y][x];
    let max = forest[0].len();

    let mut count = 0;
    for x in x + 1..max {
        let next_tree = forest[y][x];
        count += 1;
        if next_tree >= my_height {
            break;
        }
    }
    count
}

fn look_down(forest: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let my_height = forest[y][x];
    let max = forest.len();

    let mut count = 0;
    for y in y + 1..max {
        let next_tree = forest[y][x];
        count += 1;
        if next_tree >= my_height {
            break;
        }
    }
    count
}

fn look_up(forest: &[Vec<u8>], x: usize, y: usize) -> u32 {
    let my_height = forest[y][x];

    let mut count = 0;
    for y in (0..y).rev() {
        count += 1;
        let next_tree = forest[y][x];
        if next_tree >= my_height {
            break;
        }
    }
    count
}

fn pretty_print(forest: &[Vec<u8>]) {
    let max = forest.len();
    for y in 0..max {
        for x in 0..max {
            print!("{}", forest[y][x]);
        }
        println!();
    }
}

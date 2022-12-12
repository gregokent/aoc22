use std::{collections::VecDeque, iter::FromIterator};

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 11 *");

    #[rustfmt::skip]
    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"),"/input/day11/input.txt"));
    // let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day10/test"));

    #[rustfmt::skip]
    let mut monkeys: Vec<Monkey> =  vec![
        Monkey::new(&[93, 54, 69, 66, 71], 7, |old| old * 3, |divisible| { if divisible { 7 } else { 1 }}),
        Monkey::new(&[89, 51, 80, 66], 19, |old| old * 17, |divisible| { if divisible { 5 } else { 7 }}),
        Monkey::new(&[90,92,63,91,96,63,64], 13, |old| old + 1, |divisible| { if divisible { 4 } else { 3 }}),
        Monkey::new(&[65, 77], 3, |old| old + 2, |divisible| { if divisible { 4 } else { 6 }}),
        Monkey::new(&[76,68,94], 2, |old| old * old, |divisible| { if divisible { 0 } else { 6 }}),
        Monkey::new(&[86,65,66,97,73,83], 11, |old| old + 8, |divisible| { if divisible { 2 } else { 3 }}),
        Monkey::new(&[78], 17, |old| old + 6, |divisible| { if divisible { 0 } else { 1 }}),
        Monkey::new(&[89, 57, 59, 61, 87, 55, 55, 88], 5, |old| old + 7, |divisible| { if divisible { 2 } else { 5 }}),
    ];
    let mut inspections = vec![0; monkeys.len()];

    for round in 0..20 {
        // for round in 0..1 {
        for monkey in 0..monkeys.len() {
            // for item in monkeys[monkey].items.into_iter() {
            while let Some(item) = monkeys[monkey].items.pop_front() {
                inspections[monkey] += 1;
                // for item_idx in 0..monkeys[monkey].items.len() {
                let new_val = (monkeys[monkey].worry_amplifier)(item);
                let new_val = new_val / 3;
                let to_whom = monkeys[monkey].throw(new_val.clone());
                monkeys[to_whom].items.push_back(new_val);
            }
        }
    }

    inspections.sort();
    let monkey_business = inspections.iter().rev().take(2).fold(1usize, |x, a| x * a);

    println!("{monkey_business}");

    // Part 2
    #[rustfmt::skip]
    let mut monkeys: Vec<Monkey> =  vec![
        Monkey::new(&[93, 54, 69, 66, 71], 7, |old| old * 3, |divisible| { if divisible { 7 } else { 1 }}),
        Monkey::new(&[89, 51, 80, 66], 19, |old| old * 17, |divisible| { if divisible { 5 } else { 7 }}),
        Monkey::new(&[90,92,63,91,96,63,64], 13, |old| old + 1, |divisible| { if divisible { 4 } else { 3 }}),
        Monkey::new(&[65, 77], 3, |old| old + 2, |divisible| { if divisible { 4 } else { 6 }}),
        Monkey::new(&[76,68,94], 2, |old| old * old, |divisible| { if divisible { 0 } else { 6 }}),
        Monkey::new(&[86,65,66,97,73,83], 11, |old| old + 8, |divisible| { if divisible { 2 } else { 3 }}),
        Monkey::new(&[78], 17, |old| old + 6, |divisible| { if divisible { 0 } else { 1 }}),
        Monkey::new(&[89, 57, 59, 61, 87, 55, 55, 88], 5, |old| old + 7, |divisible| { if divisible { 2 } else { 5 }}),
    ];
    inspections = vec![0; monkeys.len()];
    // multiply all test values together
    let num = monkeys.iter().fold(1, |x, a| x * a.worry_level_tester);
    for round in 0..10_000 {
        for monkey in 0..monkeys.len() {
            while let Some(item) = monkeys[monkey].items.pop_front() {
                inspections[monkey] += 1;
                let new_val = (monkeys[monkey].worry_amplifier)(item);
                let new_val = new_val % num;
                let to_whom = monkeys[monkey].throw(new_val.clone());
                monkeys[to_whom].items.push_back(new_val);
            }
        }
    }

    inspections.sort();
    let monkey_business = inspections.iter().rev().take(2).fold(1usize, |x, a| x * a);

    println!("{monkey_business}");
    Ok(())
}

struct Monkey {
    items: VecDeque<u64>,
    worry_level_tester: u64,
    worry_amplifier: Box<dyn Fn(u64) -> u64 + 'static>,
    throw_chooser: Box<dyn Fn(bool) -> usize + 'static>,
}

impl Monkey {
    fn new<F, F2>(items: &[u64], tester: u64, amplifier: F, chooser: F2) -> Self
    where
        F: Fn(u64) -> u64 + 'static,
        F2: Fn(bool) -> usize + 'static,
    {
        Monkey {
            items: VecDeque::from_iter(items.iter().copied()),
            worry_level_tester: tester,
            worry_amplifier: Box::new(amplifier),
            throw_chooser: Box::new(chooser),
        }
    }

    fn throw(&self, item: u64) -> usize {
        (self.throw_chooser)(item % self.worry_level_tester == 0)
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    Vec::new()
}

fn puzzle1(input: &str) -> Result<()> {
    Ok(())
}

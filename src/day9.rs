use std::{collections::HashSet, str::FromStr};

use crate::Result;

#[derive(Debug, Copy, Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Movement(Move, u8);

impl FromStr for Movement {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s: Vec<&str> = s.split_ascii_whitespace().collect();
        let motion = match &s[..] {
            ["U", x] => Movement(Move::Up, x.parse::<u8>()?),
            ["D", x] => Movement(Move::Down, x.parse::<u8>()?),
            ["L", x] => Movement(Move::Left, x.parse::<u8>()?),
            ["R", x] => Movement(Move::Right, x.parse::<u8>()?),
            [..] => {
                return Err("error parsing".into());
            }
        };

        Ok(motion)
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Knot {
    x: isize,
    y: isize,
}

impl Knot {
    fn follow_head(&mut self, head: Knot) {
        match (head.x == self.x, head.y == self.y) {
            // Head is on tail, do nothing
            (true, true) => (),

            // head and tail are in same column
            (true, false) => {
                if head.y < self.y {
                    if self.y - 1 != head.y {
                        self.y -= 1;
                    }
                } else {
                    if self.y + 1 != head.y {
                        self.y += 1;
                    }
                }
            }

            // head and tail are in same row
            (false, true) => {
                if head.x < self.x {
                    if self.x - 1 != head.x {
                        self.x -= 1;
                    }
                } else {
                    if self.x + 1 != head.x {
                        self.x += 1;
                    }
                }
            }

            // head and tail are not in same row or column
            (false, false) => {
                if head.y < self.y && head.x < self.x {
                    if self.y - 1 == head.y && self.x - 1 == head.x {
                    } else {
                        self.y -= 1;
                        self.x -= 1;
                    }
                } else if head.y < self.y && head.x > self.x {
                    if self.y - 1 == head.y && self.x + 1 == head.x {
                    } else {
                        self.y -= 1;
                        self.x += 1;
                    }
                } else if head.y > self.y && head.x > self.x {
                    if self.y + 1 == head.y && self.x + 1 == head.x {
                    } else {
                        self.y += 1;
                        self.x += 1;
                    }
                } else if head.y > self.y && head.x < self.x {
                    if self.y + 1 == head.y && self.x - 1 == head.x {
                    } else {
                        self.y += 1;
                        self.x -= 1;
                    }
                }
            }
        }
    }
}

pub fn run() -> Result<()> {
    println!("* Day 9 *");

    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day9/input.txt"));
    // let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day9/test"));

    puzzle1(input)?;
    puzzle2(input)?;
    Ok(())
}

fn puzzle1(input: &str) -> Result<()> {
    let mut visited = HashSet::new();
    let mut head = Knot { x: 0, y: 0 };
    let mut tail = Knot { x: 0, y: 0 };
    visited.insert((tail.x, tail.y));
    for line in input.lines() {
        let Movement(motion, steps) = Movement::from_str(line)?;

        for _step in (0..steps).rev() {
            match motion {
                Move::Up => head.y += 1,
                Move::Down => head.y -= 1,
                Move::Left => head.x -= 1,
                Move::Right => head.x += 1,
            }

            tail.follow_head(head);
            visited.insert((tail.x, tail.y));
        }
    }
    println!("{}", visited.len());
    Ok(())
}
fn puzzle2(input: &str) -> Result<()> {
    let mut visited = HashSet::new();
    let mut head = Knot { x: 0, y: 0 };
    let mut n1 = Knot { x: 0, y: 0 };
    let mut n2 = Knot { x: 0, y: 0 };
    let mut n3 = Knot { x: 0, y: 0 };
    let mut n4 = Knot { x: 0, y: 0 };
    let mut n5 = Knot { x: 0, y: 0 };
    let mut n6 = Knot { x: 0, y: 0 };
    let mut n7 = Knot { x: 0, y: 0 };
    let mut n8 = Knot { x: 0, y: 0 };
    let mut n9 = Knot { x: 0, y: 0 };
    visited.insert((n9.x, n9.y));
    for line in input.lines() {
        let Movement(motion, steps) = Movement::from_str(line)?;

        for _step in (0..steps).rev() {
            match motion {
                Move::Up => head.y += 1,
                Move::Down => head.y -= 1,
                Move::Left => head.x -= 1,
                Move::Right => head.x += 1,
            }

            n1.follow_head(head);
            n2.follow_head(n1);
            n3.follow_head(n2);
            n4.follow_head(n3);
            n5.follow_head(n4);
            n6.follow_head(n5);
            n7.follow_head(n6);
            n8.follow_head(n7);
            n9.follow_head(n8);
            visited.insert((n9.x, n9.y));
        }
    }
    println!("{}", visited.len());

    Ok(())
}

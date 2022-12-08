use std::{
    cell::{RefCell, RefMut},
    collections::{HashMap, VecDeque},
    rc::Rc,
    str::FromStr,
};

use crate::Result;

pub fn run() -> Result<()> {
    println!("* Day 7 *");

    let input = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input/day7/input"));

    puzzle1(input)?;
    // puzzle2(input)?;
    Ok(())
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    In(String),
}

#[derive(Debug, Clone)]
enum Command {
    ShowDir,
    ChangeDir(Direction),
}

#[derive(Debug, Clone)]
enum Listing {
    File(u64, String),
    Dir(String),
}

#[derive(Debug, Clone)]
enum Line {
    Command(Command),
    Listing(Listing),
}

impl FromStr for Line {
    type Err = crate::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tokens: Vec<&str> = s.split_ascii_whitespace().collect();
        let line = match &tokens[..] {
            ["$", cmd @ ..] => match cmd {
                ["ls"] => Line::Command(Command::ShowDir),
                ["cd", ".."] => Line::Command(Command::ChangeDir(Direction::Up)),
                ["cd", x] => Line::Command(Command::ChangeDir(Direction::In(x.to_string()))),
                [] | [..] => return Err(format!("failed to parse command ({s}))").into()),
            },
            ["dir", x] => Line::Listing(Listing::Dir(x.to_string())),
            [size, name] => {
                let size = size.parse::<u64>()?;
                Line::Listing(Listing::File(size, name.to_string()))
            }
            [] | [..] => return Err(format!("failed to parse line ({s})").into()),
        };

        Ok(line)
    }
}

type ChildDir = Rc<RefCell<HashMap<String, Node>>>;

#[derive(Debug)]
enum Node {
    Directory(ChildDir),
    File(u64),
}

impl Clone for Node {
    fn clone(&self) -> Self {
        match self {
            Node::Directory(n) => Node::Directory(Rc::clone(n)),
            Node::File(size) => Node::File(*size),
        }
    }
}

impl Node {
    fn new_directory() -> Self {
        Self::Directory(Rc::new(RefCell::new(HashMap::new())))
    }

    fn add_directory(&self, name: &str) {
        match self {
            Node::Directory(children) => children
                .borrow_mut()
                .insert(name.into(), Node::new_directory()),
            Node::File(_) => panic!("can't add directory to a file"),
        };
    }

    fn add_file(&self, name: &str, size: u64) {
        match self {
            Node::Directory(children) => {
                children.borrow_mut().insert(name.into(), Node::File(size))
            }
            Node::File(_) => panic!("can't add file to a file"),
        };
    }

    fn get_children(&self) -> Result<ChildDir> {
        match self {
            Node::Directory(children) => Ok(Rc::clone(children)),
            Node::File(_) => panic!("can't operate on a file"),
        }
    }

    fn calculate_size(&self) -> u64 {
        match self {
            Node::Directory(children) => children
                .borrow()
                .iter()
                .map(|(_k, v)| v.calculate_size())
                .sum(),
            Node::File(size) => *size,
        }
    }

    fn get_all_directories(&self) -> Vec<u64> {
        let mut childs = Vec::new();
        match self {
            Node::Directory(children) => {
                for (_k, node) in children.borrow().iter() {
                    match node {
                        dir @ Node::Directory(_) => childs.push(dir.calculate_size()),
                        Node::File(_) => (),
                    }
                    childs.extend_from_slice(&node.get_all_directories());
                }
            }
            Node::File(_) => {}
        }

        childs
    }
}

fn get_node(dir: ChildDir, name: &str) -> Result<Node> {
    dir.borrow()
        .get(name)
        .ok_or_else(|| "node not found".into())
        .cloned()
}

fn puzzle1(input: &str) -> Result<()> {
    let mut root = Node::new_directory();

    let mut stack = VecDeque::new();
    stack.push_back(root);

    for line in input.lines().skip(1) {
        let line = Line::from_str(line)?;
        match line {
            Line::Command(Command::ShowDir) => {}
            Line::Command(Command::ChangeDir(dir)) => match dir {
                Direction::Up => {
                    let _ = stack.pop_back();
                }
                Direction::In(x) => {
                    let children = stack
                        .back()
                        .expect("stack shouldn't be empty")
                        .get_children()?;
                    stack.push_back(get_node(children, &x)?);
                }
            },
            Line::Listing(Listing::File(size, name)) => {
                stack
                    .back()
                    .expect("stack shouldn't be empty")
                    .add_file(&name, size);
            }
            Line::Listing(Listing::Dir(name)) => {
                stack
                    .back()
                    .expect("stack shouldn't be empty")
                    .add_directory(&name);
            }
        }
    }

    let root = stack.pop_front().unwrap();
    let all_dirs = root.get_all_directories();
    println!(
        "{}",
        all_dirs
            .iter()
            .filter(|&&size| size <= 100000u64)
            .sum::<u64>()
    );

    // oops didn't separate out puzzle 2
    // just thakful to get this done!
    let used_space = root.calculate_size();
    let free_space = 70000000u64 - used_space;
    let size_needed = 30000000u64 - free_space;
    println!(
        "{}",
        all_dirs
            .iter()
            .filter(|&&size| size >= size_needed)
            .min()
            .unwrap()
    );
    Ok(())
}

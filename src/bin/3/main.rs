#![feature(min_const_generics)]

use input::INPUT;
use std::fmt::Display;

mod input;

fn main() {
    let forest = Forest::new(INPUT.to_vec());

    println!("{}", forest);
}

#[derive(Debug)]
struct Forest<'a> {
    lines: Vec<ForestLine<'a>>,
}

#[derive(Debug)]
struct ForestLine<'a> {
    spaces: Vec<Space>,
    string: &'a str,
}

impl<'a> Forest<'a> {
    fn new(input: Vec<&'a str>) -> Forest {
        Forest {
            lines: input.iter().map(|line| ForestLine::new(line)).collect(),
        }
    }
    fn end_height(&self) -> u32 {
        self.lines.len() as u32
    }
}

impl<'a> std::fmt::Display for Forest<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self
            .lines
            .iter()
            .fold("".to_string(), |acc, line| format!("{}\n{}", acc, line));
        write!(f, "{}", output)
    }
}
impl<'a> ForestLine<'a> {
    fn new(string: &str) -> ForestLine {
        ForestLine {
            spaces: string.chars().map(|c| Space::new(c)).collect(),
            string,
        }
    }
    fn emoji_output(&self) -> String {
        // self.0.replace(".", "⬛").replace("#", "🌲")
        self.spaces
            .iter()
            .map(|space| space.to_string())
            .collect::<String>()
    }
}
impl<'a> Display for ForestLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|", self.emoji_output())
    }
}

#[derive(Debug)]
struct Space {
    tree: bool,
}

impl Space {
    fn new(c: char) -> Space {
        Space {
            tree: match c {
                '#' => true,
                _ => false,
            },
        }
    }
}
impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.tree {
            true => "🌲",
            false => "⬛",
        };
        write!(f, "{}", c)
    }
}

struct Sled {
    tree_hits: u32,
    position: (u32, u32),
    delta_pos: (u32, u32),
    forest: Forest<'static>,
}

impl Sled {
    fn new(delta_pos: (u32, u32), forest: Forest<'static>) -> Sled {
        Sled {
            delta_pos,
            forest,
            position: (0, 0),
            tree_hits: 0,
        }
    }
    fn slide() {}
}

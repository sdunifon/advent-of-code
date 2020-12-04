#![feature(min_const_generics)]

use std::fmt::Display;

use input::INPUT;

mod input;

fn main() {
    let forest = Forest::new(INPUT);

    println!("{}", forest);
}

#[derive(Debug)]
struct Forest<'a> {
    lines: Vec<ForestLine<'a>>,
}

#[derive(Debug)]
struct ForestLine<'a>(&'a str);

// 🌲⬜ ⚪⬛
impl<'a> Forest<'a> {
    fn new(input: Vec<&'a str>) -> Forest {
        Forest {
            lines: input.iter().map(|line| ForestLine(line)).collect(),
        }
    }
}

impl<'a> std::fmt::Display for Forest<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = self.lines.join("\n");
        write!(f, "{}", output)
    }
}
impl<'a> ForestLine<'a> {
    fn emoji_output(&self) -> String {
        self.0.replace(".", "⬛").replace("#", "🌲")
    }
}
impl<'a> Display for ForestLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-{}-", self.emoji_output())
    }
}

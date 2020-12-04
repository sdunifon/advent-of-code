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

// impl<'a, const COUNT: usize> std::fmt::Display for Forest<'a, COUNT> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let output = self.lines.join("\n");
//         write!(f, "{}", output)
//     }
// }

// impl Display for ForestLine<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "-{}-", self.0)
//     }
// }

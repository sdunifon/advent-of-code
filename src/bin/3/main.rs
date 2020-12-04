#![feature(min_const_generics)]
#![feature(array_map)]
use input::INPUT;
use std::fmt::Display;

mod input;

fn main() {
    let forest = Forest::new(INPUT.to_vec());
    println!("{}", forest);
    let mut sled = Sled::new((3, 1), forest);
    sled.slide();
    println!("Tree sled hits:{}", sled.tree_hits);
    let sled_runs = [
        Sled::new((1, 1), Forest::new(INPUT.to_vec())),
        Sled::new((3, 1), Forest::new(INPUT.to_vec())),
        Sled::new((5, 1), Forest::new(INPUT.to_vec())),
        Sled::new((7, 1), Forest::new(INPUT.to_vec())),
        Sled::new((1, 2), Forest::new(INPUT.to_vec())),
    ];
    let sled_run_hits = sled_runs.map(|mut run| run.slide().tree_hits);
    dbg!(&sled_run_hits);
    let answer = sled_run_hits.iter().fold(1, |x, acc| x * acc);

    println!("Answer:{}", answer);
}

#[derive(Debug)]
struct Forest<'a> {
    lines: Vec<ForestLine<'a>>,
}

#[derive(Debug)]
struct ForestLine<'a> {
    spaces: Vec<Space>,
    string: &'a str,
    iter_pos: usize,
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
    fn tree_at(&mut self, x: u32, y: u32) -> bool {
        self.lines
            .get_mut(y as usize)
            .unwrap()
            .nth(x as usize)
            .unwrap()
            .tree
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
            iter_pos: 0,
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
impl<'a> Iterator for ForestLine<'a> {
    type Item = Space;

    fn next(&mut self) -> Option<Self::Item> {
        let this_index = self.iter_pos % self.spaces.len();
        self.iter_pos += 1;
        Some(self.spaces[this_index])
    }
}
impl<'a> Display for ForestLine<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "|{}|", self.emoji_output())
    }
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug)]
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
    fn slide(&mut self) -> &mut Sled {
        for _i in 0..self.forest.end_height() / self.delta_pos.1 {
            let (x_pos, y_pos) = self.position;
            if self.forest.tree_at(x_pos, y_pos) {
                self.tree_hits += 1;
            }

            self.move_sled()
        }
        return self;
    }

    fn move_sled(&mut self) {
        self.position.0 += self.delta_pos.0;
        self.position.1 += self.delta_pos.1;
    }
}

#[cfg(test)]
mod test {

    use super::*;
}

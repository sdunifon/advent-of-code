use std::{collections::HashMap, iter::FromIterator};

use input::INPUT;
mod input;

fn main() {
    let passport_db = PassportDb::new(INPUT);
    dbg!(passport_db);
}

#[derive(Debug)]
struct PassportDb<'a> {
    passports: Vec<Passport<'a>>,
}

impl<'a> PassportDb<'a> {
    fn new(input: &'a str) -> Self {
        let a: Vec<Passport> = input
            .split("\n\n")
            .map(|x| Passport::new(x))
            .collect::<Vec<Passport>>();
        PassportDb { passports: a }
    }
}

#[derive(Debug, Clone)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
    // fields: &'a str,
}

// impl FromIterator {}
impl<'a> Passport<'a> {
    fn new(input: &'a str) -> Passport {
        let fields: Vec<&str> = input
            .split(&['\n', ' '][..])
            .filter(|str| !str.is_empty())
            .collect();
        // let hash_strs: Vec<(&str, &str)> = fields
        let hash_strs: Vec<(&str, &str)> = fields
            .iter()
            .map(|str| {
                let mut split_iter = str.split(":").take(2);
                (split_iter.next().unwrap(), split_iter.next().unwrap())
            })
            .collect();

        let hash_map: HashMap<_, _> = hash_strs.into_iter().collect();

        Passport { fields: hash_map }
    }
}

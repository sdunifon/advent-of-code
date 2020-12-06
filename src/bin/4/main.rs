use std::{collections::HashMap, error::Error};

use input::INPUT;
mod input;

fn main() {
    let passport_db = PassportDb::new(INPUT);
    println!("valid count:{}", passport_db.valid_count());
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
    fn valid_count(&self) -> usize {
        // dbg!(self.passports.iter().filter(|passport| !passport.valid()));
        self.passports
            .iter()
            .filter(|passport| passport.valid())
            .count()
    }
}

#[derive(Debug, Clone)]
struct Passport<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Passport<'a> {
    fn new(input: &'a str) -> Passport {
        let fields: HashMap<_, _> = input
            .split(&['\n', ' '][..])
            .filter(|str| !str.is_empty())
            .map(|str| {
                let mut split_iter = str.split(":").take(2);
                (split_iter.next().unwrap(), split_iter.next().unwrap())
            })
            .collect();

        Passport { fields }
    }
    fn valid(&self) -> bool {
        if self.fields.len() >= 8 {
            return true;
        } else if self.fields.get("cid").is_none() && self.fields.len() == 7 {
            return true;
        }

        self.field_validator().unwrap();
        return false;
    }
    fn field_validator(&self) -> Result<bool, Box<dyn Error>> {
        self.fields.get("hcl").unwrap();

        Ok(true)
    }
}

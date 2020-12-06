use std::{collections::HashSet, iter::FromIterator};

use input::INPUT;

mod input;

fn main() {
    let r = load_groups();
    dbg!(r);
}

fn load_groups() -> Vec<Group> {
    let groups: Vec<Group> = INPUT
        .split("\n\n")
        .map(|input_str| Group {
            answer_sets: input_str
                .split("\n")
                .map(|answer_str| AnswerSet::new(answer_str))
                .collect(),
        })
        .collect();
    groups
}
#[derive(Debug)]
struct Group {
    // answer_sets: Vec<&'a str>,
    answer_sets: Vec<AnswerSet>,
}
impl Group {
    fn group_answers(&self) -> HashSet<char> {
        self.answer_sets
            .iter()
            .fold(HashSet::new(), |acc, set| {
                acc.union(&set.0).collect::<HashSet<&char>>(
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct AnswerSet(HashSet<char>);
// {
//     answers: HashSet<char>,
// }

impl AnswerSet {
    fn new(input: &str) -> AnswerSet {
        AnswerSet(input.chars().collect())
        // AnswerSet {
        //     answers: input.chars().collect(),
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_ne!(INPUT, "asdf");
    }
    #[test]
    fn load_groups_test() {
        assert!(load_groups().get(0).is_some());
        assert_eq!(load_groups().len(), 483);
    }

    #[test]
    fn valid_group() {
        let groups = load_groups();

        dbg!(groups.get(0).unwrap());
        assert_eq!(groups.get(0).unwrap().answer_sets.len(), 2);

        assert_eq!(groups.get(1).unwrap().answer_sets.len(), 4);
        assert_eq!(groups.get(2).unwrap().answer_sets.len(), 4);
    }

    #[test]
    fn answer_set_test() {
        let groups = load_groups();
        let group3 = groups.get(2).unwrap();

        assert!(group3.answer_sets.get(2).unwrap().0.get(&'2').is_some());
    }
    #[test]
    fn hash_set_test() {
        let groups = load_groups();
        let answer_sets = groups.get(0).unwrap().answer_sets;
        let answer_set_0 = answer_sets[0];
        let answer_set_1 = answer_sets[1];
        let union = answer_set_0.0.union(&answer_set_1.0);
    }
    #[test]
    fn group_answer_test() {
        let groups = load_groups();

        assert_eq!(groups.get(0).unwrap().answer_sets.len(), 2);
    }
}

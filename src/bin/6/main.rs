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

#[derive(Debug)]
struct AnswerSet {
    answers: Vec<char>,
}

impl AnswerSet {
    fn new(input: &str) -> AnswerSet {
        AnswerSet {
            answers: input.chars().collect(),
        }
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

        assert_eq!(
            group3.answer_sets.get(2).unwrap().answers.get(2).unwrap(),
            &'e'
        );
    }
}

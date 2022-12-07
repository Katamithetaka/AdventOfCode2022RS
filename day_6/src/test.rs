#[allow(unused_imports)]
use crate::{part_1, part_2};

#[test]
fn part_1_test() {
    let input = common::get_demo_input().expect("Couldn't read input");
    let answers =
        std::fs::read_to_string("demo_answers_1").expect("Couldn't read answers for part 1");
    let answers = answers.lines().collect::<Vec<_>>();

    input.lines().enumerate().for_each(|(index, f)| {
        let value = answers[index].parse::<u32>().unwrap();
        assert!(
            part_1(&f.to_string()) == value,
            "part_1({}) != {}",
            f,
            value
        );
    })
}

#[test]
fn part_2_test() {
    let input = common::get_demo_input().expect("Couldn't read input");
    let answers =
        std::fs::read_to_string("demo_answers_2").expect("Couldn't read answers for part 2");
    let answers = answers.lines().collect::<Vec<_>>();

    input.lines().enumerate().for_each(|(index, f)| {
        let value = answers[index].parse::<u32>().unwrap();
        assert!(
            part_2(&f.to_string()) == value,
            "part_2({}) != {}",
            f,
            value
        );
    })
}

#[allow(unused_imports)]
use crate::{part_1, part_2};

#[test]
fn part_1_test() {
    let input = common::get_demo_input_at(1).expect("Couldn't read input");
    let answer = part_1(&input);
    let expected_answer = 13;
    assert!(
        answer == expected_answer,
        "expected {}, got {}",
        expected_answer,
        answer
    );
}

#[test]
fn part_2_test() {
    {
        let input = common::get_demo_input_at(1).expect("Couldn't read input");
        let answer = part_2(&input);
        let expected_answer = 140;
        assert!(
            answer == expected_answer,
            "expected {}, got {}",
            expected_answer,
            answer
        );
    }
}

#[allow(unused_imports)]
use crate::{part_1, part_2};

#[test]
fn part_1_test() {
    let input = common::get_demo_input().expect("Couldn't read input");

    assert!(part_1(&input) == 95437);
}

#[test]
fn part_2_test() {
    let input = common::get_demo_input().expect("Couldn't read input");

    assert!(part_2(&input) == 24933642);
}

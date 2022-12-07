mod utils;
use std::str::FromStr;

use utils::{Action, Move};
fn part_1(input: &String) -> u64 {
    println!("Part 1!");

    // Input format:
    // Each line HAS to contain two letters separated by a space
    // the first letter must be A B or C
    // the second letter must be X Y or Z

    // Step 1: Get each line of the input;
    let moves = input.lines();

    // Step 2: Get the score from each line;
    let scores = moves.map(|it| {
        let v: Vec<_> = it.split_whitespace().collect();
        let left_move = Move::from_str(v[0]).unwrap();
        let right_move = Move::from_str(v[1]).unwrap();
        return right_move.get_score(&left_move) as u64;
    });

    // Step 3: Make the sum of it all
    let score: u64 = scores.sum();

    println!("The total score is: {}", score);

    return score;
}

fn part_2(input: &String) -> u64 {
    println!("Part 2!");
    // Input format:
    // Each line HAS to contain two letters separated by a space
    // the first letter must be A B or C
    // the second letter must be X Y or Z

    // Step 1: Get each lines of the input;
    let moves = input.lines();

    // Step 2: Get the score from each line;
    let scores = moves.map(|it| {
        let v: Vec<_> = it.split_whitespace().collect();
        let left_move = Move::from_str(v[0]).unwrap();
        let action = Action::from_str(v[1]).unwrap();
        return left_move.get_score_from_action(&action) as u64;
    });

    // Step 3: Make the sum of it all
    let score: u64 = scores.sum();

    println!("The total score is: {}", score);

    return score;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    part_1(&input);
    part_2(&input);
}

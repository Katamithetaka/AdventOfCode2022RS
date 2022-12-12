use std::{cell::RefCell, rc::Rc};

use crate::utils::Monkey;

mod test;
mod utils;

fn part_1(input: &String) -> usize {
    println!("Part 1!");
    let mut monkeys = vec![Rc::new(RefCell::new(Monkey::new()))];
    let mut index = 0usize;
    let (monkeys, _) = input
        .lines()
        .fold((&mut monkeys, &mut index), |(vec, index), line| {
            let line = line.trim();

            if line.is_empty() {
                vec.push(Rc::new(RefCell::new(Monkey::new())));
                *index += 1;

                return (vec, index);
            }

            if line.starts_with("Monkey") {
            } else if line.starts_with("Starting items:") {
                let (_, items) = line.split_once(":").unwrap();
                let mut items_vec = vec![];
                let items = items.split(",").fold(&mut items_vec, |items, item| {
                    items.push(item.trim().parse::<usize>().unwrap());
                    return items;
                });

                vec[*index].borrow_mut().set_items(items.clone());
            } else if line.starts_with("Operation:") {
                let (_, op_str) = line.split_once(":").unwrap();
                let mut state = 0;
                let mut lhs = "old";
                let mut op = "*";
                let mut rhs = "old";
                for item in op_str.trim().split(" ") {
                    let item = item.trim();
                    match (item, state) {
                        ("new", 0) => state += 1,
                        ("=", 1) => state += 1,
                        ("old", 2) => {
                            state += 1;
                            lhs = "old";
                        }
                        (left, 2) => {
                            state += 1;
                            lhs = left;
                            lhs.clone()
                                .parse::<i32>()
                                .expect("Value for LHS should either be old or a number!");
                        }
                        (new_op, 3) => {
                            state += 1;
                            op = new_op;
                        }
                        ("old", 4) => {
                            state += 1;
                            rhs = "old"
                        }
                        (right, 4) => {
                            state += 1;
                            rhs = right;
                            right
                                .clone()
                                .parse::<i32>()
                                .expect("Value for LHS should either be old or a number!");
                        }
                        (_, _) => panic!("Invalid parameter for operation {:?}", (item, state)),
                    }
                }
                vec[*index].borrow_mut().set_operator_from(lhs, op, rhs);
            } else if line.starts_with("Test:") {
                let number = line.split(" ").last().unwrap().parse().unwrap();
                vec[*index].borrow_mut().set_test_from_dividable_by(number);
            } else if line.starts_with("If true:") {
                let number = line.split(" ").last().unwrap().parse().unwrap();
                vec[*index].borrow_mut().set_throw_to_if_true(number);
            } else if line.starts_with("If false:") {
                let number = line.split(" ").last().unwrap().parse().unwrap();
                vec[*index].borrow_mut().set_throw_to_if_false(number);
            }

            (vec, index)
        });
    for i in 0..20 {
        println!("Round {}", i);
        let monkeys_scores = monkeys
            .iter()
            .map(|f| {
                (
                    f.borrow().get_items_count(),
                    f.borrow().get_appraisal_count(),
                )
            })
            .collect::<Vec<_>>();
        for y in 0..monkeys.len() {
            Rc::clone(&monkeys[y]).borrow_mut().turn(monkeys);
        }
        println!("{:?}\nAfter Round {}", monkeys_scores, i);
        let monkeys_scores = monkeys
            .iter()
            .map(|f| {
                (
                    f.borrow().get_items_count(),
                    f.borrow().get_appraisal_count(),
                )
            })
            .collect::<Vec<_>>();
        println!("{:?}", monkeys_scores);
    }
    monkeys.sort_by(|a, b| {
        a.borrow()
            .get_appraisal_count()
            .cmp(&b.borrow().get_appraisal_count())
    });
    let monkeys_scores = monkeys
        .iter()
        .map(|f| f.borrow().get_appraisal_count())
        .collect::<Vec<_>>();

    monkeys_scores[monkeys_scores.len() - 2] * monkeys_scores[monkeys_scores.len() - 1]
}

fn part_2(input: &String) -> usize {
    println!("Part 2!");
    let mut monkeys = vec![Rc::new(RefCell::new(Monkey::new()))];
    let mut index = 0usize;
    let (monkeys, _) = input
        .lines()
        .fold((&mut monkeys, &mut index), |(vec, index), line| {
            let line = line.trim();

            if line.is_empty() {
                vec.push(Rc::new(RefCell::new(Monkey::new())));
                *index += 1;

                return (vec, index);
            }

            if line.starts_with("Monkey") {
            } else if line.starts_with("Starting items:") {
                let (_, items) = line.split_once(":").unwrap();
                let mut items_vec = vec![];
                let items = items.split(",").fold(&mut items_vec, |items, item| {
                    items.push(item.trim().parse::<usize>().unwrap());
                    return items;
                });

                vec[*index].borrow_mut().set_items(items.clone());
            } else if line.starts_with("Operation:") {
                let (_, op_str) = line.split_once(":").unwrap();
                let mut state = 0;
                let mut lhs = "old";
                let mut op = "*";
                let mut rhs = "old";
                for item in op_str.trim().split(" ") {
                    let item = item.trim();
                    match (item, state) {
                        ("new", 0) => state += 1,
                        ("=", 1) => state += 1,
                        ("old", 2) => {
                            state += 1;
                            lhs = "old";
                        }
                        (left, 2) => {
                            state += 1;
                            lhs = left;
                            lhs.clone()
                                .parse::<i32>()
                                .expect("Value for LHS should either be old or a number!");
                        }
                        (new_op, 3) => {
                            state += 1;
                            op = new_op;
                        }
                        ("old", 4) => {
                            state += 1;
                            rhs = "old"
                        }
                        (right, 4) => {
                            state += 1;
                            rhs = right;
                            right
                                .clone()
                                .parse::<i32>()
                                .expect("Value for LHS should either be old or a number!");
                        }
                        (_, _) => panic!("Invalid parameter for operation {:?}", (item, state)),
                    }
                }
                vec[*index].borrow_mut().set_operator_from(lhs, op, rhs);
            } else if line.starts_with("Test:") {
                let number = line.split(" ").last().unwrap().parse().unwrap();
                vec[*index].borrow_mut().set_test_from_dividable_by(number);
            } else if line.starts_with("If true:") {
                let number = line.split(" ").last().unwrap().parse().unwrap();
                vec[*index].borrow_mut().set_throw_to_if_true(number);
            } else if line.starts_with("If false:") {
                let number = line.split(" ").last().unwrap().parse().unwrap();
                vec[*index].borrow_mut().set_throw_to_if_false(number);
            }

            (vec, index)
        });
    let modulo: usize = monkeys.iter().map(|m| m.borrow().get_modulo()).product();
    for i in 0..10_000 {
        println!("Round {}", i);
        let monkeys_scores = monkeys
            .iter()
            .map(|f| {
                (
                    f.borrow().get_items_count(),
                    f.borrow().get_appraisal_count(),
                )
            })
            .collect::<Vec<_>>();
        for y in 0..monkeys.len() {
            Rc::clone(&monkeys[y]).borrow_mut().turn_p2(modulo, monkeys);
        }
        println!("{:?}\nAfter Round {}", monkeys_scores, i);
        let monkeys_scores = monkeys
            .iter()
            .map(|f| {
                (
                    f.borrow().get_items_count(),
                    f.borrow().get_appraisal_count(),
                )
            })
            .collect::<Vec<_>>();
        println!("{:?}", monkeys_scores);
    }
    monkeys.sort_by(|a, b| {
        a.borrow()
            .get_appraisal_count()
            .cmp(&b.borrow().get_appraisal_count())
    });
    let monkeys_scores = monkeys
        .iter()
        .map(|f| f.borrow().get_appraisal_count())
        .collect::<Vec<_>>();

    monkeys_scores[monkeys_scores.len() - 2] * monkeys_scores[monkeys_scores.len() - 1]
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

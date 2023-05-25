use std::{cell::RefCell, rc::Rc};

use crate::utils::PacketValue;


mod test;
mod utils;

fn part_1(input: &String) -> usize {
    println!("Part 1!");

    let input = input.lines().filter_map(|f| {
        if f.is_empty() {
            None
        }
        else {
            Some(PacketValue::from(f))
        }
    }).collect::<Vec<_>>();

    input.chunks_exact(2).enumerate().filter_map(|(index, v)| {
        if v[0] < v[1] {
            Some(index+1)
        }
        else {
            None
        }
    }).sum()
}

fn part_2(input: &String) -> usize {
    println!("Part 2!");

    let mut input = input.lines().filter_map(|f| {
        if f.is_empty() {
            None
        }
        else {
            Some((PacketValue::from(f), false))
        }
    }).collect::<Vec<_>>();
    let v1 = (PacketValue::List(vec![PacketValue::List(vec![PacketValue::Number(2)])]), true);
    let v2 = (PacketValue::List(vec![PacketValue::List(vec![PacketValue::Number(6)])]), true);

    input.push(v1.clone());
    input.push(v2.clone());

    input.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    (input.iter().position(|a| a == &v1).unwrap() + 1) *
    (input.iter().position(|b| b == &v2).unwrap() + 1) 
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

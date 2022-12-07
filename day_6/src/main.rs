mod test;
mod utils;

use utils::unique;

fn part_1(input: &String) -> u32 {
    println!("Part 1!");

    let chars = input.chars().collect::<Vec<_>>();

    for i in 0..(chars.len() - 3) {
        let to_find = chars
            .iter()
            .skip(i)
            .take(4)
            .map(|f| f.clone())
            .collect::<Vec<_>>();

        if unique(&to_find) {
            return i as u32 + 4;
        }
    }

    panic!("Not found!");
}

fn part_2(input: &String) -> u32 {
    println!("Part 2!");

    let chars = input.chars().collect::<Vec<_>>();

    for i in 0..(chars.len() - 13) {
        let to_find = chars
            .iter()
            .skip(i)
            .take(14)
            .map(|f| f.clone())
            .collect::<Vec<_>>();

        if unique(&to_find) {
            return i as u32 + 14;
        }
    }

    panic!("Not found!");
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

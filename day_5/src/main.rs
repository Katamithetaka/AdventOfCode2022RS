mod test;
use std::collections::VecDeque;

fn part_1(input: &String) -> String {
    println!("Part 1!");

    // Input format:
    // Each line is formatted this way: number1-number2,number3-number4
    // Where each number has to be an unsigned integer.

    // Step 1: Get each line of the input;

    let new_line = if input.contains("\r\n") { "\r\n" } else { "\n" };

    let double_new_line = new_line.to_owned() + new_line;

    let (crates, moves) = common::split_at(input, &double_new_line).unwrap();
    println!("{}{}", crates, moves);

    let last_line = crates.lines().last().unwrap();
    let last_line = last_line
        .split(" ")
        .map(|f| f.parse::<u32>())
        .filter_map(|f| f.ok())
        .collect::<Vec<_>>();

    let number_elements = last_line
        .last()
        .expect("Empty line when expected number of cargo columns");

    // Collect crates in VecDequeues
    let mut result = vec![];
    result.resize(*number_elements as usize, VecDeque::new());
    crates.split(new_line).for_each(|f| {
        let mut i = 0;
        let v = f.chars().collect::<Vec<_>>();
        loop {
            if i / 4 >= result.len() || i + 2 >= v.len() {
                break;
            }

            let str = String::from_iter(v.iter().skip(i).take(4));
            if str.trim().contains('[') {
                result[i / 4].push_back(v[i + 1]);
            }

            i += 4;
        }
    });

    // Get each action
    moves.split(new_line).for_each(|f| {
        println!("{}", f);
        let actions = f
            .split(" ")
            .filter_map(|m| m.parse::<usize>().ok())
            .take(3)
            .collect::<Vec<_>>();

        if actions.len() != 3 {
            return;
        }

        // Map the data to variables to simplify the code
        let (count, from, to) = (actions[0], actions[1] - 1, actions[2] - 1);

        // move {count} from {from} to {to}
        for _ in 0..count {
            let end = result[from].pop_front().unwrap();
            result[to].push_front(end);
        }
    });

    // Get the first row of each columns
    let r = result.iter().fold(String::new(), |mut acc, f| {
        acc.push(if f.len() == 0 { ' ' } else { f[0] });
        acc
    });

    println!("{:?}", r);

    return r;
}

fn part_2(input: &String) -> String {
    println!("Part 2!");
    // Input format:
    // Each line is formatted this way: number1-number2,number3-number4
    // Where each number has to be an unsigned integer.

    // Step 1: Get each line of the input;

    let new_line = if input.contains("\r\n") { "\r\n" } else { "\n" };

    let double_new_line = new_line.to_owned() + new_line;

    let (crates, moves) = common::split_at(input, &double_new_line).unwrap();

    //println!("{}\n\n{}", crates, moves);

    let last_line = crates.lines().last().unwrap();
    let last_line = last_line
        .split(" ")
        .map(|f| f.parse::<u32>())
        .filter_map(|f| f.ok())
        .collect::<Vec<_>>();

    let number_elements = last_line
        .last()
        .expect("Empty line when expected number of cargo columns");

    let mut result = vec![];
    result.resize(*number_elements as usize, VecDeque::new());
    crates.split(new_line).for_each(|f| {
        let mut i = 0;
        let v = f.chars().collect::<Vec<_>>();
        loop {
            if i / 4 >= result.len() || i + 2 >= v.len() {
                break;
            }

            let str = String::from_iter(v.iter().skip(i).take(4));
            if str.trim().contains('[') {
                result[i / 4].push_back(v[i + 1])
            }

            i += 4;
        }
    });

    moves.split("\r\n").for_each(|f| {
        let actions = f
            .split(" ")
            .filter_map(|m| m.parse::<usize>().ok())
            .take(3)
            .collect::<Vec<_>>();

        if actions.len() != 3 {
            return;
        }

        let (count, from, to) = (actions[0], actions[1] - 1, actions[2] - 1);

        //println!("move {count} from {from} to {to}");

        for i in 0..count {
            let end = result[from][count - (i + 1)].clone();
            result[to].push_front(end);
        }

        for _ in 0..count {
            result[from].pop_front();
        }
    });

    let r = result.iter().fold(String::new(), |mut acc, f| {
        acc.push(if f.len() == 0 { ' ' } else { f[0] });
        acc
    });

    println!("{:?}", r);

    return r;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    part_1(&input);
    part_2(&input);
}

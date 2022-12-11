mod test;
mod utils;

fn add_to_signal_strength(x: i64, cycle_count: i64, signal_strength: &mut i64) {
    println!(
        "{} * {} = {}, {} + {} = {}",
        cycle_count,
        x,
        cycle_count * x,
        *signal_strength,
        cycle_count * x,
        *signal_strength + cycle_count * x
    );

    *signal_strength += x * cycle_count;
}

fn increase_cycle_count(x: i64, cycle_count: &mut i64, signal_strength: &mut i64) {
    *cycle_count = *cycle_count + 1;
    if (*cycle_count == 20) || (*cycle_count > 20 && (*cycle_count - 20) % 40 == 0) {
        add_to_signal_strength(x, *cycle_count, signal_strength)
    }
}

fn add_x(x: &mut i64, value: i64, cycle_count: &mut i64, signal_strength: &mut i64) {
    for _ in 0..2 {
        increase_cycle_count(*x, cycle_count, signal_strength);
    }
    *x += value;
}

fn part_1(input: &String) -> i64 {
    println!("Part 1!");

    let mut x = 1i64;
    let mut signal_strength = 0i64;
    let mut cycle_count = 0i64;

    input.lines().for_each(|f| {
        if f == "noop" {
            increase_cycle_count(x, &mut cycle_count, &mut signal_strength);
        } else {
            let (_, number_s) = f.split_once(" ").unwrap();
            let number: i64 = number_s.parse().unwrap();

            add_x(&mut x, number, &mut cycle_count, &mut signal_strength);
        }
    });

    return signal_strength;
}

fn increase_cycle_count_p2(x: i64, cycle_count: &mut i64, output: &mut String) {
    if *cycle_count % 40 == 0 && *cycle_count != 0 {
        *output += "\n";
    }
    let c = *cycle_count % 40;
    if x - 1 == c || x == c || x + 1 == c {
        *output += "#";
    } else {
        *output += ".";
    }
    *cycle_count = *cycle_count + 1;
}

fn add_x_p2(x: &mut i64, value: i64, cycle_count: &mut i64, output: &mut String) {
    for _ in 0..2 {
        increase_cycle_count_p2(*x, cycle_count, output);
    }
    *x += value;
}

fn part_2(input: &String) -> String {
    println!("Part 2!");
    let mut x = 1i64;
    let mut cycle_count = 0i64;
    let mut output = String::from("");

    input.lines().for_each(|f| {
        if f == "noop" {
            increase_cycle_count_p2(x, &mut cycle_count, &mut output);
        } else {
            let (_, number_s) = f.split_once(" ").unwrap();
            let number: i64 = number_s.parse().unwrap();

            add_x_p2(&mut x, number, &mut cycle_count, &mut output);
        }
    });

    return output;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

mod test;
mod utils;
use utils::get_value;

fn part_1(input: &String) -> i32 {
    println!("Part 1!");

    // Input format:
    // Each line HAS n characters from a - z or A - Z
    // Each half of the string only contains one TYPE of character in common
    // Each chunk of 3 lines contains only one type of character in common

    // Step 1: Get each line of the input;
    let rucksacks = input.lines();

    // Step 2: Get the score from each line;
    let rucksacks = rucksacks.map(|it| {
        let (compartment_1, compartment_2) = it.split_at(it.len() / 2);

        // Step 3: Get the two halves of the string
        let compartment_1: Vec<_> = compartment_1.chars().collect();
        let compartment_2: Vec<_> = compartment_2.chars().collect();

        return vec![compartment_1, compartment_2];
    });

    // Step 4: Find the duplicates between the two halves
    let duplicates_values = rucksacks.map(|it| {
        let first = &it[0];
        let second = &it[1];

        // Find if the second half contains first[i]
        for i in 0..first.len() {
            if second.contains(&first[i]) {
                return get_value(first[i] as i32);
            }
        }
        panic!("Shouldn't be getting here.");
    });

    // Step 5: Get the sum of duplicates
    let sum: i32 = duplicates_values.sum();

    println!("{}", sum);

    return sum;
}

fn part_2(input: &String) -> i32 {
    println!("Part 2!");

    // Input format:
    // Each line HAS n characters from a - z or A - Z
    // Each half of the string only contains one TYPE of character in common
    // Each chunk of 3 lines contains only one letter in common

    // Step 1: Get each line of the input;
    let lines = input.lines().collect::<Vec<_>>();

    // Step 2: Get the array in chunks of 3
    let rucksacks = lines.chunks(3);

    // Step 3: Find the duplicate characters in each chunks
    let badges = rucksacks.map(|it| {
        let first: Vec<_> = it[0].chars().collect();
        let second: Vec<_> = it[1].chars().collect();
        let third: Vec<_> = it[2].chars().collect();

        for i in 0..first.len() {
            // Find if second string contains first[i]
            let second_has_duplicate = second.contains(&first[i]);

            // Find if third contains first[i] (only executes if second contains first[i])
            let has_duplicate = second_has_duplicate && third.contains(&first[i]);

            if has_duplicate {
                return get_value(first[i] as i32);
            }
        }

        // Only gets here if the file is badly formatted
        assert!(false, "Shouldn't be getting here.");
        return 0;
    });

    // Step 4: Get the sum of all badges
    let sum = badges.sum::<i32>();

    println!("{}", sum);

    return sum;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    part_1(&input);
    part_2(&input);
}

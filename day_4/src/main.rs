mod test;
mod utils;
use utils::{get_ranges, min_max};

fn part_1(input: &String) -> u32 {
    println!("Part 1!");

    // Input format:
    // Each line is formatted this way: number1-number2,number3-number4
    // Where each number has to be an unsigned integer.

    // Step 1: Get each line of the input;
    let ranges = input.lines();

    // Step 2: Get the pairs that where one contains another from each line;
    
    let ranges_contained = ranges.map(|ranges| {
        // Parse the string
        let (pair_1, pair_2) = get_ranges(ranges);

        // Simple utility function to check if a range contains another
        let contains = |range_1: &(u32, u32), range_2: &(u32, u32)| {
            range_1.0 <= range_2.0 && range_1.1 >= range_2.1
        };


        // Check if pair_1 contains pair_2 or if pair_2 contains pair_1
        if contains(&pair_1, &pair_2) || contains(&pair_2, &pair_1) {
            return 1;
        } else {
            return 0;
        }
    });

    // Step 3: Get the sum
    let sum: u32 = ranges_contained.sum();

    println!("{}", sum);

    return sum;
}

fn part_2(input: &String) -> u32 {
    println!("Part 2!");

    // Step 1: Get each line of the input;
    let ranges = input.lines();

    // Step 2: Get the score from each line;
    
    let ranges_contained = ranges.map(|ranges| {
        // Parse the string
        let (pair_1, pair_2) = get_ranges(ranges);
        
        // Simple function that checks if one lap overlaps with another
        // for simplicity sake, assume range_1 is smaller than range_2
        // see min_max to understand what I mean with smaller.
        let overlaps = |range_1: &(u32, u32), range_2: &(u32, u32)| {
            range_1.0 + (range_1.1 - range_1.0) >= range_2.0
        };
        
        // get the min_max of the pair
        let (pair_1, pair_2) = min_max(pair_1, pair_2);

        // check if it overlaps
        if overlaps(&pair_1, &pair_2) {
            return 1;
        } else {
            return 0;
        }
    });

    // Step 3: Get the sum
    let sum: u32 = ranges_contained.sum();

    println!("{}", sum);
    return sum;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    part_1(&input);
    part_2(&input);
}

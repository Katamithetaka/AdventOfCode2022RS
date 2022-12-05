fn part_1() {
    println!("Part 1!");

    // Input format:
    // Each line is formatted this way: number1-number2,number3-number4
    // Where each number has to be an unsigned integer.
    let input = include_str!("input");

    // Step 1: Get each line of the input;
    let ranges = input.lines();

    // Step 2: Get the pairs that where one contains another from each line;
    let ranges_contained = ranges.map(|pair| {
        // Parse the string
        let pair = pair
            .split(",")
            .map(|sections| {
                sections
                    .split("-")
                    .map(|section_str| section_str.parse::<u32>().unwrap())
                    .take(2)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Simple utility function to check if a range contains another
        let contains = |range_1: &(u32, u32), range_2: &(u32, u32)| {
            range_1.0 <= range_2.0 && range_1.1 >= range_2.1
        };

        let pair_1 = (pair[0][0], pair[0][1]);
        let pair_2 = (pair[1][0], pair[1][1]);

        // Check if pair_1 contains pair_2 or if pair_2 contains pair_1
        if contains(&pair_1, &pair_2) || contains(&pair_2, &pair_1) {
            return 1;
        } else {
            return 0;
        }
    });

    // Step 3: Get the sum
    let sum: u32 = ranges_contained.sum();

    println!("{}", sum)
}

fn part_2() {
    println!("Part 2!");

    let input = include_str!("input");

    // Step 1: Get each line of the input;
    let ranges = input.lines();

    // Step 2: Get the score from each line;
    let ranges_contained = ranges.map(|pair| {
        // Parse the string
        let pair = pair
            .split(",")
            .map(|sections| {
                sections
                    .split("-")
                    .map(|section_str| section_str.parse::<u32>().unwrap())
                    .take(2)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Simple function that checks if one lap overlaps with another
        // for simplicity sake, assume range_1 is smaller than range_2
        // see min_max to understand what I mean with smaller.
        let overlaps = |range_1: &(u32, u32), range_2: &(u32, u32)| {
            range_1.0 + (range_1.1 - range_1.0) >= range_2.0
        };

        // returns (smaller_range, bigger_range)
        let min_max = |range_1: (u32, u32), range_2: (u32, u32)| {
            if range_1.0 < range_2.0 {
                (range_1, range_2)
            } else if range_1.0 > range_2.0 {
                (range_2, range_1)
            } else {
                if range_1.1 < range_2.1 {
                    (range_1, range_2)
                } else {
                    (range_2, range_1)
                }
            }
        };

        let pair_1 = (pair[0][0], pair[0][1]);
        let pair_2 = (pair[1][0], pair[1][1]);
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

    println!("{}", sum)
}

fn main() {
    part_1();
    part_2();
}

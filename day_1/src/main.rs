fn part_1() {
    println!("Part 1!");
    let input = include_str!("input");
    // Step 1: Get every elf definition
    // Note: Each elf is separated by two line breaks
    // Expects CRLF new lines
    // convert to LF by replacing \r\n to \n
    // Each part is separated by a double line break
    // And each line of each part
    // HAS to contain a number
    let it = input.split("\r\n\r\n");

    // Step 2: get every line of the array as numbers
    let it = it.map(|lines| {
        lines
            .split("\r\n")
            .map(|string_num| u64::from_str_radix(string_num, 10).unwrap_or(0))
    });

    // Step 3: Add all calories from each elf together
    let elves = it.map(|calories| calories.sum::<u64>());

    // Step 4: Find elf with most calories
    println!(
        "The elf with the most calories has: {} calories",
        elves.max().unwrap_or(0)
    );
}

fn part_2() {
    println!("Part 2!");
    // Step 1: Get every elf definition
    // Note: Each elf is separated by two line breaks
    // Expects CRLF new lines
    // convert to LF by replacing \r\n to \n
    // Each part is separated by a double line break
    // And each line of each part is composed
    // HAS to contain a number
    let input = include_str!("input");
    let it = input.split("\r\n\r\n");

    // Step 2: get every line of the array as numbers
    let it = it.map(|lines| {
        lines
            .split("\r\n")
            .map(|string_num| u64::from_str_radix(string_num, 10).unwrap_or(0))
    });

    // Step 3: Add all calories from each elf together
    let elves = it.map(|calories| calories.sum::<u64>());

    // Step 4: Sort elves array
    let mut elves = elves.collect::<Vec<_>>();
    elves.sort();

    // Step 5: Add the three biggest values together
    let total_calories: u64 = elves.iter().rev().take(3).sum();

    println!("The total amount of callories of the top 3 is {total_calories}");
}
fn main() {
    part_1();
    part_2();
}

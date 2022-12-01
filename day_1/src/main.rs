fn main() {
    let input = include_str!("input");
    // Step 1: Get every elf definition
    // Note: Each elf is separated by two line breaks
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

fn part_1(input: &String) -> u64 {
    println!("Part 1!");

    // Step 1: Get every elf definition
    // Note: Each elf is separated by two line breaks
    // Expects CRLF new lines
    // convert to LF by replacing \r\n to \n
    // Each part is separated by a double line break
    // And each line of each part
    // HAS to contain a number
    let new_line = if input.contains("\r\n") { "\r\n" } else { "\n" };

    let double_new_line = new_line.to_owned() + new_line;
    let it = input.split(&double_new_line);

    // Step 2: get every line of the array as numbers
    let it = it.map(|lines| {
        lines
            .split(new_line)
            .map(|string_num| u64::from_str_radix(string_num, 10).unwrap_or(0))
    });

    // Step 3: Add all calories from each elf together
    let elves = it.map(|calories| calories.sum::<u64>());

    // Step 4: Find elf with most calories
    let result = elves.max().unwrap_or(0);
    println!("The elf with the most calories has: {} calories", result);

    return result;
}

fn part_2(input: &String) -> u64 {
    println!("Part 2!");
    // Step 1: Get every elf definition
    // Note: Each elf is separated by two line breaks
    // Expects CRLF new lines
    // convert to LF by replacing \r\n to \n
    // Each part is separated by a double line break
    // And each line of each part is composed
    // HAS to contain a number
    let new_line = if input.contains("\r\n") { "\r\n" } else { "\n" };

    let double_new_line = new_line.to_owned() + new_line;
    let it = input.split(&double_new_line);

    // Step 2: get every line of the array as numbers
    let it = it.map(|lines| {
        lines
            .split(new_line)
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

    return total_calories;
}

mod test {
    #[allow(unused_imports)]
    use crate::{part_1, part_2};

    #[test]
    fn part_1_test() {
        let input = common::get_demo_input().expect("Couldn't read input");

        assert!(part_1(&input) == 24000);
    }

    #[test]
    fn part_2_test() {
        let input = common::get_demo_input().expect("Couldn't read input");

        assert!(part_2(&input) == 45000);
    }
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    part_1(&input);
    part_2(&input);
}

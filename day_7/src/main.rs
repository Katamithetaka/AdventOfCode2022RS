use crate::utils::FileExplorer;

mod test;
mod utils;

fn part_1(input: &String) -> u64 {
    println!("Part 1!");

    let mut file_explorer = FileExplorer::new();

    input.lines().for_each(|line| {
        file_explorer.handle_input(line);
    });

    //file_explorer.print_tree();
    let binding = file_explorer.get_directories();
    let sizes = binding.iter().map(|dir| {
        let dir = dir.borrow();
        (
            dir.get_full_path(),
            dir.get_size(),
            dir.get_files().map(|f| f.len()).unwrap_or(0),
        )
    });

    let sizes = sizes
        .rev()
        .filter_map(|(str, size, _files)| {
            if size <= 100_000 {
                return Some((str, size));
            }
            return None;
        })
        .collect::<Vec<_>>();

    let size = sizes.iter().map(|f| f.1).sum();

    return size;
}

fn part_2(input: &String) -> u64 {
    println!("Part 2!");

    let mut file_explorer = FileExplorer::new();

    input.lines().for_each(|line| {
        file_explorer.handle_input(line);
    });

    //file_explorer.print_tree();
    let binding = file_explorer.get_directories();
    let sizes = binding.iter().map(|dir| {
        let dir = dir.borrow();
        (
            dir.get_full_path(),
            dir.get_size(),
            dir.get_files().map(|f| f.len()).unwrap_or(0),
        )
    });

    let total_used: u64 = file_explorer.get_root_size();
    let required_size = total_used - (70000000 - 30000000);
    println!("{}\n{}\n{}", total_used, required_size, 70000000 - 30000000);
    let size = sizes
        .filter(|(_, size, _)| size >= &required_size)
        .map(|f| f.1)
        .min()
        .unwrap_or(0);

    return size;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

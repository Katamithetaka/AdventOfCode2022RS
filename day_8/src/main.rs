mod test;
mod utils;

pub fn visible_left(trees: &Vec<char>, index: usize) -> bool {
    for i in 0..index {
        if (trees[i] as i32) >= (trees[index] as i32) {
            return false;
        }
    }
    return true;
}

pub fn visible_right(trees: &Vec<char>, index: usize) -> bool {
    for i in (index + 1)..trees.len() {
        if (trees[i] as i32) >= (trees[index] as i32) {
            return false;
        }
    }
    return true;
}

pub fn visible_top(trees: &Vec<Vec<char>>, i: usize, y: usize) -> bool {
    for u in 0..i {
        if (trees[u][y] as i32) >= (trees[i][y] as i32) {
            return false;
        }
    }
    return true;
}

pub fn visible_bottom(trees: &Vec<Vec<char>>, i: usize, y: usize) -> bool {
    for u in (i + 1)..trees.len() {
        if (trees[u][y] as i32) >= (trees[i][y] as i32) {
            return false;
        }
    }
    return true;
}

pub fn visible_trees_left(trees: &Vec<char>, index: usize) -> usize {
    let mut result = 0;
    for i in (0..index).rev() {
        result += 1;
        if (trees[i] as i32) >= (trees[index] as i32) {
            break;
        }
    }
    return result;
}

pub fn visible_trees_right(trees: &Vec<char>, index: usize) -> usize {
    let mut result = 0;
    for i in (index + 1)..trees.len() {
        result += 1;
        if (trees[i] as i32) >= (trees[index] as i32) {
            break;
        }
    }
    return result;
}

pub fn visible_trees_top(trees: &Vec<Vec<char>>, i: usize, y: usize) -> usize {
    let mut result = 0;
    for u in (0..i).rev() {
        result += 1;
        if (trees[u][y] as i32) >= (trees[i][y] as i32) {
            break;
        }
    }
    return result;
}

pub fn visible_trees_bottom(trees: &Vec<Vec<char>>, i: usize, y: usize) -> usize {
    let mut result = 0;
    for u in (i + 1)..trees.len() {
        result += 1;
        if (trees[u][y] as i32) >= (trees[i][y] as i32) {
            break;
        }
    }
    return result;
}

fn part_1(input: &String) -> usize {
    println!("Part 1!");

    let trees = input
        .lines()
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for i in 1..trees.len() - 1 {
        for y in 1..trees[i].len() - 1 {
            if visible_left(&trees[i], y)
                || visible_top(&trees, i, y)
                || visible_right(&trees[i], y)
                || visible_bottom(&trees, i, y)
            {
                count += 1;
            }
        }
    }

    count += trees[0].len() + trees[trees.len() - 1].len() + (trees.len() - 2) + trees.len() - 2;

    println!("{}", count);
    return count;
}

fn part_2(input: &String) -> usize {
    println!("Part 2!");

    let trees = input
        .lines()
        .map(|f| f.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max = 0;

    for i in 1..trees.len() - 1 {
        for y in 1..trees[i].len() - 1 {
            let left = visible_trees_left(&trees[i], y);
            let right = visible_trees_right(&trees[i], y);
            let top = visible_trees_top(&trees, i, y);
            let bottom = visible_trees_bottom(&trees, i, y);
            max = max.max(left * right * bottom * top);
        }
    }

    println!("{}", max);

    return max;
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    part_1(&input);
    part_2(&input);
}

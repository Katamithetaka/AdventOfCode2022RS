mod test;
mod utils;

fn part_1(input: &String) -> usize {
    println!("Part 1!");
    let mut set = std::collections::HashSet::new();
    set.insert((0i32, 0i32));
    let (mut tail_pos, mut head_pos) = ((0i32, 0i32), (0i32, 0i32));

    input.lines().for_each(|f| {
        let (char, count) = f.split_once(' ').unwrap();
        let movement = match char {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Invalid movement for head!"),
        };

        let count = count.parse().unwrap();

        for _ in 0..count {
            head_pos.0 += movement.0;
            head_pos.1 += movement.1;
            let (x, y) = (head_pos.0 - tail_pos.0, head_pos.1 - tail_pos.1);
            let (x2, y2) = (x * x, y * y);
            let distance_squared = x2 + y2;

            if distance_squared >= 4 {
                if x != 0 {
                    tail_pos.0 += if x > 0 { 1 } else { -1 };
                }
                if y != 0 {
                    tail_pos.1 += if y > 0 { 1 } else { -1 };
                }
            }

            set.insert(tail_pos);
        }
    });

    return set.len();
}

fn part_2(input: &String) -> usize {
    println!("Part 2!");
    let mut set = std::collections::HashSet::new();
    set.insert((0i32, 0i32));
    let mut positions = [(0i32, 0i32); 10];

    input.lines().for_each(|f| {
        let (char, count) = f.split_once(' ').unwrap();
        let movement = match char {
            "L" => (-1, 0),
            "R" => (1, 0),
            "U" => (0, -1),
            "D" => (0, 1),
            _ => panic!("Invalid movement for head!"),
        };

        let count = count.parse().unwrap();

        for _ in 0..count {
            positions[0].0 += movement.0;
            positions[0].1 += movement.1;
            for i in 1..positions.len() {
                let (x, y) = (
                    positions[i - 1].0 - positions[i].0,
                    positions[i - 1].1 - positions[i].1,
                );
                let (x2, y2) = (x * x, y * y);
                let distance_squared = x2 + y2;

                if distance_squared >= 4 {
                    if x != 0 {
                        positions[i].0 += if x > 0 { 1 } else { -1 };
                    }
                    if y != 0 {
                        positions[i].1 += if y > 0 { 1 } else { -1 };
                    }
                }
            }

            set.insert(positions[positions.len() - 1]);
        }
    });

    return set.len();
}

fn main() {
    let input = common::get_input().expect("Couldn't read input file");
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}

pub fn get_range(range: &str) -> Option<(u32, u32)> {
    let range = common::split_at(range, "-")?;

    let first_num = range.0.parse::<u32>().ok()?;
    let second_num = range.1.parse::<u32>().ok()?;

    Some((first_num, second_num))
}

pub fn get_ranges(ranges: &str) -> ((u32, u32), (u32, u32)) {
    let ranges =
        common::split_at(ranges, ",").expect("Badly formated, expected num1-num2,num3-num4");

    let first_range = get_range(ranges.0).expect("Badly formated, expected num1-num2");
    let second_range = get_range(ranges.1).expect("Badly formated, expected num3-num4");

    (first_range, second_range)
}

pub fn min_max(range_1: (u32, u32), range_2: (u32, u32)) -> ((u32, u32), (u32, u32)) {
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
}

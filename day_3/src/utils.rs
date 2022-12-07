pub fn get_value(c: i32) -> i32 {
    if c >= { 'a' as i32 } && c <= { 'z' as i32 } {
        return c - { 'a' as i32 } + 1;
    } else if c >= { 'A' as i32 } && c <= { 'Z' as i32 } {
        return c - { 'A' as i32 } + 27;
    } else {
        panic!("Invalid input: Expected character in range a-z or A-Z");
    }
}

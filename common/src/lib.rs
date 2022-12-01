// This file will be used if I find myself needing 
// the same functionalities for multiple 
// advent of codes
// like utilities to read input etc
// Reading input can be done excessively easily in rust
// therefore I won't need to add this one.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

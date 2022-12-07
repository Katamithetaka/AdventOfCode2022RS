// This file will be used if I find myself needing
// the same functionalities for multiple
// advent of codes
// like utilities to read input etc
// Reading input can be done excessively easily in rust
// therefore I won't need to add this one.

pub fn at_index<T>(vec: &Vec<T>, index: usize) -> Option<&T> {
    if index < vec.len() {
        return Some(&vec[index]);
    } else {
        return None;
    }
}

use std::{
    env::{args, var},
    fs,
};

pub fn get_input<'a>() -> Option<String> {
    let args = args().collect::<Vec<_>>();
    let demo = at_index(&args, 1)
        .map(|f| f.clone())
        .unwrap_or(
            var("DEMO")
                .map(|_| "DEMO".to_string())
                .unwrap_or("0".to_string()),
        )
        .to_uppercase();

    if &demo == "DEMO" {
        fs::read_to_string("demo_input").ok()
    } else {
        fs::read_to_string("input").ok()
    }
}

pub fn get_demo_input() -> Option<String> {
    fs::read_to_string("demo_input").ok()
}

pub fn split_at<'a>(str: &'a str, pattern: &str) -> Option<(&'a str, &'a str)> {
    str.split_once(pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Running this test from the IDE adds a command line argument therefore it won't run properly
    // and will crash
    // use cargo test to test it.
    #[test]
    fn get_input_works() {
        assert!(get_input() == fs::read_to_string("input").ok());
        std::env::set_var("DEMO", "1");
        assert!(get_input() == fs::read_to_string("demo_input").ok());
    }

    #[test]
    fn at_index_works() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert!(at_index(&vec, 0).is_some());
        assert!(at_index(&vec, vec.len()).is_none());
        assert!(at_index(&vec, 1) == Some(&vec[1]));
    }

    #[test]
    fn split_at_works() {
        let string = "ABC\r\nDEF\r\nHIJ";
        assert!(split_at(string, "zed").is_none());
        assert!(split_at(string, "\r\n").is_some());
        let result = split_at(string, "\r\n");
        assert!(result == Some((&"ABC", "DEF\r\nHIJ")));
        let result = result.map(|(l, r)| (l, split_at(r, "\r\n")));
        assert!(result == Some((&"ABC", Some((&"DEF", &"HIJ")))));
    }
}

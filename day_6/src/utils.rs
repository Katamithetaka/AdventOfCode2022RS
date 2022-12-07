pub fn contains(v: &Vec<char>, index: usize) -> bool {
    for i in 0..v.len() {
        if i == index {
            continue;
        }

        if v[i] == v[index] {
            return true;
        }
    }
    return false;
}

pub fn unique(v: &Vec<char>) -> bool {
    for i in 0..v.len() {
        if contains(&v, i) {
            return false;
        }
    }
    return true;
}

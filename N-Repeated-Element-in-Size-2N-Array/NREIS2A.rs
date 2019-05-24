use std::collections::HashSet;

pub fn repeated_n_times(a: Vec<i32>) -> i32 {
    let mut set = HashSet::new();
    for i in a {
        if !set.insert(i) {
            return i;
        }
    }
    0
}

fn main() {
    println!("{:?}", repeated_n_times(vec![1, 2, 3, 3]));
}

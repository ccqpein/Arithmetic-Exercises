use std::collections::{HashMap, HashSet};

pub fn longest_substring(s: String, k: i32) -> i32 {
    let set = s.chars().into_iter().collect::<HashSet<char>>();
    let mut hash: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *hash.entry(c).or_insert(0) += 1;
    }

    for c in set {
        if hash.get(&c).unwrap() < &k {
            let temp = s
                .split(|ch| ch == c)
                .map(|e| longest_substring(e.to_string(), k))
                .max()
                .unwrap();

            return temp;
        }
    }
    s.bytes().count() as i32
}

fn main() {
    println!("{}", longest_substring(String::from("ababbc"), 2)); //=> 5
    println!("{}", longest_substring(String::from("aaabb"), 3)); //=> 3
    println!("{}", longest_substring(String::from("aaabbb"), 3)); //=> 6
}

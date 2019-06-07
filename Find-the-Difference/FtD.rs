use std::collections::HashSet;

fn find_diff(s: String, t: String) -> String {
    let mut set: HashSet<&u8> = HashSet::new();
    for u in s.as_bytes() {
        set.insert(u);
    }

    for u in t.as_bytes() {
        if !set.contains(u) {
            return String::from_utf8(vec![*u]).unwrap();
        }
    }

    String::new()
}

fn main() {
    let s = String::from("abcd");
    let t = String::from("abcde");

    println!("{:?}", find_diff(s, t));
}

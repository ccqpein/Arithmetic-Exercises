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

pub fn find_the_difference(s: String, t: String) -> char {
    use std::collections::HashMap;
    let mut table_s: HashMap<char, u32> = HashMap::new();
    let mut table_t: HashMap<char, u32> = HashMap::new();

    let t_chars = t.chars();
    let s_chars = s.chars();

    for u in s_chars {
        *table_s.entry(u).or_insert(0) += 1;
    }

    for u in t_chars.clone() {
        *table_t.entry(u).or_insert(0) += 1;
    }

    for u in t_chars {
        let temp = table_s.get(&u);
        if temp.is_none() {
            return u;
        } else if temp.unwrap() != table_t.get(&u).unwrap() {
            return u;
        }
    }
    ' '
}

fn main() {
    let s = String::from("abcd");
    let t = String::from("abcde");

    //println!("{:?}", find_diff(s, t));
    println!("{:?}", find_the_difference(s, t));
}

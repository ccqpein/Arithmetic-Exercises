pub fn equal_frequency(word: String) -> bool {
    use std::collections::HashMap;
    let mut bucket = [0; 26];
    for b in word.bytes() {
        bucket[(b - b'a') as usize] += 1
    }

    let mut table = HashMap::new();

    for e in bucket {
        if e != 0 {
            *table.entry(e).or_insert(0) += 1
        }
    }

    if table.keys().len() == 1 {
        if table.get(&1).is_some() {
            return true;
        } else if *table.get(table.keys().nth(0).unwrap()).unwrap() == 1 {
            return true;
        }
    }

    if table.keys().len() != 2 {
        return false;
    }

    if let Some(n) = table.get(&1) {
        if *n == 1 {
            return true;
        }
    }

    let mut sorted_keys = table.keys().collect::<Vec<_>>();
    sorted_keys.sort();

    if sorted_keys[1] - sorted_keys[0] == 1 && *table.get(sorted_keys[1]).unwrap() == 1 {
        return true;
    }

    false
}

fn main() {
    assert_eq!(equal_frequency(String::from("abcc")), true);
    assert_eq!(equal_frequency(String::from("aazz")), false);
    assert_eq!(equal_frequency(String::from("bac")), true);
    assert_eq!(equal_frequency(String::from("zz")), true);
}

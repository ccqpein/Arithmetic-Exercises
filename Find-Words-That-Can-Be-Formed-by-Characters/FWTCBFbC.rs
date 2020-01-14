pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    use std::collections::{HashMap, HashSet};
    let char_set = {
        let mut char_table = HashMap::new();
        for c in chars.chars() {
            *char_table.entry(c).or_insert(0) += 1
        }
        char_table
    };
    let words_sets = words
        .iter()
        .map(|w| {
            let mut char_table = HashMap::new();
            for c in w.chars() {
                *char_table.entry(c).or_insert(0) += 1
            }
            char_table
        })
        .collect::<Vec<_>>();

    dbg!(&char_set);
    dbg!(&words_sets);

    let mut result = 0;
    for i in 0..words_sets.len() {
        if words[i].len() > chars.len() {
            continue;
        }

        let mut flag = true;
        for (k, v) in &words_sets[i] {
            if let Some(n) = char_set.get(k) {
                if n < v {
                    flag = false;
                    break;
                }
            } else {
                flag = false;
                break;
            }
        }
        if flag {
            result += words[i].len()
        }
    }

    result as i32
}

fn main() {
    dbg!(count_characters(
        vec![
            "cat".to_string(),
            "bt".to_string(),
            "hat".to_string(),
            "tree".to_string()
        ],
        "atach".to_string()
    ));

    dbg!(count_characters(
        vec![
            "hello".to_string(),
            "world".to_string(),
            "leetcode".to_string(),
        ],
        "welldonehoneyr".to_string()
    ));
}

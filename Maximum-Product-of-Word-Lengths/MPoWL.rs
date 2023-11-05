//use std::collections::HashSet;

pub fn max_product(words: Vec<String>) -> i32 {
    use std::collections::HashSet;
    let words_set = words
        .iter()
        .map(|w| w.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let mut most = 0;
    for i in 0..words_set.len() {
        for j in i + 1..words_set.len() {
            if words_set[i].intersection(&words_set[j]).count() == 0 {
                if words[i].len() * words[j].len() > most {
                    most = words[i].len() * words[j].len();
                }
            }
        }
    }

    most as i32
}

fn main() {
    dbg!(max_product(vec![
        "abcw".to_string(),
        "baz".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "xtfn".to_string(),
        "abcdef".to_string()
    ]));

    dbg!(max_product(vec![
        "a".to_string(),
        "ab".to_string(),
        "abc".to_string(),
        "d".to_string(),
        "cd".to_string(),
        "bcd".to_string(),
        "abcd".to_string(),
    ]));
}

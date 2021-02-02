use std::collections::HashMap;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut table = HashMap::new();
    helper(&s, 0, &word_dict, &mut table)
}

fn helper(s: &str, this: usize, dict: &[String], table: &mut HashMap<usize, bool>) -> bool {
    if s == "" {
        return true;
    }

    if let Some(_) = table.get(&this) {
        return false;
    }

    for d in dict {
        if s.starts_with(d) {
            if helper(s.split_at(d.len()).1, d.len() + this, dict, table) {
                return true;
            }
        }
    }
    table.insert(this, false);
    false
}

fn main() {
    assert_eq!(
        word_break(
            String::from("leetcode"),
            vec!["leet".to_string(), "code".to_string()]
        ),
        true
    );

    assert_eq!(
        word_break(
            String::from("applepenapple"),
            vec!["apple".to_string(), "pen".to_string()]
        ),
        true
    );

    assert_eq!(
        word_break(
            String::from("catsandog"),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ),
        false
    );

    assert_eq!(
        word_break(
            String::from("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab"),
            ["a","aa","aaa","aaaa","aaaaa","aaaaaa","aaaaaaa","aaaaaaaa","aaaaaaaaa","aaaaaaaaaa"].iter().map(|s| s.to_string()).collect()
        ),
        false
    );
}

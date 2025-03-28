pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
    words
        .into_iter()
        .filter_map(|w| if w.starts_with(&pref) { Some(1) } else { None })
        .sum()
}

fn main() {}

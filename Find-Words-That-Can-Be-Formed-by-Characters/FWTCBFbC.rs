pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
    let cc = to_bytes(chars);
    words
        .into_iter()
        .filter_map(|w| {
            let ll = w.len();
            let ww = to_bytes(w);
            if match_vec(&ww, &cc) {
                Some(ll as i32)
            } else {
                None
            }
        })
        .sum()
}

fn to_bytes(s: String) -> [u8; 26] {
    let mut result = [0; 26];
    for x in s.bytes() {
        result[(x - 97) as usize] += 1
    }

    result
}

fn match_vec(word: &[u8; 26], chars: &[u8; 26]) -> bool {
    (0..26).into_iter().all(|i| word[i] <= chars[i])
}

fn main() {
    assert_eq!(
        count_characters(
            vec!["cat", "bt", "hat", "tree"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            "atach".to_string()
        ),
        6
    );

    assert_eq!(
        count_characters(
            vec!["hello", "world", "leetcode"]
                .into_iter()
                .map(|s| s.to_string())
                .collect(),
            "welldonehoneyr".to_string()
        ),
        10
    );
}

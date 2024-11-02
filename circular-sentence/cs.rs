pub fn is_circular_sentence(sentence: String) -> bool {
    let sentenct: Vec<char> = sentence.chars().collect();
    for (ind, c) in sentenct.iter().enumerate() {
        if *c == ' ' {
            if sentenct[ind - 1] != sentenct[ind + 1] {
                return false;
            }
        }
    }
    true && sentenct.last().unwrap_or(&'a') == sentenct.first().unwrap_or(&'b')
}

fn main() {
    assert!(!is_circular_sentence("leetcode".to_string()));
    assert!(is_circular_sentence("eetcode".to_string()));
    assert!(!is_circular_sentence("Leetcode eisc cool".to_string()));
}

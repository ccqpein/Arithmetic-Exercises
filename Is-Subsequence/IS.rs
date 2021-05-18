pub fn is_subsequence(s: String, t: String) -> bool {
    inner(s.as_bytes(), t.as_bytes())
}

fn inner(s: &[u8], t: &[u8]) -> bool {
    match (s.len(), t.len()) {
        (0, _) => true,
        (_, 0) => false,
        _ if s[0] == t[0] => inner(&s[1..], &t[1..]),
        _ => inner(&s[..], &t[1..]),
    }
}

fn main() {
    assert!(is_subsequence(String::from("abc"), String::from("ahbgdc")));
    assert!(!is_subsequence(String::from("axc"), String::from("ahbgdc")));
}

pub fn repeated_substring_pattern(s: String) -> bool {
    for length in 1..s.len() / 2 + 1 {
        if s.len() % length == 0 {
            if helper(&s, length, s.len() / length) {
                return true;
            }
        }
    }
    return false;
}

fn helper(s: &str, length: usize, count: usize) -> bool {
    let a = &s[0..length];
    for i in 1..count {
        if s[length * i..length * i + length] != *a {
            return false;
        }
    }

    true
}

fn main() {
    dbg!(helper("abab", 2, 2));

    dbg!(repeated_substring_pattern("abab".into()));
    dbg!(repeated_substring_pattern("aba".into()));
    dbg!(repeated_substring_pattern("abcabcabcabc".into()));

    assert!(repeated_substring_pattern("bb".into()));
}

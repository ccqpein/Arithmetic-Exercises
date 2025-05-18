pub fn remove_palindrome_sub(s: String) -> i32 {
    let mut rs: Vec<_> = s.chars().rev().collect();
    let sc: Vec<char> = s.chars().collect();

    for ind in 0..(rs.len() / 2) {
        if rs[ind] != sc[ind] {
            return 2;
        }
    }

    return 1;
}

fn main() {
    assert_eq!(1, remove_palindrome_sub("ababa".to_string()));
    assert_eq!(2, remove_palindrome_sub("abb".to_string()));
    assert_eq!(2, remove_palindrome_sub("baabb".to_string()));
}

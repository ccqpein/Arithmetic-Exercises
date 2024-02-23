pub fn reverse_string(s: &mut Vec<char>) {
    let mut char_b;
    let mut i = 0;
    let len = s.len();
    loop {
        if i >= len - 1 - i {
            break;
        }
        char_b = s[i];
        s[i] = s[len - 1 - i];
        s[len - 1 - i] = char_b;
        i += 1
    }
}

fn main() {
    let mut case0 = "hello".chars().collect();
    reverse_string(&mut case0);
    assert_eq!("olleh".chars().collect::<Vec<_>>(), case0);

    let mut case0 = "Hannah".chars().collect();
    reverse_string(&mut case0);
    assert_eq!("hannaH".chars().collect::<Vec<_>>(), case0);

    let mut case0 = "\"".chars().collect();
    reverse_string(&mut case0);
    assert_eq!("\"".chars().collect::<Vec<_>>(), case0);
}

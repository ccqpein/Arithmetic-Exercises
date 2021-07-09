use std::iter::FromIterator;

pub fn truncate_sentence(s: String, mut k: i32) -> String {
    let mut cc: Vec<char> = vec![];
    for c in s.chars() {
        if c == ' ' {
            k -= 1;
        }
        if k == 0 {
            break;
        }
        cc.push(c);
    }

    String::from_iter(cc)
}

fn main() {
    assert_eq!(
        truncate_sentence("Hello how are you Contestant".to_string(), 4),
        "Hello how are you".to_string()
    );

    assert_eq!(
        truncate_sentence("What is the solution to this problem".to_string(), 4),
        "What is the solution".to_string()
    );

    assert_eq!(
        truncate_sentence("chopper is not a tanuki".to_string(), 5),
        "chopper is not a tanuki".to_string()
    );
}

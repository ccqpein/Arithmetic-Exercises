use std::str::Chars;

pub fn is_valid(s: String) -> bool {
    helper(s.chars(), &mut vec![])
}

fn helper(mut ch: Chars, stack: &mut Vec<char>) -> bool {
    //println!("chars: {:?}, stack: {:?}", ch, stack);
    match ch.next() {
        Some(c) => {
            if c == 'a' {
                stack.push('c');
                stack.push('b');
                helper(ch, stack)
            } else if c == *stack.last().unwrap_or(&' ') {
                stack.pop();
                helper(ch, stack)
            } else {
                false
            }
        }
        None => stack.len() == 0,
    }
}

fn main() {
    assert!(is_valid("aabcbc".to_string()));
    assert!(is_valid("aabcbcabcabcababcc".to_string()));
    assert!(!is_valid("abccba".to_string()));
    assert!(!is_valid("aaabc".to_string()));
}

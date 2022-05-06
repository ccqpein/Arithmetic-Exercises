pub fn is_valid(s: String) -> bool {
    let mut buffer = Vec::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' | '[' | '{' => buffer.push(c),
            ')' => {
                if buffer.last().unwrap_or(&' ') == &'(' {
                    buffer.pop();
                } else {
                    return false;
                }
            }
            ']' => {
                if buffer.last().unwrap_or(&' ') == &'[' {
                    buffer.pop();
                } else {
                    return false;
                }
            }
            '}' => {
                if buffer.last().unwrap_or(&' ') == &'{' {
                    buffer.pop();
                } else {
                    return false;
                }
            }
            _ => unreachable!(),
        }
    }
    true && buffer.is_empty()
}

fn main() {
    assert!(is_valid("()".to_string()));
    assert!(is_valid("()[]{}".to_string()));
    assert!(!is_valid("(]".to_string()));
    assert!(!is_valid("(".to_string()));
}

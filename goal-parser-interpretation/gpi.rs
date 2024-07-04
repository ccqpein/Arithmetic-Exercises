pub fn interpret(command: String) -> String {
    let mut flag = false;
    let mut result = Vec::with_capacity(command.len());
    for c in command.into_bytes() {
        match c {
            b'(' => {
                flag = true;
            }
            b')' => {
                if flag {
                    result.push(b'o')
                };
                flag = false
            }
            _ => {
                result.push(c);
                flag = false;
            }
        }
    }

    String::from_utf8(result).unwrap()
}

fn main() {
    assert_eq!(interpret("G()(al)".to_string()), "Goal".to_string());
    assert_eq!(
        interpret("G()()()()(al)".to_string()),
        "Gooooal".to_string()
    );
    assert_eq!(
        interpret("(al)G(al)()()G".to_string()),
        "alGalooG".to_string()
    );
}

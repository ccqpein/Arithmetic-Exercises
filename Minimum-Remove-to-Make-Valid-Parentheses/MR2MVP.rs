// pub fn min_remove_to_make_valid(s: String) -> String {
//     use std::iter::FromIterator;
//     let mut stack = vec![];
//     let mut result = vec![];
//     for c in s.chars() {
//         match c {
//             '(' => {
//                 stack.push(c);
//                 result.push(c);
//             }
//             ')' => {
//                 if stack.len() != 0 {
//                     stack.pop();
//                     result.push(c);
//                 }
//             }
//             _ => {
//                 result.push(c);
//             }
//         }
//     }
//     String::from_iter(result)
// }

pub fn min_remove_to_make_valid(s: String) -> String {
    use std::collections::VecDeque;
    use std::iter::FromIterator;
    let mut head = vec![];
    let mut tail = vec![];

    for (ind, c) in s.chars().enumerate() {
        match c {
            '(' => head.push(ind),
            ')' => {
                if head.len() != 0 {
                    head.pop();
                } else {
                    tail.push(ind);
                };
            }
            _ => (),
        }
    }

    let mut rmv = {
        head.append(&mut tail);
        head.sort();
        VecDeque::from(head)
    };
    let mut result = vec![];
    //dbg!(&rmv);
    for (ind, c) in s.chars().enumerate() {
        if let Some(i) = rmv.get(0) {
            if *i == ind {
                rmv.pop_front();
            } else {
                result.push(c)
            }
        } else {
            result.push(c)
        }
    }
    String::from_iter(result)
}

fn main() {
    assert_eq!(
        min_remove_to_make_valid(String::from("lee(t(c)o)de)")),
        "lee(t(c)o)de".to_string()
    );
    assert_eq!(
        min_remove_to_make_valid(String::from("a)b(c)d")),
        "ab(c)d".to_string()
    );
    assert_eq!(
        min_remove_to_make_valid(String::from("))((")),
        "".to_string()
    );
    assert_eq!(
        min_remove_to_make_valid(String::from("(a(b(c)d)")),
        "a(b(c)d)".to_string()
    );
}

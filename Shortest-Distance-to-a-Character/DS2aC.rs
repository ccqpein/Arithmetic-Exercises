pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
    let length = s.len();
    let mut from_head = Vec::with_capacity(length);
    let mut from_tail = Vec::with_capacity(length);

    let mut current = length as i32;
    let mut flag = -1;
    for ch in s.chars() {
        if c == ch {
            flag = 1;
            current = 0;
            from_head.push(current);
        } else {
            current += flag;
            from_head.push(current);
        }
    }

    let mut current = length as i32;
    let mut flag = -1;
    for ch in s.chars().rev() {
        if c == ch {
            flag = 1;
            current = 0;
            from_tail.push(current);
        } else {
            current += flag;
            from_tail.push(current);
        }
    }

    from_tail.reverse();
    //dbg!(&from_head);
    //dbg!(&from_tail);
    from_head
        .into_iter()
        .zip(from_tail)
        .map(|(a, b)| a.min(b))
        .collect()
}

fn main() {
    assert_eq!(
        shortest_to_char("loveleetcode".into(), 'e'),
        vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0]
    );

    assert_eq!(shortest_to_char("aaab".into(), 'b'), vec![3, 2, 1, 0]);
}

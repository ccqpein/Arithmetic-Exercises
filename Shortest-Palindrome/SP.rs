// pub fn shortest_palindrome(s: String) -> String {
//     if s.len() == 0 {
//         return String::new();
//     } else if s.len() == 1 {
//         return s;
//     }

//     let border = if s.len() % 2 == 0 {
//         s.len() / 2 - 1
//     } else {
//         s.len() / 2
//     };

//     let s = s.as_bytes();
//     let mut middle_point = 1 as usize;
//     let mut left: Option<Vec<_>> = None;
//     let mut try_again = false; // flag for jump one next step
//     while middle_point <= border {
//         let (mut tail_a, mut tail_b) = if s[middle_point + 1] == s[middle_point] && !try_again {
//             try_again = true;
//             (middle_point + 2, 2 * middle_point + 2)
//         } else {
//             try_again = false;
//             (middle_point + 1, 2 * middle_point + 1)
//         };

//         // when beyond the range of s
//         if tail_b > s.len() {
//             tail_a -= 1;
//             tail_b -= 1;
//             try_again = false;
//         }

//         let tail = s[tail_a..tail_b].iter().rev();
//         let head = s[0..middle_point].iter();

//         if tail.eq(head) {
//             left = Some(s[tail_b..].iter().rev().cloned().collect::<Vec<u8>>());
//             if try_again {
//                 try_again = false
//             }
//         }

//         if !try_again {
//             middle_point += 1;
//         }
//     }

//     if left.is_none() {
//         if s[0] == s[1] {
//             left = Some(s[2..].iter().rev().cloned().collect::<Vec<u8>>())
//         } else {
//             left = Some(s[1..].iter().rev().cloned().collect::<Vec<u8>>())
//         };
//     }

//     let mut a = left.unwrap();
//     a.extend(s.iter());

//     String::from_utf8(a).unwrap()
// }

// even slower......
pub fn shortest_palindrome(s: String) -> String {
    if s.len() == 0 {
        return String::new();
    }

    let mut stack = vec![];
    let s = s.as_bytes();
    let mut tail = 0 as usize;
    for (ind, b) in s.iter().enumerate() {
        stack.push(*b);
        if check_palindrome(&stack, ind + 1) {
            tail = ind;
        }
    }

    let mut a = if tail == s.len() - 1 {
        vec![]
    } else {
        s[tail + 1..].iter().rev().cloned().collect::<Vec<u8>>()
    };
    a.extend(s);
    String::from_utf8(a).unwrap()
}

fn check_palindrome(s: &Vec<u8>, len: usize) -> bool {
    let (first, second) = if len % 2 == 0 {
        s.split_at(len / 2)
    } else {
        let (a, b) = s.split_at(len / 2);
        (a, &b[1..])
    };

    second.iter().rev().collect::<Vec<&u8>>() == first.iter().map(|x| x).collect::<Vec<&u8>>()
}

fn main() {
    assert_eq!(shortest_palindrome(String::from("aacecaaa")), "aaacecaaa");
    assert_eq!(shortest_palindrome(String::from("abcd")), "dcbabcd");
    assert_eq!(shortest_palindrome(String::from("")), "");
    assert_eq!(shortest_palindrome(String::from("aba")), "aba");
    assert_eq!(shortest_palindrome(String::from("abbacd")), "dcabbacd");
    assert_eq!(shortest_palindrome(String::from("abb")), "bbabb");
    assert_eq!(shortest_palindrome(String::from("aaaaa")), "aaaaa");
    assert_eq!(shortest_palindrome(String::from("aabba")), "abbaabba");
    assert_eq!(shortest_palindrome(String::from("a")), "a");
    assert_eq!(
        shortest_palindrome(String::from("babbbabbaba")),
        "ababbabbbabbaba"
    );
    assert_eq!(
        shortest_palindrome(String::from("aaaabbaa")),
        "aabbaaaabbaa"
    );
}

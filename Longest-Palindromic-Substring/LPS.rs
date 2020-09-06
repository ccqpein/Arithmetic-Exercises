pub fn longest_palindrome(s: String) -> String {
    let mut s = s.as_bytes().to_vec();
    let mut d = s.clone();
    d.reverse();

    let mut largest_len = 0;
    let mut result = String::new();
    loop {
        if s.len() == 0 || s.len() < largest_len {
            break;
        }

        let (a, b) = inner_loop(&s, &d);
        if a > largest_len {
            largest_len = a;
            result = b
        }

        s = s.drain(1..).collect();
        d.pop();
    }

    result
}

/// find palindrome, return length, start index
fn inner_loop(s: &Vec<u8>, d: &Vec<u8>) -> (usize, String) {
    //println!("here: {:?}, {:?}", s, d);
    let mut length = s.len();
    let mut offset = 0;
    let mut flag = true;
    loop {
        flag = true;
        if length == 0 {
            break;
        }

        for ind in 0..(length / 2 + 1) {
            if s[ind] != d[ind + offset] {
                flag = false;
                break;
            }
        }

        if flag {
            return (length, String::from_utf8(s[0..length].to_vec()).unwrap());
        }

        length -= 1;
        offset += 1;
    }
    (0, String::new())
}

/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////

pub fn longest_palindrome2(s: String) -> String {
    let ss = s.as_bytes();
    let length = s.len();
    if length <= 1 {
        return s;
    }
    let (mut record_start, mut largest_size) = (0, 0);
    for start in 0..length - 1 {
        if length - start < largest_size {
            break;
        }
        for end in (start + 1..length).rev() {
            if end - start + 1 < largest_size {
                break;
            }

            let (b, size) = inner_loop2(ss, start, end);
            if b {
                if size.unwrap() > largest_size {
                    largest_size = size.unwrap();
                    record_start = start;
                }
            }
        }
    }

    //println!("{}, {}", record_start, largest_size);
    String::from_utf8(ss[record_start..record_start + largest_size + 1].to_vec()).unwrap()
}

pub fn inner_loop2(s: &[u8], start: usize, end: usize) -> (bool, Option<usize>) {
    //println!("here {},{}", start, end);
    if end - start == 0 {
        return (true, Some(end - start));
    }
    for i in 0..(end - start) {
        if s[start + i] != s[end - i] {
            return (false, None);
        }
    }
    (true, Some(end - start))
}

fn main() {
    assert_eq!("bab", longest_palindrome("babad".to_string()));
    assert_eq!("bb", longest_palindrome("cbbd".to_string()));
    assert_eq!("", longest_palindrome("".to_string()));

    assert_eq!("bab", longest_palindrome2("babad".to_string()));
    assert_eq!("bb", longest_palindrome2("cbbd".to_string()));
    assert_eq!("", longest_palindrome2("".to_string()));
}

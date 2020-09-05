pub fn longest_palindrome(s: String) -> String {
    let mut s = s.as_bytes().to_vec();
    let mut d = s.clone();
    d.reverse();

    let mut largest_len = 0;
    let mut result = String::new();
    loop {
        if s.len() == 0 {
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

fn main() {
    println!("{}", longest_palindrome("babad".to_string()));
    println!("{}", longest_palindrome("cbbd".to_string()));
}

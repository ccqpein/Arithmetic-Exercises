use std::io::{self, BufRead};

pub fn compare_version(version1: String, version2: String) -> i32 {
    let mut cursor1 = io::Cursor::new(version1 + ".");
    let mut cursor2 = io::Cursor::new(version2 + ".");

    let mut buf1 = vec![];
    let mut buf2 = vec![];
    let mut first = true;

    loop {
        let l1 = cursor1.read_until(b'.', &mut buf1).unwrap();
        let l2 = cursor2.read_until(b'.', &mut buf2).unwrap();

        if l1 == 0 && l2 == 0 {
            return 0;
        }

        if l1 == 0 {
            buf1.push(b'0');
            buf1.push(b'.');
        }
        if l2 == 0 {
            buf2.push(b'0');
            buf2.push(b'.');
        }

        //dbg!(&buf1);
        //dbg!(&buf2);
        let len1 = buf1.len();
        let len2 = buf2.len();

        let result = compare(&buf1[..len1 - 1], &buf2[..len2 - 1], first);

        if result != 0 {
            return result;
        }

        buf1.clear();
        buf2.clear();
        first = false
    }
}

fn compare(mut a: &[u8], mut b: &[u8], first: bool) -> i32 {
    //if first {
    while let [first, rest @ ..] = a {
        if *first == b'0' {
            a = rest;
        } else {
            break;
        }
    }

    while let [first, rest @ ..] = b {
        if *first == b'0' {
            b = rest;
        } else {
            break;
        }
    }
    //}

    if a.len() == 0 {
        a = &[b'0']
    }

    if b.len() == 0 {
        b = &[b'0']
    }

    // dbg!(&a);
    // dbg!(&b);

    let aa = String::from_utf8(a.to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let bb = String::from_utf8(b.to_vec())
        .unwrap()
        .parse::<usize>()
        .unwrap();

    if aa < bb {
        return -1;
    } else if aa == bb {
        return 0;
    } else {
        return 1;
    }
}

fn main() {
    assert_eq!(0, compare_version("1.01".to_string(), "1.001".to_string()));
    assert_eq!(0, compare_version("1.0".to_string(), "1.0.0".to_string()));
    assert_eq!(-1, compare_version("0.1".to_string(), "1.1".to_string()));
    assert_eq!(-1, compare_version("1.1".to_string(), "1.10".to_string()));
}

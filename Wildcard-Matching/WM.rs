pub fn is_match(s: String, p: String) -> bool {
    //rec_match(s.as_bytes(), p.as_bytes())
    println!("{}, {}", s, p);
    no_rec_match(s.as_bytes(), p.as_bytes())
}

fn rec_match(s: &[u8], p: &[u8]) -> bool {
    if s.is_empty() && p.is_empty() {
        return true;
    }

    if p.is_empty() && !s.is_empty() {
        return false;
    }

    if !p.is_empty() && s.is_empty() && p[0] != b'*' {
        return false;
    }

    match p[0] {
        b'*' => {
            let mut pos = 1;
            {
                for i in 0..p.len() {
                    if p[i] != b'*' {
                        pos = i;
                        break;
                    };
                }
            }
            //dbg!(pos);
            for i in (0..s.len() + 1).rev() {
                let (_, right) = s.split_at(i);
                if rec_match(right, &p[pos..]) {
                    return true;
                }
            }
            return false;
        }
        b'?' => rec_match(&s[1..], &p[1..]),
        _ => {
            if p[0] != s[0] {
                return false;
            } else {
                rec_match(&s[1..], &p[1..])
            }
        }
    }
}

// fast version
fn no_rec_match(s: &[u8], p: &[u8]) -> bool {
    let (mut i, mut j) = (0, 0);
    let (mut last_match, mut start) = (0, -1);

    while i < s.len() {
        println!(
            "i: {}, j: {}, last_match: {}, start: {}",
            i, j, last_match, start
        );
        if j < p.len() && (p[j] == b'?' || s[i] == p[j]) {
            i += 1;
            j += 1;
        } else if j < p.len() && p[j] == b'*' {
            start = j as i32;
            last_match = i;
            j += 1;
        } else if start != -1 {
            j = start as usize + 1;
            last_match += 1;
            i = last_match;
        } else {
            return false;
        }
    }
    println!(
        "finally: i: {}, j: {}, last_match: {}, start: {}",
        i, j, last_match, start
    );
    while j < p.len() && p[j] == b'*' {
        j += 1;
    }

    j == p.len()
}

fn main() {
    //assert!(is_match("".to_string(), "".to_string()));
    //assert!(is_match("".to_string(), "*".to_string()));

    assert!(!is_match("aa".to_string(), "a".to_string()));
    assert!(is_match("aa".to_string(), "*".to_string()));
    assert!(!is_match("cb".to_string(), "?a".to_string()));
    assert!(is_match("adceb".to_string(), "*a*b".to_string()));
    //assert!(!is_match("acdcb".to_string(), "a*c?b".to_string()));

    // assert!(!is_match(
    //     "mississippi".to_string(),
    //     "m??*ss*?i*pi".to_string()
    // ));

    // assert!(!is_match(
    //     "aaabbbaabaaaaababaabaaabbabbbbbbbbaabababbabbbaaaaba".to_string(),
    //     "a*******b".to_string()
    // ));

    // assert!(!is_match(
    //         "abbabaaabbabbaababbabbbbbabbbabbbabaaaaababababbbabababaabbababaabbbbbbaaaabababbbaabbbbaabbbbababababbaabbaababaabbbababababbbbaaabbbbbabaaaabbababbbbaababaabbababbbbbababbbabaaaaaaaabbbbbaabaaababaaaabb".to_string(),
    //         "**aa*****ba*a*bb**aa*ab****a*aaaaaa***a*aaaa**bbabb*b*b**aaaaaaaaa*a********ba*bbb***a*ba*bb*bb**a*b*bb"
    // .to_string()
    //     ));
}

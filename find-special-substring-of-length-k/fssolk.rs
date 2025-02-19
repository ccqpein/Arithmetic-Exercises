pub fn has_special_substring(s: String, k: i32) -> bool {
    use std::collections::VecDeque;
    let mut start = None;
    let mut s = s.chars().collect::<VecDeque<char>>();
    let k = k as usize;

    loop {
        let a = s.get(0).unwrap();
        let c = s.get(k);
        //dbg!(&a);
        //dbg!(&c);
        if s.range(0..k).all(|c| c == a) {
            //dbg!(&start);
            //dbg!(&c);
            match (start, c) {
                (None, None) => return true,
                (None, Some(cc)) => {
                    if a != cc {
                        return true;
                    }
                }
                (Some(aa), None) => {
                    if *a != aa {
                        return true;
                    }
                }
                (Some(aa), Some(cc)) => {
                    if *a != aa && a != cc {
                        return true;
                    }
                }
            }
        }

        if c.is_none() {
            break;
        }
        start = Some(*a);
        s.pop_front();
    }
    false
}

fn main() {
    assert!(has_special_substring("aaabaaa".to_string(), 3));
    assert!(!has_special_substring("abc".to_string(), 2));
    assert!(has_special_substring("aaa".to_string(), 3));
}

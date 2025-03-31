pub fn make_fancy_string_before(s: String) -> String {
    if s == String::new() {
        return String::new();
    };

    let mut result = vec![];
    let mut s = s.chars();
    let mut last = s.next().unwrap();
    let mut count = 1;
    for c in s {
        if c != last {
            if count <= 2 {
                result.append(&mut vec![last; count]);
            } else {
                result.append(&mut vec![last; 2]);
            }
            last = c;
            count = 1;
        } else {
            count += 1
        }
    }

    if count <= 2 {
        result.append(&mut vec![last; count]);
    } else {
        result.append(&mut vec![last; 2]);
    }

    String::from_iter(result)
}

pub fn make_fancy_string(s: String) -> String {
    if s == String::new() {
        return String::new();
    };

    let mut result = vec![];
    let mut s = s.chars();
    let mut last = s.next().unwrap();
    let mut count = 1;
    let mut cache = vec![last];
    for c in s {
        if c != last {
            result.append(&mut cache);
            cache.clear();
            last = c;
            count = 1;
            cache.push(last);
        } else {
            if count < 2 {
                count += 1;
                cache.push(last);
            }
        }
    }

    result.append(&mut cache);

    String::from_iter(result)
}

fn main() {
    assert_eq!(
        "leetcode".to_string(),
        make_fancy_string("leeetcode".to_string())
    );
    assert_eq!(
        "aabaa".to_string(),
        make_fancy_string("aaabaaaa".to_string())
    );

    assert_eq!("aab".to_string(), make_fancy_string("aab".to_string()));
}

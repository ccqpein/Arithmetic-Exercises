pub fn count_binary_substrings(s: String) -> i32 {
    let mut s = s.chars();
    let mut c = s.next().unwrap();
    let mut cache = vec![];
    let mut count = 1;

    loop {
        match s.next() {
            Some(cc) => {
                if cc != c {
                    cache.push(count);
                    c = cc;
                    count = 1;
                } else {
                    count += 1;
                }
            }
            None => {
                cache.push(count);
                break;
            }
        }
    }
    dbg!(&cache);

    let mut cache = cache.into_iter();
    let mut last = cache.next().unwrap();
    let mut result = 0;
    loop {
        match cache.next() {
            Some(x) => {
                // if x.min(last) {
                //     result += 2;
                // } else {
                //     result += 1
                // }
                result += x.min(last);
                last = x;
            }
            None => break,
        }
    }
    result
}

fn main() {
    assert_eq!(count_binary_substrings("00110011".into()), 6);
    assert_eq!(count_binary_substrings("10101".into()), 4);
    assert_eq!(count_binary_substrings("000111000".into()), 6);
}

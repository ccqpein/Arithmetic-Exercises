pub fn length_of_longest_substring(s: String) -> i32 {
    let mut temp_cache = vec![];
    let mut to_bytes = s.bytes();
    let mut result: i32 = 0;
    while let Some(b) = to_bytes.next() {
        if temp_cache.contains(&b) {
            if temp_cache.len() as i32 >= result {
                result = temp_cache.len() as i32
            }
            let index = temp_cache.iter().position(|&r| r == b).unwrap();
            temp_cache = temp_cache.drain(index + 1..).collect();
            temp_cache.push(b);
        } else {
            temp_cache.push(b);
        }
    }
    if temp_cache.len() as i32 > result {
        temp_cache.len() as i32
    } else {
        result
    }
}

fn main() {
    dbg!(length_of_longest_substring(String::from("abcabcb"))); //=> 3
    dbg!(length_of_longest_substring(String::from("bbb"))); //=> 1
    dbg!(length_of_longest_substring(String::from("pwwkew"))); //=> 3
    dbg!(length_of_longest_substring(String::from("dvdf"))); //=> 3
}

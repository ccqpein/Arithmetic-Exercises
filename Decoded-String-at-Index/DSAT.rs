pub fn decode_at_index(s: String, k: i32) -> String {
    use std::iter::FromIterator;
    let mut length = 0_usize;
    let mut cache = Vec::new();
    let mut result = String::new();
    let mut cache_clean = false;
    for c in s.chars() {
        if let Some(times) = c.to_digit(10) {
            length *= times as usize;
            //result += &String::from_iter(cache);
            //result = result.repeat(times as usize);
            cache_clean = true;
        } else {
            if cache_clean {
                cache = vec![];
                cache_clean = false;
            }
            length += 1;
            cache.push(c);
        }

        //println!("{}", result);
        if length == k as usize {
            return cache.pop().unwrap().to_string();
        }

        if length > k as usize {
            println!("{}", length);
            return result.get(k as usize - 1..k as usize).unwrap().into();
        }
    }

    result
}

fn inner_helper(s: &str, mut k: i32, last_cache: Vec<char>) -> String {
    let mut cache = last_cache;
    for (ind, c) in s.chars().enumerate() {
        if let Some(times) = c.to_digit(10) {
            if times as i32 * cache.len() as i32 >= k {
                return cache[(k % cache.len() as i32) as usize].into();
            } else {
                return inner_helper(
                    &s[ind..s.len()],
                    k - times as i32 * cache.len() as i32,
                    cache,
                );
            }
        } else {
            k -= 1;
            if k == 0 {
                return c.into();
            }
        }
    }
}

fn main() {
    dbg!(decode_at_index("leet2code3".to_string(), 10));
    dbg!(decode_at_index("ha22".to_string(), 5));
    dbg!(decode_at_index("a2345678999999999999999".to_string(), 1));
    dbg!(decode_at_index("a23".to_string(), 6));
}

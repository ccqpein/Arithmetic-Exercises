// pub fn decode_at_index(s: String, k: i32) -> String {
//     use std::iter::FromIterator;
//     let mut length = 0_usize;
//     let mut cache = Vec::new();
//     let mut result = String::new();
//     let mut cache_clean = false;
//     for c in s.chars() {
//         if let Some(times) = c.to_digit(10) {
//             length *= times as usize;
//             //result += &String::from_iter(cache);
//             //result = result.repeat(times as usize);
//             cache_clean = true;
//         } else {
//             if cache_clean {
//                 cache = vec![];
//                 cache_clean = false;
//             }
//             length += 1;
//             cache.push(c);
//         }

//         //println!("{}", result);
//         if length == k as usize {
//             return cache.pop().unwrap().to_string();
//         }

//         if length > k as usize {
//             println!("{}", length);
//             return result.get(k as usize - 1..k as usize).unwrap().into();
//         }
//     }

//     result
// }

pub fn decode_at_index(s: String, k: i32) -> String {
    inner_helper(&s, k, 0, vec![])
}

fn inner_helper(s: &str, k: i32, last_sum_k: i32, last_cache: Vec<char>) -> String {
    println!("s: {}, k: {}, last_cache: {:?}", s, k, last_cache);
    let mut cache = last_cache;
    let mut cache_new = false;
    let mut mut_k = k;
    for (ind, c) in s.chars().enumerate() {
        println!("cache is: {:?}", cache);
        if let Some(times) = c.to_digit(10) {
            dbg!(times as i32 * cache.len() as i32);
            if (times as i32 * cache.len() as i32) >= k {
                if k > cache.len() as i32 {
                    return cache[((k + 1) % cache.len() as i32) as usize].to_string();
                } else {
                    return cache[k as usize - 1].to_string();
                }
            } else {
                return inner_helper(
                    &s[ind + 1..s.len()],
                    k - times as i32 * (cache.len() as i32 + last_sum_k),
                    times as i32 * (cache.len() as i32 + last_sum_k),
                    cache,
                );
            }
        } else {
            mut_k -= 1;
            if mut_k == 0 {
                return c.to_string();
            }

            if !cache_new {
                cache = vec![];
                cache_new = true;
            }
            cache.push(c);
        }
    }

    if k > cache.len() as i32 {
        return cache[((k + 1) % cache.len() as i32) as usize].to_string();
    } else {
        return cache[k as usize - 1].to_string();
    }
}

fn main() {
    // assert_eq!(
    //     decode_at_index("leet2code3".to_string(), 10),
    //     "o".to_string()
    // );
    // assert_eq!(decode_at_index("ha22".to_string(), 5), "h".to_string());
    // assert_eq!(
    //     decode_at_index("a2345678999999999999999".to_string(), 1),
    //     "a".to_string()
    // );
    // assert_eq!(decode_at_index("a23".to_string(), 6), "a".to_string());

    assert_eq!(
        decode_at_index("a2b3c4d5e6f7g8h9".to_string(), 9),
        "c".to_string()
    );
}

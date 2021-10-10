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
    //inner_helper(&s, k, 0, vec![])
    inner_helper(&s, k as i64)
}

// fn inner_helper(s: &str, k: i32, last_sum_k: i32, last_cache: Vec<char>) -> String {
//     println!("s: {}, k: {}, last_cache: {:?}", s, k, last_cache);
//     let mut cache = last_cache;
//     let mut cache_new = false;
//     let mut mut_k = k;
//     for (ind, c) in s.chars().enumerate() {
//         println!("cache is: {:?}", cache);
//         if let Some(times) = c.to_digit(10) {
//             dbg!(times as i32 * cache.len() as i32);
//             if (times as i32 * cache.len() as i32) >= k {
//                 if k > cache.len() as i32 {
//                     return cache[((k + 1) % cache.len() as i32) as usize].to_string();
//                 } else {
//                     return cache[k as usize - 1].to_string();
//                 }
//             } else {
//                 return inner_helper(
//                     &s[ind + 1..s.len()],
//                     k - times as i32 * (cache.len() as i32 + last_sum_k),
//                     times as i32 * (cache.len() as i32 + last_sum_k),
//                     cache,
//                 );
//             }
//         } else {
//             mut_k -= 1;
//             if mut_k == 0 {
//                 return c.to_string();
//             }

//             if !cache_new {
//                 cache = vec![];
//                 cache_new = true;
//             }
//             cache.push(c);
//         }
//     }

//     if k > cache.len() as i32 {
//         return cache[((k + 1) % cache.len() as i32) as usize].to_string();
//     } else {
//         return cache[k as usize - 1].to_string();
//     }
// }

fn _inner_helper_v0(s: &str, mut k: i32) -> String {
    let mut groups = vec![];
    let (mut last_group, mut this_group) = (0, 0);
    let mut this_group_cache: Vec<char> = vec![];
    let mut group_cache = vec![];

    for (ind, c) in s.chars().enumerate() {
        if let Some(times) = c.to_digit(10) {
            group_cache.push(this_group_cache.clone());
            this_group_cache.clear();

            last_group += this_group;
            last_group *= times;
            groups.push(last_group);
            this_group = 0;
            if last_group as i32 > k {
                break;
            }
        } else {
            this_group_cache.push(c);
            this_group += 1;
        }
    }

    dbg!(&groups);
    dbg!(&group_cache);

    for x in (0..groups.len()).rev() {
        if (groups[x] as i32) < k {
            k %= groups[x] as i32;
            dbg!(x);
            break;
        }
    }
    dbg!(k);

    String::new()
}

fn inner_helper_v1(s: &str, k: i32) {
    let mut each_section: Vec<Vec<char>> = vec![];
    let mut section_cache: Vec<char> = vec![];

    let mut length_before_multi: Vec<i32> = vec![];

    let mut lenght_after_multi: Vec<i32> = vec![];

    let mut last_length_after_multi = 0;
    let mut last = true;
    for c in s.chars() {
        if let Some(times) = c.to_digit(10) {
            length_before_multi.push(last_length_after_multi + section_cache.len() as i32);
            lenght_after_multi
                .push((last_length_after_multi + section_cache.len() as i32) * times as i32);

            last_length_after_multi =
                (last_length_after_multi + section_cache.len() as i32) * times as i32;

            each_section.push(section_cache.clone());
            section_cache.clear();
        } else {
            section_cache.push(c);
        }

        if last_length_after_multi > k {
            last = false;
            break;
        }
    }

    // last
    if last {
        length_before_multi.push(last_length_after_multi + section_cache.len() as i32);
        each_section.push(section_cache.clone());
    }

    dbg!(each_section);
    dbg!(lenght_after_multi);
    dbg!(length_before_multi);
}

fn inner_helper(s: &str, k: i64) -> String {
    let mut l = 0;
    for c in s.chars() {
        if let Some(times) = c.to_digit(10) {
            if k <= l * (times as i64) {
                if k % l == 0 {
                    return inner_helper(s, l);
                }
                return inner_helper(s, k % l);
            }
            l *= times as i64;
        } else {
            l += 1;
            if l == k {
                return c.to_string();
            }
        }
    }
    "".to_string()
}

fn main() {
    assert_eq!(
        decode_at_index("leet2code3".to_string(), 10),
        "o".to_string()
    );

    assert_eq!(decode_at_index("ha22".to_string(), 5), "h".to_string());

    assert_eq!(
        decode_at_index("a2345678999999999999999".to_string(), 1),
        "a".to_string()
    );

    assert_eq!(decode_at_index("a23".to_string(), 6), "a".to_string());

    assert_eq!(
        decode_at_index("a2b3c4d5e6f7g8h9".to_string(), 9),
        "b".to_string()
    );

    assert_eq!(
        decode_at_index("ajx37nyx97niysdrzice4petvcvmcgqn282zicpbx6okybw93vhk782unctdbgmcjmbqn25rorktmu5ig2qn2y4xagtru2nehmsp".to_string(), 976159153),
        "a".to_string()
    );

    //dbg!(inner_helper("leet2code3", 10));
    //dbg!(inner_helper("a2b3c4d5e6f7g8h9", 9));
    //dbg!(inner_helper("ha22", 3));
    //dbg!(inner_helper("a23", 6));
}

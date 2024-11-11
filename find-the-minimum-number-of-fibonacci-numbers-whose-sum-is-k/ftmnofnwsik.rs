use std::collections::HashMap;

const TABLE: [i32; 40] = [
    1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
];

pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
    let mut cache = HashMap::new();
    for kk in 1..=k {
        helper(kk, &mut cache);
    }
    *cache.get(&k).unwrap()
}

fn helper(k: i32, cache: &mut HashMap<i32, i32>) -> Option<i32> {
    //dbg!(k);
    if k == 0 {
        return Some(0);
    }

    if let Some(aa) = cache.get(&k) {
        return Some(*aa);
    }

    //let mut result = None;
    // for n in TABLE.iter().rev().filter_map(|n| {
    //     if *n <= k {
    //         match helper(k - n) {
    //             Some(res) => Some(res + 1),
    //             None => None,
    //         }
    //     } else {
    //         None
    //     }
    // }) {
    //     match result.as_mut() {
    //         Some(a) => *a += n,
    //         None => result = Some(n),
    //     }
    // }

    let a = TABLE
        .iter()
        .rev()
        .filter_map(|n| {
            if *n <= k {
                match helper(k - n, cache) {
                    Some(res) => Some(res + 1),
                    None => None,
                }
            } else {
                None
            }
        })
        .min();
    if let Some(aa) = a {
        // let en = cache.entry(k).or_insert(i32::MAX);
        // *en = (*en).min(aa);
        // return Some(en.clone());
        cache.insert(k, aa);
        cache.get(&k).copied()
    } else {
        return None;
    }
    //result
}

fn main() {
    assert_eq!(find_min_fibonacci_numbers(2), 1);
    assert_eq!(find_min_fibonacci_numbers(7), 2);
    assert_eq!(find_min_fibonacci_numbers(10), 2);
    assert_eq!(find_min_fibonacci_numbers(19), 3);
    assert_eq!(find_min_fibonacci_numbers(3), 1);
    dbg!(find_min_fibonacci_numbers(9083494));
}

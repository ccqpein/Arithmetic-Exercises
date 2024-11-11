use std::collections::HashMap;

const TABLE: [i32; 45] = [
    1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946,
    17711, 28657, 46368, 75025, 121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
    433494437, 701408733, 1134903170,
];

pub fn find_min_fibonacci_numbers(k: i32) -> i32 {
    let mut cache = HashMap::new();
    for nn in TABLE {
        cache.insert(nn, 1);
    }

    // for kk in 1..=(*TABLE.iter().rev().find(|n| **n < k).unwrap()) {
    //     helper(kk, &mut cache);
    // }

    // for kk in 1..=k {
    //     helper2(kk, &mut cache);
    // }
    //helper2(k, &mut cache);
    helper3(k, &mut cache);

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
                match cache.get(&(k - n)) {
                    Some(small_res) => Some(small_res + 1),
                    None => None,
                }
                // match helper(k - n, cache) {
                //     Some(res) => Some(res + 1),
                //     None => None,
                // }
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

fn helper2(k: i32, cache: &mut HashMap<i32, i32>) -> Option<i32> {
    if k == 0 {
        return Some(0);
    }

    if let Some(aa) = cache.get(&k) {
        return Some(*aa);
    }

    let mut res = i32::MAX;
    for nn in TABLE.iter().rev() {
        if *nn <= k {
            if let Some(next) = helper2(k - *nn, cache) {
                res = res.min(1 + next);
                break;
            }
        }
    }

    if res == i32::MAX {
        None
    } else {
        cache.insert(k, res);
        Some(res)
    }
}

fn helper3(k: i32, cache: &mut HashMap<i32, i32>) -> Option<i32> {
    if k == 0 {
        return Some(0);
    }

    if let Some(aa) = cache.get(&k) {
        return Some(*aa);
    }

    let mut res = i32::MAX;
    for nn in TABLE.iter().rev() {
        if *nn <= k {
            if let Some(next) = helper3(k - (k / *nn) * nn, cache) {
                res = res.min((k / *nn) + next);
            }
        }
    }

    if res == i32::MAX {
        None
    } else {
        cache.insert(k, res);
        Some(res)
    }
}

fn main() {
    assert_eq!(find_min_fibonacci_numbers(2), 1);
    assert_eq!(find_min_fibonacci_numbers(7), 2);
    assert_eq!(find_min_fibonacci_numbers(10), 2);
    assert_eq!(find_min_fibonacci_numbers(19), 3);
    assert_eq!(find_min_fibonacci_numbers(3), 1);
    assert_eq!(find_min_fibonacci_numbers(9083494), 10);
    assert_eq!(find_min_fibonacci_numbers(645157245), 13);
}

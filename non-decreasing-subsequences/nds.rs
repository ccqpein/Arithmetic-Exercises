use std::collections::{HashMap, HashSet};

pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut x = 0;

    let mut a = nums[0];
    let mut bucket = vec![];
    for i in 1..nums.len() {
        if nums[i] < a {
            bucket.push((x, i));
            x = i
        } else {
            a = nums[i]
        }
    }
    bucket.push((x, nums.len()));

    //dbg!(&bucket);
    //dbg!(pick_some(&nums, bucket[0], (bucket[0].1 - bucket[0].0)));

    let mut set = HashSet::new();
    let mut table = HashMap::new();
    for b in bucket {
        if b.1 - b.0 > 1 {
            for bb in pick_some(&nums, b, b.1 - b.0, &mut table) {
                set.insert(bb);
            }
        }
    }

    set.into_iter().collect()
}

// fn subseq(nums: &Vec<i32>, coops: (usize, usize)) -> Vec<Vec<i32>> {
//     if coops.1 - coops.0 == 1 {
//         return vec![];
//     }
//     let mut result = vec![];
//     for count in 2..=(coops.1 - coops.0) {
//         let mut cache = vec![nums[start]];
//         for start in coops.0..coops.1 {
//             result.push(vec![
//                 cache,
//                 pick_some(nums, (start + 1, coops.1), count - 1),
//             ])
//         }
//     }
//     result
// }

fn pick_some(
    nums: &Vec<i32>,
    coops: (usize, usize),
    left: usize,
    table: &mut HashMap<(usize, usize, usize), Vec<Vec<i32>>>,
) -> Vec<Vec<i32>> {
    if left == 0 || left > coops.1 - coops.0 {
        return vec![];
    };

    if let Some(cc) = table.get(&(coops.0, coops.1, left)) {
        return cc.clone();
    }

    if left == 1 {
        table.insert(
            (coops.0, coops.1, left),
            (coops.0..coops.1).map(|ind| vec![nums[ind]]).collect(),
        );
        return (coops.0..coops.1).map(|ind| vec![nums[ind]]).collect();
    }

    let mut result = vec![];
    for x in coops.0..coops.1 {
        let cache = vec![nums[x]];
        for ll in (1..left).rev() {
            if let Some(cc) = table.get(&(x + 1, coops.1, left - ll)) {
                for xx in cc {
                    result.push([cache.clone(), xx.clone()].concat());
                }
                continue;
            }
            let cc = pick_some(nums, (x + 1, coops.1), left - ll, table);
            table.insert((x + 1, coops.1, left - ll), cc.clone());
            for xx in cc {
                result.push([cache.clone(), xx].concat());
            }
        }
    }

    table.insert((coops.0, coops.1, left), result.clone());
    result
}

fn main() {
    //dbg!(find_subsequences(vec![4, 6, 7, 7]));
    //dbg!(find_subsequences(vec![4, 4, 3, 2, 1]));
    dbg!(find_subsequences(vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15
    ]));

    //dbg!(pick_some(&vec![4, 6, 7, 7], (0, 4), 4));
}

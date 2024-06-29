// pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
//     let cache = helper(&nums);
//     //let threshold = threshold as f64;

//     for ind in 0..cache.len() {
//         let left = cache[0..ind]
//             .iter()
//             .filter(|(x, a)| *a != cache[ind].1 && *x < cache[ind].0)
//             .map(|(x, _)| *x + 1)
//             .max()
//             .unwrap_or(0);
//         let right = cache[0..ind]
//             .iter()
//             .filter(|(x, a)| *a != cache[ind].1 && *x > cache[ind].0)
//             .map(|(x, _)| *x)
//             .min()
//             .unwrap_or(cache.len());

//         dbg!(cache[ind]);
//         dbg!(left);
//         dbg!(right);

//         if threshold / ((right - left) as i32) < cache[ind].1 {
//             return (right - left) as i32;
//         }
//     }
//     -1
// }

// fn helper(nums: &[i32]) -> Vec<(usize, i32)> {
//     let mut result = nums
//         .iter()
//         .enumerate()
//         .map(|(ind, a)| (ind, *a))
//         .collect::<Vec<_>>();
//     result.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
//     result
// }

use std::collections::HashMap;

pub fn valid_subarray_size(nums: Vec<i32>, threshold: i32) -> i32 {
    let cache = helper(&nums);
    //let threshold = threshold as f64;
    let length = nums.len();
    let (mut left, mut right) = (0, length);
    let mut all_inds = vec![None; nums.len()];
    for mut xx in cache {
        for i in &xx.1 {
            (left, right) = faster_find(&all_inds, *i);

            dbg!(&xx);
            dbg!(left);
            dbg!(right);

            if threshold / ((right - left) as i32) < xx.0 {
                return (right - left) as i32;
            }
        }

        //all_inds.append(&mut xx.1);
        for i in xx.1 {
            all_inds[i] = Some(xx.0);
        }
        dbg!(&all_inds);
    }
    -1
}

fn faster_find(all_ind: &[Option<i32>], x: usize) -> (usize, usize) {
    let mut left = None;

    if x == 0 {
        left = Some(0);
    } else {
        for i in (0..x).rev() {
            if all_ind[i].is_some() {
                left = Some(i + 1);
                break;
            }
        }
    }

    if left.is_none() {
        left = Some(0)
    }
    /////////////

    let mut right = None;

    if x == all_ind.len() - 1 {
        right = Some(all_ind.len());
    } else {
        for i in x..all_ind.len() {
            if all_ind[i].is_some() {
                right = Some(i);
                break;
            }
        }
    }

    if right.is_none() {
        right = Some(all_ind.len())
    }

    (left.unwrap(), right.unwrap())
}

fn helper(nums: &[i32]) -> Vec<(i32, Vec<usize>)> {
    let mut table = HashMap::new();
    let _ = nums
        .iter()
        .enumerate()
        .for_each(|(ind, x)| table.entry(x).or_insert(vec![]).push(ind));

    let mut result = table.into_iter().map(|(k, v)| (*k, v)).collect::<Vec<_>>();
    result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    result
}

fn main() {
    //dbg!(helper(&vec![1, 3, 4, 3, 1]));
    //dbg!(helper(&vec![6, 5, 6, 5, 8]));

    //dbg!(valid_subarray_size(vec![1, 3, 4, 3, 1], 6));
    //dbg!(valid_subarray_size(vec![6, 5, 6, 5, 8], 7));
    dbg!(valid_subarray_size(
        vec![818, 232, 595, 418, 608, 229, 37, 330, 876, 774, 931, 939, 479, 884, 354, 328],
        3790
    ));
}

use std::collections::HashMap;

// pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
//     let mut bucket = vec![0; 201];
//     for n in nums {
//         bucket[(n - (-100)) as usize] += 1
//     }

//     let mut bucket = bucket
//         .into_iter()
//         .enumerate()
//         .filter_map(|(ind, f)| {
//             if f != 0 {
//                 Some(((ind as i32 - 100), f))
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>();

//     bucket.sort_by(|a, b| a.1.cmp(&b.1));
//     bucket
//         .into_iter()
//         .map(|(n, f)| vec![n; f])
//         .flatten()
//         .collect()
// }

pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
    let mut bucket = vec![0; 201];
    for n in nums {
        bucket[(n - (-100)) as usize] += 1
    }

    let mut table = HashMap::new();
    bucket
        .into_iter()
        .enumerate()
        .filter_map(|(ind, f)| {
            if f != 0 {
                Some(((ind as i32 - 100), f))
            } else {
                None
            }
        })
        .for_each(|(n, f)| {
            table.entry(f).or_insert(vec![]).push(n);
        });

    let mut result = vec![];
    for i in 1..=100 {
        match table.get_mut(&i) {
            Some(nums) => {
                if nums.len() == 1 {
                    result.append(&mut vec![nums[0]; i as usize]);
                } else {
                    nums.sort_by(|a, b| b.cmp(&a));
                    for n in nums {
                        result.append(&mut vec![*n; i as usize]);
                    }
                }
            }
            None => (),
        }
    }
    result
}

fn main() {
    dbg!(frequency_sort(vec![1, 1, 2, 2, 2, 3]));
    dbg!(frequency_sort(vec![2, 3, 1, 3, 2]));
    dbg!(frequency_sort(vec![-1, 1, -6, 4, 5, -6, 1, 4, 1]));
}

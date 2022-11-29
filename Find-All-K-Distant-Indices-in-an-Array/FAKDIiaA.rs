use std::collections::HashSet;

pub fn find_k_distant_indices(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
    let max_ind = nums.len() as i32;
    let mut placeholder = -1;
    let mut result = nums
        .into_iter()
        .enumerate()
        .flat_map(|(ind, v)| {
            if v == key {
                //println!("{}, {}", ind, v);
                0.max(ind as i32 - k)..=max_ind.min(ind as i32 + k)
            } else {
                -100..=-99
            }
        })
        .collect::<HashSet<i32>>()
        .into_iter()
        .filter(|i| *i >= 0 && *i < max_ind)
        .collect::<Vec<_>>();

    result.sort();
    //result.drain(2..).collect()
    result
}

// pub fn find_k_distant_indices2(nums: Vec<i32>, key: i32, k: i32) -> Vec<i32> {
//     let max_ind = nums.len() as i32;
//     let mut result = vec![];
//     let mut ii = nums.into_iter().enumerate();
//     let mut this_ind = 0;
//     loop {
//         if let Some((ind, v)) = ii.nth(this_ind) {
//             if v == key {
//                 result.append(
//                     &mut (0.max(ind as i32 - k)..=max_ind.min(ind as i32 + k))
//                         .into_iter()
//                         .collect(),
//                 );
//                 this_ind += k as usize
//             } else {
//                 this_ind += 1;
//             }
//         } else {
//             break;
//         }
//     }
//     result
// }

fn main() {
    assert_eq!(
        find_k_distant_indices(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
        vec![1, 2, 3, 4, 5, 6]
    );
    assert_eq!(
        find_k_distant_indices(vec![2, 2, 2, 2, 2], 2, 2),
        vec![0, 1, 2, 3, 4]
    );
    // assert_eq!(
    //     find_k_distant_indices2(vec![3, 4, 9, 1, 3, 9, 5], 9, 1),
    //     vec![1, 2, 3, 4, 5, 6]
    // );
    // assert_eq!(
    //     find_k_distant_indices2(vec![2, 2, 2, 2, 2], 2, 2),
    //     vec![0, 1, 2, 3, 4]
    // );
}

// pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
//     let mut cache = [-1; 50_000];
//     let mut cache_smaller = [0; 50_000];
//     let length = nums.len();
//     if length < 2 {
//         return 0;
//     }

//     let nums: Vec<i64> = nums.iter().map(|&s| s as i64).collect();
//     for i in (0..=length - 2).rev() {
//         cache[i] = 0;
//         let mut flag = 0;
//         for j in i + 1..=length - 1 {
//             if cache[j] != -1 && nums[i] > nums[j] << 1 {
//                 if nums[j] <= flag {
//                     break;
//                 } else {
//                     flag = nums[j]
//                 }
//                 cache[i] += cache[j] + 1;
//                 cache[i] += cache_smaller[j];
//             } else if nums[i] > nums[j] << 1 {
//                 cache[i] += 1;
//             } else if nums[i] >= nums[j] {
//                 cache_smaller[i] += 1;
//             }
//         }
//     }
//     dbg!(&cache[0..17]);
//     dbg!(&cache_smaller[0..17]);
//     cache.iter().filter(|&x| *x != -1).sum()
// }

// of course this method is too slow
pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let nums: Vec<i64> = nums.iter().map(|&s| s as i64).collect();

    let mut count = 0;
    for i in 0..nums.len() {
        for j in 0..i {
            if nums[j] > nums[i] * 2 {
                count += 1
            }
        }
    }
    count
}

fn main() {
    // assert_eq!(2, reverse_pairs(vec![1, 3, 2, 3, 1]));
    // assert_eq!(3, reverse_pairs(vec![2, 4, 3, 5, 1]));
    // assert_eq!(0, reverse_pairs(vec![1]));
    // assert_eq!(4, reverse_pairs(vec![5, 4, 3, 2, 1]));
    // assert_eq!(
    //     0,
    //     reverse_pairs(vec![
    //         2147483647, 2147483647, 2147483647, 2147483647, 2147483647, 2147483647
    //     ])
    // );
    // assert_eq!(
    //     9,
    //     reverse_pairs(vec![
    //         2147483647,
    //         2147483647,
    //         -2147483647,
    //         -2147483647,
    //         -2147483647,
    //         2147483647
    //     ])
    // );

    assert_eq!(
        40,
        reverse_pairs(vec![
            233, 2000000001, 234, 2000000006, 235, 2000000003, 236, 2000000007, 237, 2000000002,
            2000000005, 233, 233, 233, 233, 233, 2000000004
        ])
    );
}

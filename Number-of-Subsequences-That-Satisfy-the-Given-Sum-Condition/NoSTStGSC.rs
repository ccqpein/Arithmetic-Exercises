// pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
//     let mut nums = nums;
//     nums.sort();

//     let mut result = 0u128;

//     loop {
//         let mut iteror = nums.iter();
//         let position = match iteror.rposition(|x| *x + nums[0] <= target) {
//             Some(n) => n,
//             None => break,
//         };

//         if nums[0] * 2 <= target {
//             result += 1;
//         }

//         // 2^29 < 1000000007 < 2^30
//         //result = (result + 2u128.pow(position as u32) - 1) % 1000000007;
//         result = result % 1000000007 + big_modulo_helper(position as u32) - 1;

//         nums = nums.drain(1..).collect()
//     }

//     result as i32
// }

// fn big_modulo_helper(mut exp: u32) -> u128 {
//     let mut result = 1u128;
//     while exp >= 30 {
//         result *= 73741817;
//         result %= 1000000007;
//         //result *= 2u128.pow(30) % 1000000007;
//         exp -= 30
//     }

//     result *= 2u128.pow(exp) % 1000000007;
//     result % 1000000007
// }

/// solution below is faster
use std::collections::HashMap;
pub fn num_subseq(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();

    let mut result = 0u128;

    let mut head_ind = 0;
    let mut end_ind = nums
        .iter()
        .rposition(|x| *x + nums[0] <= target)
        .unwrap_or(0);

    let mut store_table: HashMap<u32, u128> = HashMap::new();
    make_modulo_table(end_ind as u32, &mut store_table);

    loop {
        if head_ind > end_ind {
            break;
        }

        loop {
            if nums[end_ind] + nums[head_ind] <= target {
                break;
            } else {
                if end_ind == 0 {
                    return result as i32;
                }
                end_ind -= 1;
            }

            if end_ind < head_ind {
                return result as i32;
            }
        }

        if nums[head_ind] * 2 <= target {
            result += 1;
        }

        result = result % 1000000007 + store_table.get(&((end_ind - head_ind) as u32)).unwrap() - 1;

        head_ind += 1;
    }

    result as i32
}

fn make_modulo_table(exp: u32, table: &mut HashMap<u32, u128>) {
    table.insert(0, 1);
    for i in 1..=exp {
        let v = table.get(&(i - 1)).unwrap() * 2 % 1000000007;
        table.insert(i, v);
    }
}

fn main() {
    assert_eq!(num_subseq(vec![1], 1), 0);
    assert_eq!(num_subseq(vec![3, 5, 6, 7], 9), 4);
    assert_eq!(num_subseq(vec![3, 3, 6, 8], 10), 6);
    assert_eq!(num_subseq(vec![2, 3, 3, 4, 6, 7], 12), 61);
    assert_eq!(num_subseq(vec![5, 2, 4, 1, 7, 6, 8], 16), 127);

    assert_eq!(
        num_subseq(
            vec![
                14, 4, 6, 6, 20, 8, 5, 6, 8, 12, 6, 10, 14, 9, 17, 16, 9, 7, 14, 11, 14, 15, 13,
                11, 10, 18, 13, 17, 17, 14, 17, 7, 9, 5, 10, 13, 8, 5, 18, 20, 7, 5, 5, 15, 19, 14
            ],
            22
        ),
        272187084
    );

    assert_eq!(
        num_subseq(
            vec![
                27, 21, 14, 2, 15, 1, 19, 8, 12, 24, 21, 8, 12, 10, 11, 30, 15, 18, 28, 14, 26, 9,
                2, 24, 23, 11, 7, 12, 9, 17, 30, 9, 28, 2, 14, 22, 19, 19, 27, 6, 15, 12, 29, 2,
                30, 11, 20, 30, 21, 20, 2, 22, 6, 14, 13, 19, 21, 10, 18, 30, 2, 20, 28, 22
            ],
            31
        ),
        688052206
    );

    assert_eq!(
        num_subseq(
            vec![
                9, 25, 9, 28, 24, 12, 17, 8, 28, 7, 21, 25, 10, 2, 16, 19, 12, 13, 15, 28, 14, 12,
                24, 9, 6, 7, 2, 15, 19, 13, 30, 30, 23, 19, 11, 3, 17, 2, 14, 20, 22, 30, 12, 1,
                11, 2, 2, 20, 20, 27, 15, 9, 10, 4, 12, 30, 13, 5, 2, 11, 29, 5, 3, 13, 22, 5, 16,
                19, 7, 19, 11, 16, 11, 25, 29, 21, 29, 3, 2, 9, 20, 15, 9
            ],
            32
        ),
        91931447
    );

    assert_eq!(
        num_subseq(
            vec![
                82, 30, 56, 28, 63, 24, 17, 50, 45, 95, 50, 41, 10, 59, 59, 17, 39, 11, 36, 64, 44,
                16, 56, 31, 83, 100, 12, 69, 13, 46, 58, 92, 55, 71, 33, 99, 78, 13, 17, 50, 82,
                33, 12, 22, 48, 48, 77, 71, 11, 83, 26, 51, 26, 56, 51, 82, 54, 50, 13, 64, 83, 48,
                31, 28, 33, 89, 60, 22, 25, 35, 89, 80, 65, 92, 52, 29, 64, 96, 98, 76, 93, 77, 90,
                72, 49, 62, 78, 41, 99, 22, 36, 64, 23, 76, 71, 69, 83, 84, 77, 72, 24, 21, 89, 37,
                20, 59, 48, 78, 68, 60, 25, 93, 38, 83, 16, 85, 10, 20, 34, 98, 73, 16, 45, 99, 75,
                29, 12, 27, 35, 54, 72, 10, 99, 61, 92, 67, 43, 31, 27, 88, 51, 92, 75, 59, 47, 48,
                48, 22, 61, 69, 69, 90, 61, 85, 48, 83, 79, 96, 27, 48, 24, 20, 35, 52, 49, 58, 84,
                71, 30, 89, 72, 74, 72, 87, 31, 43, 66, 18, 92, 63, 48, 71, 75, 42, 37, 53, 76, 79,
                87
            ],
            125
        ),
        945569954
    )
}

fn sum_vec(nums: &Vec<i32>) -> i32 {
    let mut result = 0;
    for i in nums {
        result += *i;
    }
    result
}

pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
    if nums.len() == 1 {
        return false;
    }
    let mut slice_copy = nums.as_slice();
    for count in 1..nums.len() {
        for start in 0..nums.len() - count {
            match k {
                0 => {
                    if slice_copy[start..start + count + 1].iter().sum::<i32>() == 0 {
                        return true;
                    }
                }
                _ => {
                    if slice_copy[start..start + count + 1].iter().sum::<i32>() % k.abs() == 0 {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn main() {
    dbg!(check_subarray_sum(vec![23, 2, 4, 6, 7], 6)); //t
    dbg!(check_subarray_sum(vec![23, 2, 6, 4, 7], -6)); //t
    dbg!(check_subarray_sum(vec![0, 0], -1)); //t
    dbg!(check_subarray_sum(vec![1, 2, 3], 4)); //nil
    dbg!(check_subarray_sum(vec![0, 0], 0)); //t
    dbg!(check_subarray_sum(vec![0], 0)); //nil
}

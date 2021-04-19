use std::collections::HashMap;

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut table: HashMap<usize, usize> = HashMap::new();

    (0..nums.len())
        .rev() // add rev() make it faster
        .map(|ind| helper(&nums, ind, &mut table))
        .max()
        .unwrap() as i32
}

fn helper(nums: &Vec<i32>, ind: usize, table: &mut HashMap<usize, usize>) -> usize {
    if let Some(v) = table.get(&ind) {
        *v
    } else {
        let this = (ind + 1..nums.len())
            .filter_map(|i| {
                if nums[i] > nums[ind] {
                    Some(1 + helper(nums, i, table))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(1);
        table.insert(ind, this);
        this
    }
}

/// solution below are faster

pub fn length_of_lis_2(nums: Vec<i32>) -> i32 {
    let mut cache = [0; 2500];
    (0..nums.len())
        .rev()
        .map(|ind| helper_2(&nums, ind, &mut cache))
        .max()
        .unwrap() as i32
}

fn helper_2(nums: &Vec<i32>, ind: usize, cache: &mut [usize]) -> usize {
    if cache[ind] != 0 {
        cache[ind]
    } else {
        let this = (ind + 1..nums.len())
            .filter_map(|i| {
                if nums[i] > nums[ind] {
                    Some(1 + helper_2(nums, i, cache))
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(1);
        cache[ind] = this;
        this
    }
}

pub fn length_of_lis_3(nums: Vec<i32>) -> i32 {
    use std::cmp::max;
    let mut tail = vec![0; nums.len()];
    let mut size = 0;
    for n in nums {
        let (mut i, mut j) = (0, size);
        while i != j {
            let m = (i + j) / 2;
            if tail[m as usize] < n {
                i = m + 1
            } else {
                j = m
            }
        }
        tail[i as usize] = n;
        size = max(i + 1, size);
    }
    size
}

fn main() {
    // assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    // assert_eq!(length_of_lis(vec![0, 1, 0, 3, 2, 3]), 4);
    // assert_eq!(length_of_lis(vec![7, 7, 7, 7, 7, 7, 7]), 1);

    assert_eq!(length_of_lis_2(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    assert_eq!(length_of_lis_2(vec![0, 1, 0, 3, 2, 3]), 4);
    assert_eq!(length_of_lis_2(vec![7, 7, 7, 7, 7, 7, 7]), 1);
    assert_eq!(length_of_lis_2(vec![1, 3, 6, 7, 9, 4, 10, 5, 6]), 6);
}

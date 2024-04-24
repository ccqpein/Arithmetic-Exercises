use std::collections::VecDeque;

pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    //let (a, b) = helper(nums.into());
    let (a, b) = helper2(&nums, 0, nums.len() - 1);
    println!("a: {}, b: {}", a, b);
    a >= b
}

fn helper(mut nums: VecDeque<i32>) -> (i32, i32) {
    if nums.len() == 0 {
        return (0, 0);
    }
    //dbg!(&nums);

    if nums.len() == 1 {
        return (nums.pop_front().unwrap(), 0);
    }

    let mut nums1 = nums.clone();
    let aa0 = nums.pop_front().unwrap();
    let (a0, b0) = helper(nums);

    let aa1 = nums1.pop_back().unwrap();
    let (a1, b1) = helper(nums1);

    if aa0 + b0 > aa1 + b1 {
        return (b0 + aa0, a0);
    } else {
        return (b1 + aa1, a1);
    }
}

fn helper2(nums: &Vec<i32>, start: usize, end: usize) -> (i32, i32) {
    if start > end {
        return (0, 0);
    }

    if start == end {
        return (nums[start], 0);
    }

    let aa0 = nums[start];
    let (a0, b0) = helper2(nums, start + 1, end);

    let aa1 = nums[end];
    let (a1, b1) = helper2(nums, start, end - 1);

    if aa0 + b0 > aa1 + b1 {
        return (b0 + aa0, a0);
    } else {
        return (b1 + aa1, a1);
    }
}

fn main() {
    assert!(!predict_the_winner(vec![1, 5, 2]));
    assert!(predict_the_winner(vec![1, 5, 233, 7]));
}

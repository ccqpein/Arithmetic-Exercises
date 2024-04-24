use std::collections::VecDeque;

pub fn predict_the_winner(nums: Vec<i32>) -> bool {
    let (a, b) = helper(nums.into());
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

fn main() {
    assert!(!predict_the_winner(vec![1, 5, 2]));
    assert!(predict_the_winner(vec![1, 5, 233, 7]));
}

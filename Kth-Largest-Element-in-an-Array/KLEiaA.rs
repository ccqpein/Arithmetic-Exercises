use std::collections::HashSet;
use std::iter::FromIterator;

pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    //let set: HashSet<i32> = HashSet::from_iter(nums.iter().cloned());
    //dbg!(&set);
    //let mut inner_nums = set.into_iter().collect::<Vec<i32>>();
    let mut inner_nums = nums;
    inner_nums.sort_by(|a, b| b.partial_cmp(a).unwrap());
    //dbg!(&inner_nums);
    inner_nums[(k - 1) as usize]
}

fn main() {
    println!("{:?}", find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4));
    println!("{:?}", find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));
}

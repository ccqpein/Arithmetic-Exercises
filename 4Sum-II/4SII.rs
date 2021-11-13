use std::collections::HashMap;

pub fn four_sum_count(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>, nums4: Vec<i32>) -> i32 {
    let a = helper(&nums1, &nums2);
    let b = helper(&nums3, &nums4);
    dbg!("{:?}", &a);
    dbg!("{:?}", &b);
    let mut result = 0;
    for (key, v) in a {
        if let Some(vb) = b.get(&(0 - key)) {
            result += v * vb;
        }
    }

    result as i32
}

fn helper(nums1: &[i32], nums2: &[i32]) -> HashMap<i32, usize> {
    let mut table = HashMap::new();
    for n1 in nums1 {
        for n2 in nums2 {
            let en = table.entry(n1 + n2).or_insert(0_usize);
            *en += 1;
        }
    }

    table
}

fn main() {
    assert_eq!(
        four_sum_count(vec![1, 2], vec![-2, -1], vec![-1, 2], vec![0, 2]),
        2
    );
}

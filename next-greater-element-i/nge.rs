use std::collections::HashMap;

pub fn next_greater_element(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
    let mut table: HashMap<i32, i32> = HashMap::new();
    nums2.reverse();

    let mut bucket = vec![];

    for n in nums2 {
        loop {
            if bucket.is_empty() {
                table.insert(n, -1);
                break;
            };
            if bucket[bucket.len() - 1] > n {
                table.insert(n, bucket[bucket.len() - 1]);
                break;
            } else {
                bucket.pop();
            }
        }
        bucket.push(n);
    }

    nums1.into_iter().map(|n| table[&n]).collect()
}

fn main() {
    assert_eq!(
        vec![-1, 3, -1],
        next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
    );

    assert_eq!(
        vec![3, -1],
        next_greater_element(vec![2, 4], vec![1, 2, 3, 4])
    );
}

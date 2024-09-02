pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
    if nums.is_empty() {
        return vec![];
    }

    let mut bucket: Vec<i32> = nums.iter().rev().map(|n| *n).collect();
    let mut res = Vec::with_capacity(nums.len());
    for n in nums.iter().rev() {
        loop {
            match bucket.pop() {
                Some(nn) => {
                    if nn > *n {
                        bucket.push(nn);
                        bucket.push(*n);
                        res.push(nn);
                        break;
                    }
                }
                None => {
                    res.push(-1);
                    bucket.push(*n);
                    break;
                }
            }
        }
    }

    res.reverse();
    res
}

fn main() {
    dbg!(next_greater_elements(vec![1, 2, 1]));
    dbg!(next_greater_elements(vec![5, 4, 3, 2, 1]));
}

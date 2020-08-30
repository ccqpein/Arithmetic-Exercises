use std::collections::HashMap;

pub fn rob(nums: Vec<i32>) -> i32 {
    let mut table: HashMap<usize, i32> = HashMap::new();
    rob_inner(&nums, 0, nums.len(), &mut table)
}

fn rob_inner(nums: &Vec<i32>, start: usize, len: usize, ta: &mut HashMap<usize, i32>) -> i32 {
    if let Some(v) = ta.get(&start) {
        return *v;
    }

    use std::cmp::max;
    if start == len - 1 {
        ta.insert(start, nums[start]);
        return nums[start];
    } else if start == len {
        ta.insert(start, 0);
        return 0;
    }

    let a = max(
        (start + 1..len)
            .into_iter()
            .map(|x| rob_inner(nums, x, len, ta))
            .max()
            .unwrap(),
        nums[start] + rob_inner(nums, start + 2, len, ta),
    );

    ta.insert(start, a);

    a
}

fn main() {
    assert_eq!(rob(vec![2, 7, 9, 3, 1]), 12);
    assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    assert_eq!(rob(vec![2, 7, 9, 3, 1, 8]), 19);
    assert_eq!(
        rob(vec![
            183, 219, 57, 193, 94, 233, 202, 154, 65, 240, 97, 234, 100, 249, 186, 66, 90, 238,
            168, 128, 177, 235, 50, 81, 185, 165, 217, 207, 88, 80, 112, 78, 135, 62, 228, 247,
            211
        ]),
        3365
    );
}

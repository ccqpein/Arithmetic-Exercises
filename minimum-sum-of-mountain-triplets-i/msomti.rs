pub fn minimum_sum(nums: Vec<i32>) -> i32 {
    let mut nums = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();
    nums.sort_by(|a, b| a.1.cmp(&b.1));
    //dbg!(&nums);

    let mut right = None;
    let mut left = None;
    let mut result = vec![];
    for ind in 2..nums.len() {
        for i in 0..ind {
            if nums[i].0 > nums[ind].0 && nums[i].1 != nums[ind].1 {
                right = nums.get(i);
                break;
            }
        }

        for i in 0..ind {
            if nums[i].0 < nums[ind].0 && nums[i].1 != nums[ind].1 {
                left = nums.get(i);
                break;
            }
        }

        match (right, left) {
            (Some(a), Some(b)) => {
                //dbg!(&left, &right, &ind);
                result.push(a.1 + b.1 + nums[ind].1);
            }
            _ => {}
        }
        left = None;
        right = None
    }

    result.into_iter().min().unwrap_or(-1)
}

fn main() {
    //dbg!(minimum_sum(vec![5, 4, 8, 7, 10, 2]));
    dbg!(minimum_sum(vec![8, 6, 1, 5, 3]));
    //dbg!(minimum_sum(vec![6, 5, 4, 3, 4, 5]));
    //dbg!(minimum_sum(vec![2, 2, 1]));
    dbg!(minimum_sum(vec![37, 29, 28, 34, 25, 36, 34, 19]));
}

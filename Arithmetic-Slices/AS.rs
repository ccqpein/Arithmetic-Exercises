pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
    if nums.len() < 3 {
        return 0;
    }

    let mut longest = vec![];
    let mut last_long = 0;
    let mut last_diff = None;
    for i in 1..nums.len() {
        //dbg!(last_long);
        //dbg!(nums[i]);
        //dbg!(last_diff);
        match last_diff {
            Some(n) => {
                if nums[i] - nums[i - 1] != n {
                    if last_long >= 3 {
                        longest.push(last_long);
                        last_long = 2;
                        last_diff = Some(nums[i] - nums[i - 1]);
                    } else {
                        last_long = 2;
                        last_diff = Some(nums[i] - nums[i - 1]);
                    }
                } else {
                    last_long += 1;
                }
            }
            None => {
                last_long = 2;
                last_diff = Some(nums[i] - nums[i - 1]);
            }
        }
    }
    if last_long != 0 {
        longest.push(last_long);
    }
    dbg!(&longest);
    longest.iter().map(|i| (1..=i - 2).sum::<i32>()).sum()
}

fn main() {
    assert_eq!(number_of_arithmetic_slices(vec![1, 3, 5, 7, 9]), 6);
    assert_eq!(number_of_arithmetic_slices(vec![7, 7, 7, 7]), 3);
    assert_eq!(number_of_arithmetic_slices(vec![3, -1, -5, -9]), 3);
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 4]), 3);
    assert_eq!(number_of_arithmetic_slices(vec![1]), 0);
    assert_eq!(number_of_arithmetic_slices(vec![1, 2, 3, 5, 7]), 2);
}

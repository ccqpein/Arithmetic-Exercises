pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
    let mut inner_nums = nums;
    inner_nums.sort();

    let mut count = 1;
    for num in inner_nums {
        if num <= 0 {
            continue;
        } else {
            if count != num {
                if count - 1 == num {
                    continue;
                } else {
                    return count as i32;
                }
            }
            count += 1;
        }
    }
    count
}

fn main() {
    println!("{}", first_missing_positive(vec![1, 2, 0]));
    println!("{}", first_missing_positive(vec![3, 4, -1, 1]));
    println!("{}", first_missing_positive(vec![7, 8, 9, 11, 12]));

    println!("{}", first_missing_positive(vec![0, 2, 2, 1, 1]));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<u32> {
    let len = nums.len();
    for i in 0..len {
        for j in i + 1..len {
            if nums[i] + nums[j] == target {
                return vec![i as u32, j as u32];
            }
        }
    }
    return vec![];
}

fn main() {
    let testcase0 = vec![2, 7, 11, 15];
    let testcase1 = vec![0, 4, 3, 0];
    let testcase2 = vec![-3, 4, 3, 90];

    println!("{:?}", two_sum(testcase0, 9));
    println!("{:?}", two_sum(testcase1, 0));
    println!("{:?}", two_sum(testcase2, 0));
}

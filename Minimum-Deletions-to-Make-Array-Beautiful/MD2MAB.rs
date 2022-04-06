pub fn min_deletion(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut ind = 0;
    let mut count = 0;
    while ind < n - 1 {
        if (ind - count) % 2 == 0 {
            if nums[ind] == nums[ind + 1] {
                count += 1;
            }
        }
        ind += 1;
    }
    if (n - count) % 2 != 0 {
        count += 1;
    }
    count as i32
}

fn main() {}

pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
    let len = nums[0].len();

    let nums = nums
        .into_iter()
        .map(|mut x| {
            x.sort();
            x
        })
        .collect::<Vec<_>>();

    (0..len)
        .map(|ind| nums.iter().map(|nn| nn.get(ind).unwrap()).max().unwrap())
        .sum::<i32>()
}

fn main() {}

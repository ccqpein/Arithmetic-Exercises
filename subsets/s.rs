pub fn subsets(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 0 {
        return vec![vec![]];
    }

    let last = nums.pop().unwrap();
    let next = subsets(nums);

    let mut res = next.clone();
    for mut v in next {
        v.push(last);
        res.push(v);
    }
    res
}

fn main() {}

pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
    nums.sort();
    let mut result = 0;
    let mut a = nums.iter();
    while let Some(i) = a.next() {
        result += i;
        a.next();
    }
    result
}

fn main() {}

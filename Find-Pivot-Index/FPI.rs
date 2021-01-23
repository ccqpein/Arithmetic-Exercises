pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let a = make_inner_vec(&nums);
    let len = a.len();
    for i in 0..len {
        if a[i] - nums[i] == a[len - 1] - a[i] {
            return i as i32;
        }
    }

    -1
}

fn make_inner_vec(nums: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    let mut cache = 0;

    for (ind, n) in nums.iter().enumerate() {
        cache += n;
        result[ind] = cache
    }
    result
}

fn main() {
    assert_eq!(pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(pivot_index(vec![1, 2, 3]), -1);
}

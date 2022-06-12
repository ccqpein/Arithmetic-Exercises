pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![1; nums.len()];
    let mut pre;
    for i in 1..nums.len() {
        pre = result[i - 1];
        result[i] = pre * nums[i - 1];
    }

    let mut post = 1;
    for i in (0..=nums.len() - 2).rev() {
        post *= nums[i + 1];
        result[i] *= post;
    }

    return result;
}

fn main() {
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(
        product_except_self(vec![-1, 1, 0, -3, 3]),
        vec![0, 0, 9, 0, 0]
    );
}

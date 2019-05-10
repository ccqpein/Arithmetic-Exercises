struct NumArray {
    ar: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        NumArray { ar: nums }
    }

    fn update(&mut self, i: i32, val: i32) {
        self.ar[i as usize] = val;
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        self.ar[i as usize..=j as usize].iter().sum()
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * obj.update(i, val);
 * let ret_2: i32 = obj.sum_range(i, j);
 */
fn main() {
    let mut a = NumArray::new(vec![1, 3, 5]);
    println!("{:?}", a.sum_range(0, 2));
    a.update(1, 2);
    println!("{:?}", a.sum_range(0, 2));
}

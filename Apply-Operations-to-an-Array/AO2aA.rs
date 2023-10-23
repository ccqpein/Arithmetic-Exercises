fn apply_operations(nums: Vec<i32>) -> Vec<i32> {
    let num_len = nums.len();
    let mut buff = Vec::with_capacity(nums.len() + 1);
    let last = nums.into_iter().fold(-1, |a, b| {
        if a != b {
            if a != 0 {
                buff.push(a)
            };
            b
        } else {
            if a != 0 {
                buff.push(a * 2);
            }
            0
        }
    });
    //dbg!(last);
    buff.push(last);
    //println!("{:?}", &buff);
    //dbg!(num_len);
    //dbg!(buff.len());
    for _ in buff.len()..=num_len {
        buff.push(0);
    }
    buff[1..].into()
}

fn main() {
    println!("{:?}", apply_operations(vec![1, 2, 2, 1, 1, 0]));
    println!("{:?}", apply_operations(vec![0, 1]));
}

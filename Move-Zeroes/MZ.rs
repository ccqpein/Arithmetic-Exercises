fn main() {
    let nums = vec![0,1,0,3,12];
    let mut result:Vec<i32> = Vec::new();
    let mut zeros = 0;

    for i in &nums {
        if *i != 0 {
            result.push(*i);
        }else {
            zeros += 1;
        }
    }

    for _ in 0..zeros {
        result.push(0)
    }

    println!("{:?}", nums);
    println!("{:?}", result);
}


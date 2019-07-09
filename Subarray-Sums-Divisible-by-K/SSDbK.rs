pub fn subarrays_div_by_k(a: Vec<i32>, k: i32) -> i32 {
    let mut cache = vec![0];
    for e in a {
        let add = cache.last().unwrap() + e;
        if add >= 0 {
            cache.push(add % k);
        } else {
            println!("add: {}", add);
            println!(
                "r: {}",
                //(add as f32 - (add as f32 / 10.0).floor() * 10.0) as i32 % k
                ((add % k) + k) % k
            );
            cache.push(((add % k) + k) % k);
        }
    }

    println!("{:?}", cache);

    let mut result = 0;
    for i in 0..k {
        let this = cache.iter().filter(|x| **x == i).count() as i32;
        //println!("{}", this);
        result += this * (this - 1) / 2;
    }

    result
}

fn main() {
    println!("{}", subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5)); //=> 7
    println!("{}", subarrays_div_by_k(vec![-1, 2, 9], 2)); //=> 2
    println!("{}", subarrays_div_by_k(vec![2, -2, 2, -4], 6)); //=> 2
}

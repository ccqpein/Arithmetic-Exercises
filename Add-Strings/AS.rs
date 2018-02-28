use std::collections::HashMap;

fn exponent(exp: i32) -> i32 {
    let mut result = 1;
    for _ in 0..exp {
        result *= 10;
    }
    result
}

fn add_strings(nums1: &str, nums2: &str, dict: &HashMap<char, i32>) -> i32 {
    let length1 = nums1.len() as i32;
    let length2 = nums2.len() as i32;
    let mut sum = 0;
    let mut count = 1;

    for c in nums1.chars() {
        //:= MARK: well, (expression) wont take ownship
        sum += *dict.get(&c).unwrap() * exponent((length1 - count));
        count += 1;
        //println!("{}", &sum);
    }

    count = 1;
    for c in nums2.chars() {
        sum += *dict.get(&c).unwrap() * exponent((length2 - count));
        count += 1;
        //println!("{}", &sum);
    }

    sum
}

fn main() {
    let mut dict = HashMap::new();
    let mut temp_num = 0;

    for c in "0123456789".chars() {
        dict.insert(c, temp_num);
        temp_num += 1;
    }

    println!("{}", add_strings(&"102", &"20", &dict));
}

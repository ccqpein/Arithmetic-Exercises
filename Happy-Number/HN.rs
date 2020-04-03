fn split_num(s: &str) -> Vec<i32> {
    let a = s.chars();
    let result = a
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    result
}

fn is_happy(s: &str) -> bool {
    let num_vec: i32 = split_num(s).iter().sum();
    if num_vec < 1 {
        return false;
    }
    if num_vec == 1 {
        return true;
    }

    return is_happy(&num_vec.to_string());
}

// leetcode change the quiz
use std::collections::HashSet;
fn is_happy_2(n: i32) -> bool {
    let mut ss: HashSet<i32> = HashSet::new();
    inner_func(n, &mut ss)
}

fn inner_func(n: i32, ss: &mut HashSet<i32>) -> bool {
    if ss.contains(&n) {
        return false;
    };
    let num_vec: i32 = split_num(&n.to_string()).iter().map(|x| x * x).sum();

    if num_vec == 1 {
        return true;
    }

    ss.insert(n);

    return inner_func(num_vec, ss);
}

fn main() {
    println!("{}", is_happy("19"));
    println!("{}", is_happy("1"));
    println!("{}", is_happy_2(2));
    println!("{}", is_happy_2(19));
}

use std::collections::HashMap;

pub fn punishment_number(n: i32) -> i32 {
    let mut re = 0;
    let mut cache = HashMap::new();
    for i in 1..=n {
        let a = split_it(i * i);
        if can_sum_to(i, &a, &mut cache) {
            //dbg!(i);
            //dbg!(a);
            re += i * i;
        }
    }
    re
}

fn split_it(mut n: i32) -> Vec<i32> {
    let mut result = vec![];
    loop {
        result.push(n % 10);
        n /= 10;
        if n == 0 {
            result.reverse();
            return result;
        }
    }
}

fn can_sum_to(n: i32, ns: &[i32], cache: &mut HashMap<(i32, Vec<i32>), bool>) -> bool {
    //println!("{n}, {:?}", ns);
    // match cache.get(&(n, ns.to_vec())) {
    //     Some(v) => return *v,
    //     None => (),
    // }
    if n == 0 && ns.len() == 0 {
        return true;
    }
    if ns.len() == 1 && ns[0] == n {
        return true;
    }
    for i in 1..=ns.len() {
        if can_sum_to(n - merge_number(ns, i), ns.get(i..).unwrap(), cache) {
            //cache.insert((n, ns.to_vec()), true);
            return true;
        }
    }
    //cache.insert((n, ns.to_vec()), false);
    false
}

fn merge_number(ns: &[i32], ind: usize) -> i32 {
    let mut result = 0;
    for ii in 0..ind {
        result += ns[ii] * 10_i32.pow((ind - 1 - ii).try_into().unwrap())
    }
    result
}

fn main() {
    // dbg!(merge_number(&vec![1, 2, 3, 4], 1));
    // dbg!(merge_number(&vec![1, 2, 3, 4], 2));
    // dbg!(merge_number(&vec![1, 2, 3, 4], 3));
    // dbg!(merge_number(&vec![1, 2, 3, 4], 4));

    // dbg!(can_sum_to(10, &vec![1, 0, 0]));
    // dbg!(can_sum_to(9, &vec![8, 1]));
    // dbg!(can_sum_to(1, &vec![1]));
    // dbg!(can_sum_to(36, &vec![1, 2, 9, 6]));
    //dbg!(split_it(81));
    //assert_eq!(punishment_number(10), 182);
    //assert_eq!(punishment_number(37), 1478);
    //dbg!(can_sum_to(45, &vec![2, 0, 2, 5]));
    assert_eq!(punishment_number(45), 3503);
    dbg!(punishment_number(1000));
}

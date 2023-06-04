use std::collections::{HashMap, HashSet};

pub fn count_good_bkp(nums: Vec<i32>, k: i32) -> i64 {
    let n = min_len_of_arr(k);
    let mut count = 0;

    for len in n..=nums.len() {
        for i in 0..=nums.len() - len {
            // if helper(&nums[i..i + len], k) {
            //     println!("yea?");
            //     count += 1
            // }
            let set = make_sets(&nums[i..i + len]);
            //println!("{:?}", &set);
            if helper2(set, k) {
                //println!("yea?");
                count += 1
            }
        }
    }

    count
}

pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
    let mut mp = HashMap::<i32, i32>::new();
    let (n, mut j) = (nums.len(), 0);
    let (mut ret, mut count, k) = (0, 0, k as i64);

    for i in 0..n {
        println!("i: {i}");
        while j < n && count < k {
            println!("j: {j}");
            if let Some(cnt) = mp.get(&nums[j]) {
                println!("cnt: {cnt}");
                count += *cnt as i64;
            }
            *mp.entry(nums[j]).or_insert(0) += 1;
            j += 1;
        }
        if count < k {
            break;
        }
        ret += (n - j + 1) as i64;
        *mp.entry(nums[i]).or_insert(0) -= 1;
        if let Some(cnt) = mp.get(&nums[i]) {
            count -= *cnt as i64;
        }
    }

    ret
}

fn make_sets(slice: &[i32]) -> Vec<i32> {
    //dbg!(slice);
    let mut table = HashMap::new();
    slice.iter().for_each(|e| *table.entry(e).or_insert(0) += 1);
    table.iter().map(|(_, v)| *v).collect()
}

fn helper2(set: Vec<i32>, k: i32) -> bool {
    let mut count = 0_i32;
    for v in set {
        if v >= 2 {
            count += v * (v - 1) / 2;
        }
    }
    count >= k
}

fn helper(slice: &[i32], k: i32) -> bool {
    let mut table = HashMap::new();
    slice.iter().for_each(|e| *table.entry(e).or_insert(0) += 1);

    let mut count = 0;
    for (_, v) in table {
        if v >= 2 {
            count += v * (v - 1) / 2;
        }
    }
    count >= k
}

fn min_len_of_arr(k: i32) -> usize {
    let mut n = 2;
    loop {
        if n * (n - 1) >= k * 2 {
            return n as usize;
        }
        n += 1
    }
}

fn main() {
    //dbg!(min_len_of_arr(100));
    //assert_eq!(1, count_good(vec![1, 1, 1, 1, 1], 10));
    assert_eq!(4, count_good(vec![3, 1, 4, 3, 2, 2, 4], 2));
}

// pub fn find_kth_number(n: i32, k: i32) -> i32 {
//     let mut count = k;
//     for l in 1.. {
//         count -= 1;
//         if count == 0 {
//             return l;
//         }

//         let result = inner_rec(l, n, &mut count);
//         if result != 0 {
//             return result;
//         };
//     }
//     0
// }

// fn inner_rec(head: i32, k: i32, count: &mut i32) -> i32 {
//     if head * 10 < n {
//         0
//     }
//     0
// }

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut count = k;
    return test_inner_rec(1 as u64, &(n as u64), &mut count);
}

fn test_inner_rec(head: u64, n: &u64, count: &mut i32) -> i32 {
    if head > *n {
        return 0;
    }
    //print!("{:?} ", head);
    *count -= 1;

    if *count == 0 {
        return head as i32;
    }

    let re = test_inner_rec(head * 10, n, count);
    if re != 0 {
        return re;
    } else {
        if head % 10 == 9 {
            return 0;
        }
        return test_inner_rec(head + 1, n, count);
    }
}

fn main() {
    let testcase0 = 13;
    let k = 2;
    //dbg!(test_inner_rec(1, &testcase0, &mut k));
    //dbg!(find_kth_number(testcase0, k)); // => 10

    let testcase1 = 100;
    let k = 90;
    //dbg!(find_kth_number(testcase1, k)); // => 9

    let testcase1 = 681692778;
    let k = 351251360;
    //dbg!(find_kth_number(testcase1, k)); // => 416126219

    let testcase1 = 957747794;
    let k = 424238336;
    dbg!(find_kth_number(testcase1, k)); // => 416126219
}

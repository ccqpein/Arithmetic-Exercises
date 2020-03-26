pub fn find_kth_number_old(n: i32, k: i32) -> i32 {
    let mut count = k as u64;
    return test_inner_rec(1 as u64, &(n as u64), &mut count) as i32;
}

// this version not fast enough, but it works.
fn test_inner_rec(head: u64, n: &u64, count: &mut u64) -> u64 {
    if head > *n {
        return 0;
    }

    //print!("{:?} ", head);
    *count -= 1;

    if *count == 0 {
        return head;
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

///////////////////////////////////////

pub fn find_kth_number(n: i32, k: i32) -> i32 {
    let mut curr = 1;

    let mut kk = (k - 1) as u64;
    while kk > 0 {
        let steps = cal_steps(n, curr, curr + 1);
        //println!("{}", steps);
        if steps <= kk {
            curr += 1;
            kk -= steps;
        } else {
            curr *= 10;
            kk -= 1;
        }
    }

    return curr as i32;
}

fn cal_steps(n: i32, n1: u64, n2: u64) -> u64 {
    use std::cmp;

    let mut steps = 0;
    let nn = n as u64;
    let (mut n11, mut n22) = (n1, n2);
    while n11 <= nn {
        steps += cmp::min(nn + 1, n22) - n11;
        n11 *= 10;
        n22 *= 10;
    }
    return steps;
}

fn main() {
    let testcase0 = 13;
    let k = 2;
    //dbg!(find_kth_number(testcase0, k)); // => 10

    let testcase1 = 100;
    let k = 90;
    //dbg!(find_kth_number(testcase1, k)); // => 9

    let testcase1 = 681692778;
    let k = 351251360;
    dbg!(find_kth_number(testcase1, k)); // => 416126219

    let testcase1 = 957747794;
    let k = 424238336;
    dbg!(find_kth_number(testcase1, k)); // => 481814499

    let testcase2 = 10000;
    let mut k = 10000;
    dbg!(find_kth_number(testcase2, k)); // => 9999
}

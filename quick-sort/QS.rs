fn quick_sort(l: Vec<u32>) -> Vec<u32> {
    if l.len() == 0 {
        return vec![];
    }

    let key = l[0];
    let mut smaller = vec![];
    let mut larger = vec![];

    for i in &l[1..] {
        if *i <= key {
            smaller.push(*i);
        } else {
            larger.push(*i);
        }
    }

    let mut result = quick_sort(smaller);

    result.push(key);
    result.append(&mut quick_sort(larger));
    result
}

unsafe fn quick_sort_2(mut start_prt: *mut u32, mut end_prt: *mut u32) {
    if start_prt >= end_prt {
        return;
    }

    let (copy_start_prt, copy_end_prt) = (start_prt, end_prt);
    while start_prt != end_prt {
        if *start_prt > *end_prt {
            let temp = *end_prt;
            *end_prt = *start_prt;
            end_prt = end_prt.sub(1);
            *start_prt = *end_prt;
            *end_prt = temp;
        } else {
            start_prt = start_prt.add(1);
        }
    }

    quick_sort_2(copy_start_prt, start_prt.sub(1));
    quick_sort_2(end_prt.add(1), copy_end_prt);
}

fn main() {
    let result = quick_sort(vec![3, 2, 4, 1, 5, 4, 3, 5, 7, 7, 8, 0, 3, 2]);
    println!("{:?}", result);

    let mut testcase0 = vec![3, 7, 8, 5, 2, 1, 9, 5, 4];

    let prt = testcase0.as_mut_ptr();
    unsafe {
        quick_sort_2(prt, prt.add(testcase0.len() - 1));
    }
    println!("{:?}", testcase0);
}
